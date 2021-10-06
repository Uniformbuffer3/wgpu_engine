use crate::common::*;
use crate::engine::resource_manager::ResourceManager;
use std::collections::HashMap;

pub struct Batch<'a> {
    resource_manager: &'a mut ResourceManager,
    batches: HashMap<DeviceId, DeviceBatch>,
}
impl<'a> Batch<'a> {
    pub fn new(resource_manager: &'a mut ResourceManager) -> Self {
        let batches = HashMap::new();
        Self {
            resource_manager,
            batches,
        }
    }
    pub fn resource_manager_ref(&self) -> &ResourceManager {
        &self.resource_manager
    }
    pub fn resource_manager_mut(&mut self) -> &mut ResourceManager {
        &mut self.resource_manager
    }
    pub fn add_resource_write(&mut self, resource_write: ResourceWrite) -> bool {
        let device_id = match resource_write {
            ResourceWrite::Buffer(ref write) => {
                self.resource_manager.entity_device_id(write.buffer)
            }
            ResourceWrite::Texture(ref write) => {
                self.resource_manager.entity_device_id(write.texture)
            }
        };

        let device_id = match device_id {
            Some(device_id) => device_id,
            None => return false,
        };
        self.batches
            .entry(device_id)
            .or_insert(DeviceBatch::default())
            .add_resource_write(resource_write);
        true
    }

    pub fn add_resource_writes(&mut self, resource_writes: Vec<ResourceWrite>) -> bool {
        for write in resource_writes {
            if !self.add_resource_write(write) {
                return false;
            }
        }
        true
    }

    pub fn add_command_buffer(&mut self, command_buffer: CommandBufferId) -> bool {
        let swapchains = match self
            .resource_manager
            .command_buffer_descriptor_ref(&command_buffer)
        {
            Some(descriptor) => descriptor.swapchains(),
            None => return false,
        };
        let device_id = match self.resource_manager.entity_device_id(command_buffer) {
            Some(device_id) => device_id,
            None => return false,
        };
        let entry = self
            .batches
            .entry(device_id)
            .or_insert(DeviceBatch::default());
        swapchains
            .into_iter()
            .for_each(|swapchain| entry.add_swapchain(swapchain));
        entry.add_command_buffer(command_buffer);
        true
    }

    pub fn submit(mut self) {
        for (device_id, batch) in self.batches {
            batch.submit(&mut self.resource_manager, &device_id)
        }
    }
}

#[derive(Debug, Default)]
pub struct DeviceBatch {
    resource_writes: Vec<ResourceWrite>,
    swapchains_to_clear: Vec<SwapchainId>,
    command_buffers_to_dispatch: Vec<CommandBufferId>,
}
impl DeviceBatch {
    pub fn add_resource_write(&mut self, resource_write: ResourceWrite) {
        self.resource_writes.push(resource_write);
    }
    pub fn add_resource_writes(&mut self, mut resource_writes: Vec<ResourceWrite>) {
        self.resource_writes.append(&mut resource_writes);
    }
    pub fn add_swapchain(&mut self, swapchain: SwapchainId) {
        self.swapchains_to_clear.push(swapchain);
    }
    pub fn add_command_buffer(&mut self, command_buffer: CommandBufferId) {
        self.command_buffers_to_dispatch.push(command_buffer);
    }
    pub fn add_command_buffers(&mut self, mut command_buffers: Vec<CommandBufferId>) {
        self.command_buffers_to_dispatch
            .append(&mut command_buffers);
    }

    pub fn submit(self, resource_manager: &mut ResourceManager, device_id: &DeviceId) {
        log::info!(target: "Engine","Submitting batch: {:#?}",self);
        let device = match resource_manager.device_handle_ref(device_id) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "Engine","Failed to dispatch Batch: Device {} does not exists, skipping",device_id);
                return;
            }
        };

        let queue = &device.2;
        self.resource_writes
            .into_iter()
            .for_each(|resource_write| resource_write.record(&resource_manager, queue));

        let mut command_buffers = Vec::new();
        self.swapchains_to_clear.iter().for_each(|id| match resource_manager.swapchain_handle_ref(id) {
                Some(swapchain) => {
                    log::error!(target: "Engine","Preparing clear command buffer for {} ",id);
                    let current_frame = swapchain.current_frame();
                    let color_attachments = vec![crate::wgpu::RenderPassColorAttachment {
                        view: &current_frame.as_ref().unwrap().output.view,
                        resolve_target: None,
                        ops: crate::wgpu::Operations {
                            load: crate::wgpu::LoadOp::Clear(crate::wgpu::Color::BLACK),
                            store: false,
                        },
                    }];

                    let render_pass_descriptor = crate::wgpu::RenderPassDescriptor {
                        label: None,
                        color_attachments: &color_attachments,
                        depth_stencil_attachment: None,
                    };
                    let mut encoder = device
                        .1
                        .create_command_encoder(&crate::wgpu::CommandEncoderDescriptor { label: None });
                    {
                        let mut _render_pass = encoder.begin_render_pass(&render_pass_descriptor);
                    }
                    command_buffers.push(encoder.finish());
                }
                _=> {
                    log::error!(target: "Engine","Failed to dispatch Batch: CommandBuffer {} does not exists, skipping",id);
                }
            });
        
        self.command_buffers_to_dispatch.into_iter().for_each(|id|{
            match resource_manager.take_command_buffer(&id){
                Some(command_buffer)=>command_buffers.push(command_buffer),
                None=>{
                    log::error!(target: "Engine","Failed to dispatch Batch: CommandBuffer {} does not exists, skipping",id);
                }
            }
        });

        queue.submit(command_buffers);
        for swapchain_id in &self.swapchains_to_clear {
            if let Some(swapchain) = resource_manager.swapchain_handle_ref(swapchain_id) {
                swapchain.present();
                //swapchain.prepare_frame();
            }
        }
    }
}
