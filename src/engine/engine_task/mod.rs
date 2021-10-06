use crate::common::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::Arc;

enum PendingCommand {
    CreateSwapchain {
        external_id: usize,
        label: String,
        surface: Arc<crate::wgpu::Surface>,
        width: u32,
        height: u32,
    },
    ResizeSwapchain {
        external_id: usize,
        width: u32,
        height: u32,
    },
    DestroySwapchain {
        external_id: usize,
    },
}

pub struct EngineTask {
    tokio: tokio::runtime::Handle,
    id: TaskId,
    instance: InstanceId,
    devices: Vec<DeviceId>,
    swapchains: HashMap<usize, SwapchainId>,

    pending_commands: Vec<PendingCommand>,
}

impl EngineTask {
    const TASK_NAME: &'static str = "Engine";
    pub fn new(
        id: TaskId,
        tokio: tokio::runtime::Handle,
        requirements: impl Into<Requirements>,
        update_context: &mut UpdateContext,
    ) -> Self {
        let (features, limits) = requirements.into().into();

        let backend = crate::wgpu::BackendBit::VULKAN;
        let instance_descriptor = InstanceDescriptor {
            label: String::from("Engine"),
            backend,
        };
        let instance_handle = Arc::new(crate::wgpu::Instance::new(backend));

        let instance =
            update_context.add_instance(instance_descriptor, Some(instance_handle.clone()));

        let instance = match instance {
            Ok(instance) => instance,
            Err(err) => {
                log::error!(target: "Engine","Failed to initialize Instance: {:#?}",err);
                //return Err(WGpuEngineError::InitializationFailed);
                panic!()
            }
        };

        let devices: Vec<_> = instance_handle
            .enumerate_adapters(backend)
            .map(|adapter| {
                let features = adapter.features() & features;
                let limits = adapter.limits().min(limits.clone());

                let adapter_info = adapter.get_info();

                let descriptor = DeviceDescriptor {
                    label: adapter_info.name,
                    instance,
                    backend,
                    pci_id: adapter_info.vendor,
                    features,
                    limits: limits.clone(),
                };

                let device_descriptor = crate::wgpu::DeviceDescriptor {
                    label: None,
                    features,
                    limits,
                };
                let (device, queue) = tokio
                    .block_on(adapter.request_device(&device_descriptor, None))
                    .unwrap();
                (descriptor, Arc::new((adapter, device, queue)))
            })
            .filter_map(|(device_descriptor, device_handle)| {
                let device_result =
                    update_context.add_device(device_descriptor, Some(device_handle));

                match device_result {
                    Ok(device) => Some(device),
                    Err(err) => {
                        log::error!(target: "Engine","Failed to initialize Device: {:#?}",err);
                        None
                    }
                }
            })
            .collect();

        let swapchains = HashMap::new();
        let pending_commands = Vec::new();

        Self {
            tokio,
            id,
            instance,
            devices,
            swapchains,
            pending_commands,
        }
    }

    pub fn instance(&self) -> &InstanceId {
        &self.instance
    }
    pub fn devices(&self) -> &Vec<DeviceId> {
        &self.devices
    }
    pub fn swapchains(&self) -> impl Iterator<Item = &SwapchainId> {
        self.swapchains.values()
    }

    pub fn create_swapchain(
        &mut self,
        external_id: usize,
        label: String,
        surface: Arc<crate::wgpu::Surface>,
        width: u32,
        height: u32,
    ) {
        self.pending_commands.push(PendingCommand::CreateSwapchain {
            external_id,
            label,
            surface,
            width,
            height,
        });
    }

    pub fn resize_swapchain(&mut self, external_id: usize, width: u32, height: u32) {
        self.pending_commands.push(PendingCommand::ResizeSwapchain {
            external_id,
            width,
            height,
        });
    }

    pub fn remove_swapchain(&mut self, external_id: usize) {
        self.pending_commands
            .push(PendingCommand::DestroySwapchain { external_id });
    }
}

impl TaskTrait for EngineTask {
    fn name(&self) -> String {
        Self::TASK_NAME.to_string()
    }

    fn update(&mut self) {}

    fn update_resources(&mut self, update_context: &mut UpdateContext) {
        let events: Vec<_> = self.pending_commands.drain(..).collect();



        let prepared_swapchains: HashSet<_> = events.into_iter().filter_map(|event| match event {
            PendingCommand::CreateSwapchain {
                external_id,
                label,
                surface,
                width,
                height,
            } => {
                let device = match self.devices.get(0) {
                    Some(device) => *device,
                    None => return None,
                };

                let format = update_context
                    .device_handle_ref(&device)
                    .unwrap()
                    .0
                    .get_swap_chain_preferred_format(&surface)
                    .expect("Incompatible device");

                let usage = crate::wgpu::TextureUsage::RENDER_ATTACHMENT;
                let present_mode = crate::wgpu::PresentMode::Mailbox;

                let descriptor = SwapchainDescriptor {
                    label,
                    device,
                    surface,
                    format,
                    width,
                    height,
                    usage,
                    present_mode,
                };

                match update_context.add_swapchain_descriptor(descriptor) {
                    Ok(id) => {
                        //swapchain_to_prepare.remove(&id);
                        self.swapchains.insert(external_id, id);
                        update_context.push_event(ResourceEvent::SwapchainCreated(id));
                        log::info!(target: "EngineTask","{} created",id);
                        Some(id)
                    }
                    Err(()) => None,
                }
            }
            PendingCommand::ResizeSwapchain {
                external_id,
                width,
                height,
            } => {

                if let Some(id) = self.swapchains.get_mut(&external_id) {
                    update_context.swapchain_descriptor_ref(id)
                    .cloned()
                    .map(|mut descriptor|{
                        log::info!(target: "EngineTask","Resizing swapchain");
                        descriptor.width = width;
                        descriptor.height = height;

                        let result = update_context.update_swapchain_descriptor(id,descriptor);
                        if result {
                            //swapchain_to_prepare.remove(&id);
                            update_context.swapchain_handle_ref(id).map(|handle|handle.present());
                            update_context.push_event(ResourceEvent::SwapchainUpdated(*id));
                            log::info!(target: "EngineTask","{} resized",id);
                            Some(*id)
                        } else {
                            log::error!("Surface {} does not exists", id);
                            None
                        }
                    }).flatten()
                }
                else{None}
            }
            PendingCommand::DestroySwapchain { external_id } => {
                self.swapchains.remove(&external_id).map(|id|{
                    //swapchain_to_prepare.remove(&id);
                    update_context.remove_swapchain(&id).unwrap();
                    update_context.push_event(ResourceEvent::SwapchainDestroyed(id));
                    log::info!(target: "EngineTask","{} destroyed",id);
                    id
                })
            }
        }).collect();

        let current_swapchains: HashSet<SwapchainId> =
            self.swapchains.values().cloned().collect();

        println!("Prepared swapchains: {:#?}",prepared_swapchains);
        println!("Current swapchains: {:#?}",current_swapchains);
        println!("Outdated swapchains: {:#?}",current_swapchains.difference(&prepared_swapchains));

        current_swapchains.difference(&prepared_swapchains).for_each(|id| {
            update_context
                .swapchain_handle_ref(&id)
                .map(|handle|{
                    log::info!(target: "EngineTask","Preparing frame for {}",id);
                    handle.prepare_frame()
                });
        });
    }
    fn command_buffers(&self) -> Vec<CommandBufferId> {
        Vec::new()
    }
}
