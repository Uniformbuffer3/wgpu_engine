use crate::entity_manager::UpdateContext;
use crate::*;
use bytemuck::{Pod, Zeroable};
use inline_spirv::*;
use std::num::NonZeroU32;
use ultraviolet::{Mat4, Vec4};
mod surface_manager;
use std::path::PathBuf;
use surface_manager::*;

const VERTEX_SHADER_CODE: &[u32] = inline_spirv!(
    r#"
#version 450

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 size;
layout(location = 2) in uint index;

layout(push_constant) uniform PushConstants {
    mat4 projection_matrix;
};

layout(location = 0) out vec3 fragment_pos;
layout(location = 1) flat out uint out_index;

void main() {
    switch (gl_VertexIndex) {
        case 0:{
            vec3 vertex = position;

            vec4 projected_vertex = projection_matrix * vec4(vertex,1.0);
            gl_Position = vec4(projected_vertex.xy,0.0,1.0);
            fragment_pos = vec3(0.0,0.0,projected_vertex.z);
            break;
        }
        case 1:{
            vec3 vertex = vec3(position.x,position.y+size.y,position.z);

            vec4 projected_vertex = projection_matrix * vec4(vertex,1.0);
            gl_Position = vec4(projected_vertex.xy,0.0,1.0);
            fragment_pos = vec3(0.0,1.0,projected_vertex.z);
            break;
        }
        case 2:{
            vec3 vertex = vec3(position.x+size.x,position.y,position.z);

            vec4 projected_vertex = projection_matrix * vec4(vertex,1.0);
            gl_Position = vec4(projected_vertex.xy,0.0,1.0);
            fragment_pos = vec3(1.0,0.0,projected_vertex.z);
            break;
        }
        case 3:{
            vec3 vertex = vec3(position.x+size.x,position.y+size.y,position.z);

            vec4 projected_vertex = projection_matrix * vec4(vertex,1.0);
            gl_Position = vec4(projected_vertex.xy,0.0,1.0);
            fragment_pos = vec3(1.0,1.0,projected_vertex.z);
            break;
        }
        default:{
            vec3 vertex = vec3(2.0,2.0,2.0);

            gl_Position = vec4(vertex,1.0);
            fragment_pos = vec3(0.0,0.0,0.0);
        }
    }

    out_index = index;
}
"#,
    vert
);

const FRAGMENT_SHADER_CODE: &[u32] = inline_spirv!(
    r#"
#version 450

#extension GL_EXT_nonuniform_qualifier : require

layout(location = 0) in vec3 fragment_position;
layout(location = 1) nonuniformEXT flat in uint index;  // dynamically non-uniform
layout(location = 0) out vec4 fragment_color;

layout(set = 0, binding = 0) uniform texture2D textures[];
layout(set = 0, binding = 1) uniform sampler samp;

void main() {
    vec4 color = vec4(texture(sampler2D(textures[index], samp), fragment_position.xy));
    if(color.w == 0.0) {discard;}
    else{fragment_color = color;}

    gl_FragDepth = fragment_position.z;
}
"#,
    frag
);

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct PushConstants {
    projection_matrix: Mat4,
}
impl PushConstants {
    pub fn new(target_surface_size: [u32; 2], max_surface_count: u32) -> Self {
        let projection_matrix = Mat4::new(
            Vec4::new(2.0 / target_surface_size[0] as f32, 0.0, 0.0, 0.0),
            Vec4::new(0.0, -2.0 / target_surface_size[1] as f32, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0 / max_surface_count as f32, 0.0),
            Vec4::new(-1.0, 1.0, 0.0, 0.0),
        );
        Self { projection_matrix }
    }
}

enum TaskEvent {
    CreateSurface {
        id: usize,
        label: String,
        source: SurfaceSource,
        position: [u32; 3],
        size: [u32; 2],
    },
}

pub struct RectangleTask {
    id: EntityId,
    pending_events: Vec<TaskEvent>,

    rectangle_manager: RectangleManager,

    target_surface: EntityId,
    target_surface_size: [u32; 2],

    fragment_shader_id: EntityId,
    vertex_shader_id: EntityId,
    sampler_id: EntityId,

    bind_group_layout_id: EntityId,
    pipeline_layout_id: EntityId,
    render_pipeline_id: EntityId,

    bind_group_id: EntityId,
    command_buffer_id: EntityId,

    data_copy_command_buffer_id: EntityId,

    command_buffers_to_execute: Vec<EntityId>,
}

