use crate::entity_manager::UpdateContext;
use crate::*;
//use bytemuck::{Pod, Zeroable};
use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct DeviceResources {
    swapchains: Vec<SwapchainId>,

    shader_module: ShaderModuleId,
    render_pipeline: RenderPipelineId,
    command_buffer: CommandBufferId,
}

pub struct TriangleTask {
    devices: HashMap<DeviceId, DeviceResources>,
}

impl TriangleTask {
    const TASK_NAME: &'static str = "TriangleTask";

    pub fn new(_update_context: &mut UpdateContext) -> Self {
        let devices = HashMap::new();

        Self { devices }
    }

    pub fn swapchains(&self) -> Vec<SwapchainId> {
        self.devices
            .values()
            .map(|resources| resources.swapchains.clone())
            .flatten()
            .collect()
    }

    fn init_device_resources(
        update_context: &mut UpdateContext,
        device: DeviceId,
        swapchain: SwapchainId,
    ) -> DeviceResources {
        //let format = update_context.swapchain_descriptor_ref(&swapchain).unwrap().format;
        let swapchains = vec![swapchain];

        let shader_module_descriptor = ShaderModuleDescriptor {
            label: Self::TASK_NAME.to_string(),
            device,
            source: ShaderSource::Wgsl(include_str!("shader.wgsl").to_string()),
            flags: crate::wgpu::ShaderFlags::VALIDATION,
        };
        let shader_module = update_context
            .add_shader_module_descriptor(shader_module_descriptor)
            .unwrap();

        let render_pipeline_descriptor =
            Self::prepare_pipeline(update_context, device, &swapchains, shader_module);
        let render_pipeline = update_context
            .add_render_pipeline_descriptor(render_pipeline_descriptor)
            .unwrap();

        let command_buffer_descriptor =
            Self::prepare_command_buffer(device, &swapchains, render_pipeline);
        let command_buffer = update_context
            .add_command_buffer_descriptor(command_buffer_descriptor)
            .unwrap();

        DeviceResources {
            swapchains,
            shader_module,
            render_pipeline,
            command_buffer,
        }
    }

    fn deinit_device_resources(
        update_context: &mut UpdateContext,
        resources: &mut DeviceResources,
    ) {
        update_context.remove_command_buffer(&resources.command_buffer);
        update_context.remove_render_pipeline(&resources.render_pipeline);
        update_context.remove_shader_module(&resources.shader_module);
        resources.swapchains.iter().for_each(|swapchain| {
            update_context.remove_swapchain(swapchain);
        });
    }

    fn prepare_pipeline(
        update_context: &mut UpdateContext,
        device: DeviceId,
        swapchains: &Vec<SwapchainId>,
        shader_module: ShaderModuleId,
    ) -> RenderPipelineDescriptor {
        let formats: Vec<_> = swapchains
            .into_iter()
            .map(|swapchain| {
                update_context
                    .swapchain_descriptor_ref(&swapchain)
                    .unwrap()
                    .format
            })
            .collect();

        RenderPipelineDescriptor {
            label: Self::TASK_NAME.to_string(),
            device,
            layout: None, //Some(self.pipeline_layout_id),
            vertex: VertexState {
                module: shader_module,
                entry_point: String::from("vs_main"),
                buffers: Vec::new(),
            },
            primitive: crate::wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: crate::wgpu::MultisampleState::default(),
            fragment: Some(FragmentState {
                module: shader_module,
                entry_point: String::from("fs_main"),
                targets: vec![crate::wgpu::ColorTargetState {
                    format: formats[0],
                    blend: None,
                    write_mask: crate::wgpu::ColorWrite::ALL,
                }],
            }),
        }
    }