impl RectangleTask {
    const TASK_NAME: &'static str = "RectangleTask";

    pub fn new(
        id: EntityId,
        update_context: &mut UpdateContext,
        target_surface: EntityId,
        target_surface_size: [u32; 2],
    ) -> Self {
        let pending_events = Vec::new();
        let task_name = Self::TASK_NAME.to_string();

        let vertex_shader_descriptor = ShaderModuleDescriptor {
            label: String::from("RectangleTask vertex shader"),
            source: ShaderSource::SpirV(VERTEX_SHADER_CODE.to_vec()),
            flags: wgpu::ShaderFlags::empty(),
        };
        let vertex_shader_id = update_context
            .add_resource_descriptor(vertex_shader_descriptor)
            .unwrap();

        let fragment_shader_descriptor = ShaderModuleDescriptor {
            label: String::from("RectangleTask fragment shader"),
            source: ShaderSource::SpirV(FRAGMENT_SHADER_CODE.to_vec()),
            flags: wgpu::ShaderFlags::empty(),
        };
        let fragment_shader_id = update_context
            .add_resource_descriptor(fragment_shader_descriptor)
            .unwrap();

        let sampler_descriptor = SamplerDescriptor {
            label: Self::TASK_NAME.to_string() + " index buffer",
            anisotropy_clamp: Some(NonZeroU8::new(16).unwrap()),
            min_filter: wgpu::FilterMode::Linear,
            mag_filter: wgpu::FilterMode::Linear,
            ..SamplerDescriptor::default()
        };
        let sampler_id = update_context.add_resource_descriptor(sampler_descriptor).unwrap();

        let bind_group_layout = BindGroupLayoutDescriptor {
            label: Self::TASK_NAME.to_string() + " bind group layout",
            entries: vec![
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: NonZeroU32::new(0),
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        comparison: false,
                        filtering: true,
                    },
                    count: None,
                },
            ],
        };
        let bind_group_layout_id = update_context.add_resource_descriptor(bind_group_layout).unwrap();

        let aligned_size = ((std::mem::size_of::<PushConstants>() + 4 - 1) / 4) * 4;
        let pipeline_layout_descriptor = PipelineLayoutDescriptor {
            label: task_name.clone(),
            bind_group_layouts: vec![bind_group_layout_id],
            push_constant_ranges: vec![wgpu::PushConstantRange {
                stages: wgpu::ShaderStage::VERTEX,
                range: 0..aligned_size as u32,
            }],
        };
        let pipeline_layout_id = update_context
            .add_resource_descriptor(pipeline_layout_descriptor)
            .unwrap();

        let render_pipeline_descriptor = RenderPipelineDescriptor {
            label: task_name,
            layout: Some(pipeline_layout_id),
            vertex: VertexState {
                module: vertex_shader_id,
                entry_point: String::from("main"),
                buffers: vec![VertexBufferLayout {
                    array_stride: std::mem::size_of::<Rectangle>() as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Instance,
                    attributes: wgpu::vertex_attr_array![
                        0 => Float32x3,
                        1 => Float32x2,
                        2 => Uint32,
                    ]
                    .to_vec(),
                }],
            },
            primitive: wgpu::PrimitiveState {
                //front_face: wgpu::FrontFace::Ccw,
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(FragmentState {
                module: fragment_shader_id,
                entry_point: String::from("main"),
                targets: Vec::new(),
            }),
            surface_id: target_surface,
        };
        let render_pipeline_id = update_context
            .add_resource_descriptor(render_pipeline_descriptor)
            .unwrap();

        let bind_group_descriptor = BindGroupDescriptor {
            label: Self::TASK_NAME.to_string() + " bind group",
            entries: vec![
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureViewArray(Vec::new()),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(sampler_id),
                },
            ],
            layout: bind_group_layout_id,
        };
        let bind_group_id = update_context.add_resource_descriptor(bind_group_descriptor).unwrap();

        let command_buffer_descriptor = CommandBufferDescriptor {
            label: Self::TASK_NAME.to_string() + " command buffer",
            commands: vec![Command::RenderPass(target_surface, Vec::new())],
        };
        let command_buffer_id = update_context
            .add_resource_descriptor(command_buffer_descriptor)
            .unwrap();

        let data_copy_command_buffer_descriptor = CommandBufferDescriptor {
            label: Self::TASK_NAME.to_string() + " data copy command buffer",
            commands: vec![],
        };
        let data_copy_command_buffer_id = update_context
            .add_resource_descriptor(data_copy_command_buffer_descriptor)
            .unwrap();

        let command_buffers_to_execute = Vec::new();

        let rectangle_manager = RectangleManager::new(update_context);

        Self {
            id,

            pending_events,
            rectangle_manager,

            target_surface,
            target_surface_size,

            vertex_shader_id,
            fragment_shader_id,
            sampler_id,

            bind_group_layout_id,
            pipeline_layout_id,
            render_pipeline_id,
            bind_group_id,
            command_buffer_id,

            data_copy_command_buffer_id,
            command_buffers_to_execute,
        }
    }

    pub fn create_surface(
        &mut self,
        label: String,
        source: SurfaceSource,
        position: [u32; 3],
        size: [u32; 2],
    ) -> usize {
        let id = self.rectangle_manager.book_id();
        self.pending_events.push(TaskEvent::CreateSurface {
            id,
            label,
            source,
            position,
            size,
        });
        id
    }

    pub fn resize_surface(&mut self) {}

    fn update_bind_group_and_command_buffer(
        &mut self,
        update_context: &mut UpdateContext,
        push_constants: Vec<u8>,
    ) {
        let bind_group_layout = BindGroupLayoutDescriptor {
            label: Self::TASK_NAME.to_string() + " bind group layout",
            entries: vec![
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: NonZeroU32::new(self.rectangle_manager.len() as u32),
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        comparison: false,
                        filtering: true,
                    },
                    count: None,
                },
            ],
        };
        update_context
            .update_resource(&self.bind_group_layout_id, bind_group_layout)
            .unwrap();

        let bind_group_descriptor = BindGroupDescriptor {
            label: Self::TASK_NAME.to_string() + " bind group",
            entries: vec![
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureViewArray(
                        self.rectangle_manager.rectangle_views(),
                    ),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(self.sampler_id),
                },
            ],
            layout: self.bind_group_layout_id,
        };
        update_context
            .update_resource(&self.bind_group_id, bind_group_descriptor)
            .unwrap();

        let command_buffer_descriptor = CommandBufferDescriptor {
            label: Self::TASK_NAME.to_string() + " command buffer",
            commands: vec![Command::RenderPass(
                self.target_surface,
                vec![
                    RenderCommand::SetPipeline {
                        pipeline: self.render_pipeline_id,
                    },
                    RenderCommand::SetPushConstants {
                        stages: wgpu::ShaderStage::VERTEX,
                        offset: 0,
                        data: push_constants,
                    },
                    RenderCommand::SetBindGroup {
                        index: 0,
                        bind_group: self.bind_group_id,
                        offsets: Vec::new(),
                    },
                    RenderCommand::SetVertexBuffer {
                        slot: 0,
                        buffer: *self.rectangle_manager.buffer_id(),
                        slice: Slice::from(..),
                    },
                    RenderCommand::Draw {
                        vertices: 0..4,
                        instances: 0..self.rectangle_manager.len() as u32,
                    },
                ],
            )],
        };
        update_context
            .update_resource(&self.command_buffer_id, command_buffer_descriptor)
            .unwrap();
    }

    fn elaborate_events(&mut self, update_context: &mut UpdateContext) {
        let mut update = false;
        for event in self.pending_events.drain(..) {
            match event {
                TaskEvent::CreateSurface {
                    id,
                    label,
                    source,
                    position,
                    size,
                } => {
                    self.rectangle_manager.create_surface(
                        update_context,
                        label,
                        id,
                        source,
                        position,
                        size,
                    );
                    update = true;
                }
            }
        }

        if update {
            let push_constants = PushConstants::new(self.target_surface_size, 1024);
            self.update_bind_group_and_command_buffer(
                update_context,
                bytemuck::bytes_of(&push_constants).to_vec(),
            );
        }

        let mut command_buffers_to_execute = Vec::new();
        command_buffers_to_execute.push(self.command_buffer_id);

        let commands = self.rectangle_manager.update(update_context);
        if !commands.is_empty() {
            let data_copy_command_buffer_descriptor = CommandBufferDescriptor {
                label: Self::TASK_NAME.to_string() + " data copy command buffer",
                commands,
            };
            update_context
                .update_resource(
                    &self.data_copy_command_buffer_id,
                    data_copy_command_buffer_descriptor,
                )
                .unwrap();
            command_buffers_to_execute.push(self.data_copy_command_buffer_id);
        }

        self.command_buffers_to_execute = command_buffers_to_execute;
    }
}

impl TaskTrait for RectangleTaskTrait {
    fn name(&self) -> String {
        Self::TASK_NAME.to_string()
    }

    fn update_resources(&mut self, update_context: &mut UpdateContext) {
        self.elaborate_events(update_context);
    }
    fn command_buffers(&self) -> Vec<EntityId> {
        self.command_buffers_to_execute.clone()
    }
}

impl AsRef<EntityId> for RectangleTask {
    fn as_ref(&self) -> &EntityId {
        &self.id
    }
}

#[test]
fn rectangle_task() {
    use std::collections::HashMap;
    env_logger::init();
    use crate::WGpuEngine;
    use pal::definitions::*;

    let features = wgpu::Features::EXTERNAL_MEMORY
        | wgpu::Features::PUSH_CONSTANTS
        | wgpu::Features::UNSIZED_BINDING_ARRAY
        | wgpu::Features::SAMPLED_TEXTURE_BINDING_ARRAY
        | wgpu::Features::SAMPLED_TEXTURE_ARRAY_DYNAMIC_INDEXING
        | wgpu::Features::SAMPLED_TEXTURE_ARRAY_NON_UNIFORM_INDEXING;

    let mut limits = wgpu::Limits::default();
    limits.max_push_constant_size = std::mem::size_of::<PushConstants>() as u32;

    let mut wgpu_engine = WGpuEngine::new(features, limits.clone());

    let mut platform = pal::Platform::new(vec![Box::new(wgpu_engine.wgpu_context())]);
    platform.request(vec![Request::from(SurfaceRequest::Create(None))]);

    let surfaces: HashMap<SurfaceId, (EntityId, [u32; 2])> = platform
        .events()
        .into_iter()
        .filter_map(|event| match event {
            pal::Event::Surface(ref surface_event) => {
                let surface_id = surface_event.id;
                match &surface_event.event_type {
                    pal::SurfaceEventType::Added(surface_info) => {
                        if let Surface::WGpu(surface) = &surface_info.surface {
                            let resource_id = wgpu_engine.create_surface(
                                String::from("MainSurface"),
                                surface.clone(),
                                surface_info.size.width,
                                surface_info.size.height,
                            );

                            match resource_id {
                                Ok(resource_id) => Some((
                                    surface_id,
                                    (
                                        resource_id,
                                        [surface_info.size.width, surface_info.size.height],
                                    ),
                                )),
                                Err(_) => None,
                            }
                        } else {
                            panic!("It is not of WGpu type");
                        }
                    }
                    _ => None,
                }
            }
            _ => None,
        })
        .collect();

    let target_surface = *surfaces.values().next().unwrap();

    let mut task = wgpu_engine
        .create_task(
            "RectangleTask".into(),
            features,
            limits,
            move |context, resources| {
                RectangleTask::new(context, resources, target_surface.0, target_surface.1)
            },
        )
        .unwrap();

    task.create_surface(
        String::from("Surface"),
        SurfaceSource::File {
            path: PathBuf::from("/home/fabio/wgpu_engine/src/logo.png"),
        },
        [150, 150, 0],
        [100, 100],
    );
    //task.create_surface_from_file(String::from("/home/fabio/wgpu_engine/src/logo.png"));

    let mut tasks = vec![task];
    'main_loop: loop {
        for event in platform.events() {
            match event {
                pal::Event::Surface(ref surface_event) => {
                    let surface_id = surface_event.id;
                    let resource_id = match surfaces.get(&surface_id) {
                        Some(resource_id) => resource_id.0,
                        None => continue,
                    };
                    match &surface_event.event_type {
                        pal::SurfaceEventType::Added(surface_info) => {
                            if let Surface::WGpu(surface) = &surface_info.surface {
                                wgpu_engine
                                    .create_surface(
                                        String::from("MainSurface"),
                                        surface.clone(),
                                        surface_info.size.width,
                                        surface_info.size.height,
                                    )
                                    .unwrap();
                            } else {
                                panic!("It is not of WGpu type");
                            }
                        }
                        pal::SurfaceEventType::Resized(size) => {
                            wgpu_engine
                                .resize_surface(&resource_id, size.width, size.height)
                                .unwrap();
                        }
                        pal::SurfaceEventType::Removed => {
                            wgpu_engine.remove_surface(resource_id);
                            if wgpu_engine.surface_count() == 0 {
                                break 'main_loop;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        wgpu_engine.dispatch_tasks(&mut tasks);
        std::thread::sleep(std::time::Duration::from_secs(3));
    }
}