    fn prepare_command_buffer(
        device: DeviceId,
        swapchains: &Vec<SwapchainId>,
        render_pipeline: RenderPipelineId,
    ) -> CommandBufferDescriptor {
        let commands: Vec<_> = swapchains
            .into_iter()
            .map(|swapchain| Command::RenderPass {
                label: Self::TASK_NAME.to_string(),
                depth_stencil: None,
                color_attachments: vec![RenderPassColorAttachment {
                    view: ColorView::Swapchain(*swapchain),
                    resolve_target: None,
                    ops: crate::wgpu::Operations {
                        load: crate::wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                commands: vec![
                    RenderCommand::SetPipeline {
                        pipeline: render_pipeline,
                    },
                    RenderCommand::Draw {
                        vertices: 0..3,
                        instances: 0..1,
                    },
                ],
            })
            .collect();

        CommandBufferDescriptor {
            label: String::from("TriangleTask"),
            device,
            commands,
        }
    }

    fn update_pipeline_and_command_buffer(
        update_context: &mut UpdateContext,
        device: DeviceId,
        resources: &mut DeviceResources,
    ) {
        let render_pipeline_descriptor = Self::prepare_pipeline(
            update_context,
            device,
            &resources.swapchains,
            resources.shader_module,
        );
        assert!(update_context.update_render_pipeline_descriptor(
            &mut resources.render_pipeline,
            render_pipeline_descriptor
        ));

        let command_buffer_descriptor = Self::prepare_command_buffer(
            device,
            &resources.swapchains,
            resources.render_pipeline,
        );
        assert!(update_context.update_command_buffer_descriptor(
            &mut resources.command_buffer,
            command_buffer_descriptor
        ));
    }
}

impl TaskTrait for TriangleTask {
    fn name(&self) -> String {
        Self::TASK_NAME.to_string()
    }

    fn update_resources(&mut self, update_context: &mut UpdateContext) {
        println!("Events: {:#?}", update_context.events());

        for event in update_context.events().clone() {
            match event {
                ResourceEvent::SwapchainCreated(swapchain) => {
                    let device = update_context.entity_device_id(swapchain).unwrap();
                    match self.devices.entry(device) {
                        Entry::Vacant(vacant) => {
                            let resources =
                                Self::init_device_resources(update_context, device, swapchain);
                            vacant.insert(resources);
                        }
                        Entry::Occupied(mut occupied) => {
                            let resources = occupied.get_mut();
                            resources.swapchains.push(swapchain);
                            Self::update_pipeline_and_command_buffer(
                                update_context,
                                device,
                                resources,
                            );
                        }
                    }
                }
                ResourceEvent::SwapchainDestroyed(swapchain) => {
                    self.devices.retain(|device, resources| {
                        if let Some(index) = resources
                            .swapchains
                            .iter()
                            .position(|current_swapchain| current_swapchain == &swapchain)
                        {
                            resources.swapchains.remove(index);
                            if !resources.swapchains.is_empty() {
                                Self::update_pipeline_and_command_buffer(
                                    update_context,
                                    *device,
                                    resources,
                                );
                                true
                            } else {
                                false
                            }
                        } else {
                            true
                        }
                    });
                }
                _ => (),
            }
        }
    }

    fn command_buffers(&self) -> Vec<CommandBufferId> {
        self.devices
            .values()
            .map(|resources| resources.command_buffer)
            .collect()
    }
}

#[test]
fn triangle_task() {
    env_logger::init();
    quick_run(
        2,
        crate::wgpu::Features::default(),
        crate::wgpu::Limits::default(),
        |_id, _tokio_runtime, update_context| TriangleTask::new(update_context)
    )
/*
    use std::collections::HashSet;
    env_logger::init();
    use crate::WGpuEngine;
    use pal::definitions::*;

    let features = crate::wgpu::Features::default();
    let limits = crate::wgpu::Limits::default();
    //limits.max_push_constant_size = std::mem::size_of::<PushConstants>() as u32;

    let mut wgpu_engine =
        WGpuEngine::new((features, limits)).expect("Failed to initialize the engine: {}");

    let mut platform = pal::Platform::new(vec![Box::new(wgpu_engine.wgpu_context())]);
    platform.request(vec![Request::from(SurfaceRequest::Create(None))]);
    platform.request(vec![Request::from(SurfaceRequest::Create(None))]);

    let _task = wgpu_engine
        .create_task(
            "TriangleTask".into(),
            Requirements::default().into(),
            move ,
        )
        .unwrap();

    let mut surfaces = HashSet::new();

    'main_loop: loop {
        for event in platform.events() {
            match event {
                pal::Event::Surface(ref surface_event) => {
                    let surface_id = surface_event.id;
                    match &surface_event.event_type {
                        pal::SurfaceEventType::Added(surface_info) => {
                            if let Surface::WGpu(surface) = &surface_info.surface {
                                wgpu_engine.create_surface(
                                    surface_id.id() as usize,
                                    String::from("MainSurface"),
                                    surface.clone(),
                                    surface_info.size.width,
                                    surface_info.size.height,
                                );
                                surfaces.insert(surface_id.id() as usize);
                            } else {
                                panic!("It is not of WGpu type");
                            }
                        }
                        pal::SurfaceEventType::Resized(size) => {
                            wgpu_engine.resize_surface(
                                surface_id.id() as usize,
                                size.width,
                                size.height,
                            );
                        }
                        pal::SurfaceEventType::Removed => {
                            let id = surface_id.id() as usize;
                            wgpu_engine.remove_surface(id);
                            surfaces.remove(&id);
                            if surfaces.is_empty() {
                                break 'main_loop;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        wgpu_engine.dispatch_tasks();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    */
}
