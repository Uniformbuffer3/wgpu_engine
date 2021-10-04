use crate::common::*;
use crate::engine::resource_manager::ResourceManager;
use std::borrow::Cow::Borrowed;
use std::ops::Range;
use std::sync::{Arc, MutexGuard};

pub enum ResourceBuilderError {
    MissingDependencies,
}

pub enum ResourceBuilder {
    Instance(InstanceBuilder),
    Device(DeviceBuilder),
    Swapchain(SwapchainBuilder),

    Buffer(BufferBuilder),
    Texture(TextureBuilder),
    TextureView(TextureViewBuilder),
    Sampler(SamplerBuilder),
    ShaderModule(ShaderModuleBuilder),

    BindGroupLayout(BindGroupLayoutBuilder),
    BindGroup(BindGroupBuilder),

    PipelineLayout(PipelineLayoutBuilder),
    RenderPipeline(RenderPipelineBuilder),
    ComputePipeline(ComputePipelineBuilder),
    CommandBuffer(CommandBufferBuilder),
}
impl ResourceBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: EntityId,
        descriptor: &ResourceDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        match descriptor {
            ResourceDescriptor::Instance(descriptor) => {
                let id = InstanceId::new(id);
                match InstanceBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::Instance(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::Device(descriptor) => {
                let id = DeviceId::new(id);
                match DeviceBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::Device(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::Swapchain(descriptor) => {
                let id = SwapchainId::new(id);
                match SwapchainBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::Swapchain(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::Buffer(descriptor) => {
                let id = BufferId::new(id);
                match BufferBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::Buffer(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::Texture(descriptor) => {
                let id = TextureId::new(id);
                match TextureBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::Texture(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::TextureView(descriptor) => {
                //let id = id.try_into().unwrap();
                match TextureViewBuilder::new(resource_manager, descriptor) {
                    Ok(builder) => Ok(Self::TextureView(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::Sampler(descriptor) => {
                let id = SamplerId::new(id);
                match SamplerBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::Sampler(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::ShaderModule(descriptor) => {
                let id = ShaderModuleId::new(id);
                match ShaderModuleBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::ShaderModule(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::BindGroupLayout(descriptor) => {
                let id = BindGroupLayoutId::new(id);
                match BindGroupLayoutBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::BindGroupLayout(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::BindGroup(descriptor) => {
                let id = BindGroupId::new(id);
                match BindGroupBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::BindGroup(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::PipelineLayout(descriptor) => {
                let id = PipelineLayoutId::new(id);
                match PipelineLayoutBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::PipelineLayout(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::RenderPipeline(descriptor) => {
                let id = RenderPipelineId::new(id);
                match RenderPipelineBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::RenderPipeline(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::ComputePipeline(descriptor) => {
                let id = ComputePipelineId::new(id);
                match ComputePipelineBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::ComputePipeline(builder)),
                    Err(err) => Err(err),
                }
            }
            ResourceDescriptor::CommandBuffer(descriptor) => {
                let id = CommandBufferId::new(id);
                match CommandBufferBuilder::new(resource_manager, id, descriptor) {
                    Ok(builder) => Ok(Self::CommandBuffer(builder)),
                    Err(err) => Err(err),
                }
            }
        }
    }
    pub fn build(&self) -> ResourceHandle {
        match self {
            Self::Instance(builder) => ResourceHandle::Instance(builder.build()),
            Self::Device(builder) => ResourceHandle::Device(builder.build()),
            Self::Swapchain(builder) => ResourceHandle::Swapchain(builder.build()),
            Self::Buffer(builder) => ResourceHandle::Buffer(builder.build()),
            Self::Texture(builder) => ResourceHandle::Texture(builder.build()),
            Self::TextureView(builder) => ResourceHandle::TextureView(builder.build()),
            Self::Sampler(builder) => ResourceHandle::Sampler(builder.build()),
            Self::ShaderModule(builder) => ResourceHandle::ShaderModule(builder.build()),
            Self::BindGroupLayout(builder) => ResourceHandle::BindGroupLayout(builder.build()),
            Self::BindGroup(builder) => ResourceHandle::BindGroup(builder.build()),
            Self::PipelineLayout(builder) => ResourceHandle::PipelineLayout(builder.build()),
            Self::RenderPipeline(builder) => ResourceHandle::RenderPipeline(builder.build()),
            Self::ComputePipeline(builder) => ResourceHandle::ComputePipeline(builder.build()),
            Self::CommandBuffer(builder) => ResourceHandle::CommandBuffer(builder.build()),
        }
    }
}

#[derive(Debug)]
pub struct InstanceBuilder {
    pub id: InstanceId,
    pub label: String,
    pub backend: wgpu::BackendBit,
}
impl InstanceBuilder {
    pub fn new(
        _resource_manager: &ResourceManager,
        id: InstanceId,
        descriptor: &InstanceDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let label = descriptor.label.clone();
        let backend = descriptor.backend;
        Ok(Self { id, label, backend })
    }
    pub fn build(&self) -> InstanceHandle {
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(wgpu::Instance::new(self.backend))
    }
}

#[derive(Debug)]
pub struct DeviceBuilder {
    pub id: DeviceId,
    pub label: String,
    pub instance: InstanceHandle,
    pub backend: wgpu::BackendBit,
    pub pci_id: usize,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
}
impl DeviceBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: DeviceId,
        descriptor: &DeviceDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let instance = match resource_manager.instance_handle_ref(&descriptor.instance) {
            Some(handle) => handle.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather Device resources: Instance {} not found",descriptor.instance);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let label = descriptor.label.clone();
        let backend = descriptor.backend;
        let pci_id = descriptor.pci_id;
        let features = descriptor.features;
        let limits = descriptor.limits.clone();

        Ok(Self {
            id,
            label,
            instance,
            backend,
            pci_id,
            features,
            limits,
        })
    }
    pub fn build(&self) -> DeviceHandle {
        let adapter = self
            .instance
            .enumerate_adapters(self.backend)
            .find(|adapter| adapter.get_info().device == self.pci_id)
            .unwrap();

        let descriptor = wgpu::DeviceDescriptor {
            label: Some(self.label.as_str()),
            features: self.features,
            limits: self.limits.clone(),
        };

        let (device, queue) = tokio::runtime::Handle::try_current()
            .unwrap()
            .block_on(adapter.request_device(&descriptor, None))
            .unwrap();
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new((adapter, device, queue))
    }
}

pub struct SwapchainBuilder {
    pub id: SwapchainId,
    pub label: String,
    pub device: DeviceHandle,
    pub surface: Arc<wgpu::Surface>,
    pub width: u32,
    pub height: u32,
}
impl SwapchainBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: SwapchainId,
        descriptor: &SwapchainDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let device = match resource_manager.device_handle_ref(&descriptor.device) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather Swapchain resources: Device {} not found",descriptor.device);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };
        let label = descriptor.label.clone();
        let surface = descriptor.surface.clone();
        let width = descriptor.width;
        let height = descriptor.height;
        Ok(Self {
            id,
            label,
            device,
            surface,
            width,
            height,
        })
    }
    pub fn build(&self) -> SwapchainHandle {
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(
            Swapchain::new(&self.device, self.surface.clone(), self.width, self.height).unwrap(),
        )
    }
}

#[derive(Debug, Clone)]
pub struct BufferBuilder {
    pub id: BufferId,
    pub device: DeviceHandle,
    pub label: String,
    pub size: wgpu::BufferAddress,
    pub usage: wgpu::BufferUsage,
}
impl BufferBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: BufferId,
        descriptor: &BufferDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let device = match resource_manager.device_handle_ref(&descriptor.device) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather Buffer resources: parent Device of {} not found",id);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let label = descriptor.label.clone();
        let size = descriptor.size;
        let usage = descriptor.usage;

        Ok(Self {
            id,
            device,
            label,
            size,
            usage,
        })
    }
    pub fn build(&self) -> BufferHandle {
        let descriptor = wgpu::BufferDescriptor {
            label: Some(self.label.as_str()),
            size: self.size,
            usage: self.usage,
            mapped_at_creation: false,
        };
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(self.device.1.create_buffer(&descriptor))
    }
}

#[derive(Debug, Clone)]
pub struct TextureBuilder {
    pub id: TextureId,
    pub device: DeviceHandle,
    pub label: String,
    pub source: TextureSource,
    pub size: wgpu::Extent3d,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub dimension: wgpu::TextureDimension,
    pub format: wgpu::TextureFormat,
    pub usage: wgpu::TextureUsage,
}
impl TextureBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: TextureId,
        descriptor: &TextureDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let device = match resource_manager.device_handle_ref(&descriptor.device) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather Texture resources: parent Device of {} not found",id);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let label = descriptor.label.clone();
        let source = descriptor.source.clone();
        let size = descriptor.size;
        let mip_level_count = descriptor.mip_level_count;
        let sample_count = descriptor.sample_count;
        let dimension = descriptor.dimension;
        let format = descriptor.format;
        let usage = descriptor.usage;

        Ok(Self {
            id,
            device,
            label,
            source,
            size,
            mip_level_count,
            sample_count,
            dimension,
            format,
            usage,
        })
    }
    pub fn build(&self) -> TextureHandle {
        let descriptor = wgpu::TextureDescriptor {
            label: Some(self.label.as_str()),
            size: self.size,
            mip_level_count: self.mip_level_count,
            sample_count: self.sample_count,
            dimension: self.dimension,
            format: self.format,
            usage: self.usage,
        };
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(self.device.1.create_texture(&descriptor))
    }
}

#[derive(Debug, Clone)]
pub struct TextureViewBuilder {
    pub label: String,
    pub texture: TextureHandle,
    pub format: wgpu::TextureFormat,
    pub dimension: wgpu::TextureViewDimension,
    pub aspect: wgpu::TextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: Option<std::num::NonZeroU32>,
    pub base_array_layer: u32,
    pub array_layer_count: Option<std::num::NonZeroU32>,
}
impl TextureViewBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &TextureViewDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let texture = if let Some(texture) =
            resource_manager.texture_handle_ref(&descriptor.texture)
        {
            texture.clone()
        } else {
            log::error!(target: "EntityManager","Failed to gather TextureView resources: Texture {} not found",descriptor.texture);
            return Err(ResourceBuilderError::MissingDependencies);
        };

        let label = descriptor.label.clone();
        let format = descriptor.format;
        let dimension = descriptor.dimension;
        let aspect = descriptor.aspect;
        let base_mip_level = descriptor.base_mip_level;
        let mip_level_count = descriptor.mip_level_count;
        let base_array_layer = descriptor.base_array_layer;
        let array_layer_count = descriptor.array_layer_count;

        Ok(Self {
            label,
            texture,
            format,
            dimension,
            aspect,
            base_mip_level,
            mip_level_count,
            base_array_layer,
            array_layer_count,
        })
    }

    pub fn build(&self) -> TextureViewHandle {
        let descriptor = wgpu::TextureViewDescriptor {
            label: Some(self.label.as_str()),
            format: Some(self.format),
            dimension: Some(self.dimension),
            aspect: self.aspect,
            base_mip_level: self.base_mip_level,
            mip_level_count: self.mip_level_count,
            base_array_layer: self.base_array_layer,
            array_layer_count: self.array_layer_count,
        };
        Arc::new(self.texture.create_view(&descriptor))
    }
}

#[derive(Debug, Clone)]
pub struct SamplerBuilder {
    pub id: SamplerId,
    pub device: DeviceHandle,
    pub label: String,
    pub address_mode_u: wgpu::AddressMode,
    pub address_mode_v: wgpu::AddressMode,
    pub address_mode_w: wgpu::AddressMode,
    pub mag_filter: wgpu::FilterMode,
    pub min_filter: wgpu::FilterMode,
    pub mipmap_filter: wgpu::FilterMode,
    pub lod_min_clamp: f32,
    pub lod_max_clamp: f32,
    pub compare: Option<wgpu::CompareFunction>,
    pub anisotropy_clamp: Option<std::num::NonZeroU8>,
    pub border_color: Option<wgpu::SamplerBorderColor>,
}
impl SamplerBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: SamplerId,
        descriptor: &SamplerDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let device = match resource_manager.device_handle_ref(&descriptor.device) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather Sampler resources: parent Device of {} not found",id);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };
        let label = descriptor.label.clone();
        let address_mode_u = descriptor.address_mode_u;
        let address_mode_v = descriptor.address_mode_v;
        let address_mode_w = descriptor.address_mode_w;
        let mag_filter = descriptor.mag_filter;
        let min_filter = descriptor.min_filter;
        let mipmap_filter = descriptor.mipmap_filter;
        let lod_min_clamp = descriptor.lod_min_clamp;
        let lod_max_clamp = descriptor.lod_max_clamp;
        let compare = descriptor.compare;
        let anisotropy_clamp = descriptor.anisotropy_clamp;
        let border_color = descriptor.border_color;

        Ok(Self {
            id,
            device,
            label,
            address_mode_u,
            address_mode_v,
            address_mode_w,
            mag_filter,
            min_filter,
            mipmap_filter,
            lod_min_clamp,
            lod_max_clamp,
            compare,
            anisotropy_clamp,
            border_color,
        })
    }
    pub fn build(&self) -> SamplerHandle {
        let descriptor = wgpu::SamplerDescriptor {
            label: Some(self.label.as_str()),
            address_mode_u: self.address_mode_u,
            address_mode_v: self.address_mode_v,
            address_mode_w: self.address_mode_w,
            mag_filter: self.mag_filter,
            min_filter: self.min_filter,
            mipmap_filter: self.mipmap_filter,
            lod_min_clamp: self.lod_min_clamp,
            lod_max_clamp: self.lod_max_clamp,
            compare: self.compare,
            anisotropy_clamp: self.anisotropy_clamp,
            border_color: self.border_color,
        };
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(self.device.1.create_sampler(&descriptor))
    }
}

#[derive(Debug, Clone)]
pub struct ShaderModuleBuilder {
    pub id: ShaderModuleId,
    pub device: DeviceHandle,
    pub label: String,
    pub source: ShaderSource,
    pub flags: wgpu::ShaderFlags,
}
impl ShaderModuleBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: ShaderModuleId,
        descriptor: &ShaderModuleDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let device = match resource_manager.device_handle_ref(&descriptor.device) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather ShaderModule resources: parent Device of {} not found",id);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };
        let label = descriptor.label.clone();
        let source = descriptor.source.clone();
        let flags = descriptor.flags;

        Ok(Self {
            id,
            device,
            label,
            source,
            flags,
        })
    }
    pub fn build(&self) -> ShaderModuleHandle {
        let descriptor = wgpu::ShaderModuleDescriptor {
            label: Some(self.label.as_str()),
            source: match self.source {
                ShaderSource::SpirV(ref spirv) => {
                    wgpu::ShaderSource::SpirV(Borrowed(spirv.as_slice()))
                }
                ShaderSource::Wgsl(ref wgsl) => wgpu::ShaderSource::Wgsl(Borrowed(wgsl.as_str())),
            },
            flags: self.flags,
        };
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(self.device.1.create_shader_module(&descriptor))
    }
}

#[derive(Debug, Clone)]
pub struct BindGroupLayoutBuilder {
    pub id: BindGroupLayoutId,
    pub device: DeviceHandle,
    pub label: String,
    pub entries: Vec<wgpu::BindGroupLayoutEntry>,
}
impl BindGroupLayoutBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: BindGroupLayoutId,
        descriptor: &BindGroupLayoutDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let device = match resource_manager.device_handle_ref(&descriptor.device) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather BindGroupLayout resources: parent Device of {} not found",id);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };
        let label = descriptor.label.clone();
        let entries = descriptor.entries.clone();

        Ok(Self {
            id,
            device,
            label,
            entries,
        })
    }
    pub fn build(&self) -> BindGroupLayoutHandle {
        let descriptor = wgpu::BindGroupLayoutDescriptor {
            label: Some(self.label.as_str()),
            entries: self.entries.as_slice(),
        };
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(self.device.1.create_bind_group_layout(&descriptor))
    }
}

#[derive(Debug, Clone)]
pub struct BufferBindingBuilder {
    pub buffer: BufferHandle,
    pub offset: wgpu::BufferAddress,
    pub size: Option<wgpu::BufferSize>,
}
impl BufferBindingBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &BufferBinding,
    ) -> Result<Self, ResourceBuilderError> {
        let buffer = if let Some(buffer) = resource_manager.buffer_handle_ref(&descriptor.buffer) {
            buffer.clone()
        } else {
            log::error!(target: "EntityManager","Failed to gather BufferBinding resources: Buffer {} not found",descriptor.buffer);
            return Err(ResourceBuilderError::MissingDependencies);
        };

        let offset = descriptor.offset;
        let size = descriptor.size;

        Ok(Self {
            buffer,
            offset,
            size,
        })
    }
    pub fn build(&self) -> wgpu::BufferBinding {
        wgpu::BufferBinding {
            buffer: &self.buffer,
            offset: self.offset,
            size: self.size,
        }
    }
}

#[derive(Debug, Clone)]
pub enum BindingResourceBuilder {
    Buffer(BufferBindingBuilder),
    BufferArray(Vec<BufferBindingBuilder>),
    Sampler(SamplerHandle),
    TextureView(TextureViewHandle),
    TextureViewArray(Vec<TextureViewHandle>),
}
impl BindingResourceBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &BindingResource,
    ) -> Result<Self, ResourceBuilderError> {
        let binding_resource = match descriptor {
            BindingResource::Buffer(buffer_binding) => {
                match BufferBindingBuilder::new(resource_manager, buffer_binding) {
                    Ok(buffer_binding_builder) => Self::Buffer(buffer_binding_builder),
                    Err(_) => {
                        log::error!(target: "EntityManager","Failed to gather BindingResource::Buffer resources: BufferBinding failed to create");
                        return Err(ResourceBuilderError::MissingDependencies);
                    }
                }
            }
            BindingResource::BufferArray(buffer_bindings) => {
                let mut buffer_binding_builders = Vec::with_capacity(buffer_bindings.len());
                for buffer_binding in buffer_bindings {
                    match BufferBindingBuilder::new(resource_manager, buffer_binding) {
                        Ok(buffer_binding_builder) => {
                            buffer_binding_builders.push(buffer_binding_builder)
                        }
                        Err(err) => {
                            log::error!(target: "EntityManager","Failed to gather BindingResource::BufferArray resources: BufferBinding failed to create");
                            return Err(err);
                        }
                    }
                }
                Self::BufferArray(buffer_binding_builders)
            }
            BindingResource::Sampler(sampler) => {
                let sampler = if let Some(sampler) = resource_manager.sampler_handle_ref(sampler) {
                    sampler.clone()
                } else {
                    log::error!(target: "EntityManager","Failed to gather BindingResource::Sampler resources: Sampler {} not found",sampler);
                    return Err(ResourceBuilderError::MissingDependencies);
                };

                Self::Sampler(sampler)
            }
            BindingResource::TextureView(texture_view) => {
                let texture_view = if let Some(texture_view) =
                    resource_manager.texture_view_handle_ref(texture_view)
                {
                    texture_view.clone()
                } else {
                    log::error!(target: "EntityManager","Failed to gather BindingResource::TextureView resources: TextureView {} not found",texture_view);
                    return Err(ResourceBuilderError::MissingDependencies);
                };

                Self::TextureView(texture_view)
            }
            BindingResource::TextureViewArray(texture_views) => {
                let mut arc_texture_views = Vec::with_capacity(texture_views.len());
                for texture_view in texture_views {
                    let texture_view = if let Some(texture_view) =
                        resource_manager.texture_view_handle_ref(texture_view)
                    {
                        texture_view.clone()
                    } else {
                        log::error!(target: "EntityManager","Failed to gather BindingResource::TextureViewArray resources: TextureView {} not found",texture_view);
                        return Err(ResourceBuilderError::MissingDependencies);
                    };

                    arc_texture_views.push(texture_view);
                }
                Self::TextureViewArray(arc_texture_views)
            }
        };

        Ok(binding_resource)
    }

    pub fn build<'a>(
        &'a self,
        support1: &'a mut Vec<wgpu::BufferBinding<'a>>,
        support2: &'a mut Vec<&'a wgpu::TextureView>,
    ) -> wgpu::BindingResource<'a> {
        match self {
            Self::Buffer(buffer_binding) => wgpu::BindingResource::Buffer(buffer_binding.build()),
            Self::BufferArray(buffer_bindings) => {
                buffer_bindings
                    .iter()
                    .for_each(|buffer_binding| support1.push(buffer_binding.build()));
                wgpu::BindingResource::BufferArray(support1.as_slice())
            }
            Self::Sampler(sampler) => wgpu::BindingResource::Sampler(sampler.as_ref()),
            Self::TextureView(texture_view) => {
                wgpu::BindingResource::TextureView(texture_view.as_ref())
            }
            Self::TextureViewArray(texture_views) => {
                texture_views
                    .iter()
                    .for_each(|texture_view| support2.push(texture_view.as_ref()));
                wgpu::BindingResource::TextureViewArray(support2.as_slice())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BindGroupEntryBuilder {
    pub binding: u32,
    pub resource: BindingResourceBuilder,
}
impl BindGroupEntryBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &BindGroupEntry,
    ) -> Result<Self, ResourceBuilderError> {
        let binding = descriptor.binding;
        let resource = match BindingResourceBuilder::new(resource_manager, &descriptor.resource) {
            Ok(resource) => resource,
            Err(_) => {
                log::error!(target: "EntityManager","Failed to gather BindGroupEntry resources: BindingResource failed to create");
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        Ok(Self { binding, resource })
    }
    pub fn build<'a>(
        &'a self,
        support1: &'a mut Vec<wgpu::BufferBinding<'a>>,
        support2: &'a mut Vec<&'a wgpu::TextureView>,
    ) -> wgpu::BindGroupEntry<'a> {
        let descriptor = wgpu::BindGroupEntry {
            binding: self.binding,
            resource: self.resource.build(support1, support2),
        };
        descriptor
    }
}

#[derive(Debug, Clone)]
pub struct BindGroupBuilder {
    pub id: BindGroupId,
    pub device: DeviceHandle,
    pub label: String,
    pub layout: BindGroupLayoutHandle,
    pub entries: Vec<BindGroupEntryBuilder>,
}
impl BindGroupBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: BindGroupId,
        descriptor: &BindGroupDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let device = match resource_manager.device_handle_ref(&descriptor.device) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather BindGroup resources: parent Device of {} not found",id);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };
        let layout = if let Some(bind_group_layout) =
            resource_manager.bind_group_layout_handle_ref(&descriptor.layout)
        {
            bind_group_layout.clone()
        } else {
            log::error!(target: "EntityManager","Failed to gather BindGroup resources: BindGroupLayout {} not found",descriptor.layout);
            return Err(ResourceBuilderError::MissingDependencies);
        };
        let label = descriptor.label.clone();
        let mut entries = Vec::with_capacity(descriptor.entries.len());
        for entry in &descriptor.entries {
            let bind_group_entry = match BindGroupEntryBuilder::new(resource_manager, entry) {
                Ok(bind_group_entry) => bind_group_entry,
                Err(err) => {
                    log::error!(target: "EntityManager","Failed to gather BindGroup resources: BindGroupEntry failed to create");
                    return Err(err);
                }
            };
            entries.push(bind_group_entry);
        }

        Ok(Self {
            id,
            device,
            label,
            layout,
            entries,
        })
    }
    pub fn build(&self) -> BindGroupHandle {
        let mut supports1: Vec<Vec<wgpu::BufferBinding>> = Vec::new();
        let mut supports2: Vec<Vec<&wgpu::TextureView>> = Vec::new();
        self.entries.iter().for_each(|_| {
            supports1.push(Vec::new());
            supports2.push(Vec::new());
        });

        let mut entries = Vec::new();
        supports1
            .iter_mut()
            .zip(supports2.iter_mut())
            .enumerate()
            .for_each(|(index, (support1, support2))| {
                let bind_group_entity = self.entries.get(index).unwrap();
                entries.push(bind_group_entity.build(support1, support2));
            });

        let descriptor = wgpu::BindGroupDescriptor {
            label: Some(self.label.as_str()),
            layout: self.layout.as_ref(),
            entries: entries.as_slice(),
        };
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(self.device.1.create_bind_group(&descriptor))
    }
}

#[derive(Debug, Clone)]
pub struct PipelineLayoutBuilder {
    pub id: PipelineLayoutId,
    pub device: DeviceHandle,
    pub label: String,
    pub bind_group_layouts: Vec<BindGroupLayoutHandle>,
    pub push_constant_ranges: Vec<wgpu::PushConstantRange>,
}
impl PipelineLayoutBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: PipelineLayoutId,
        descriptor: &PipelineLayoutDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let device = match resource_manager.device_handle_ref(&descriptor.device) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather BindGroup resources: parent Device of {} not found",id);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };
        let mut bind_group_layouts: Vec<BindGroupLayoutHandle> =
            Vec::with_capacity(descriptor.bind_group_layouts.len());
        for id in &descriptor.bind_group_layouts {
            if let Some(bind_group_layout) = resource_manager.bind_group_layout_handle_ref(id) {
                bind_group_layouts.push(bind_group_layout.clone());
            } else {
                log::error!(target: "EntityManager","Failed to gather PipelineLayout resources: BindGroupLayout {} not found",id);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        }

        let label = descriptor.label.clone();
        let push_constant_ranges = descriptor.push_constant_ranges.clone();

        Ok(Self {
            id,
            device,
            label,
            bind_group_layouts,
            push_constant_ranges,
        })
    }
    pub fn build(&self) -> PipelineLayoutHandle {
        let bind_group_layouts: Vec<&wgpu::BindGroupLayout> = self
            .bind_group_layouts
            .iter()
            .map(|bind_group_layout| bind_group_layout.as_ref())
            .collect();
        let descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some(self.label.as_str()),
            bind_group_layouts: bind_group_layouts.as_slice(),
            push_constant_ranges: self.push_constant_ranges.as_slice(),
        };
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(self.device.1.create_pipeline_layout(&descriptor))
    }
}

#[derive(Debug, Clone)]
pub struct VertexBufferLayoutBuilder {
    pub array_stride: wgpu::BufferAddress,
    pub step_mode: wgpu::InputStepMode,
    pub attributes: Vec<wgpu::VertexAttribute>,
}
impl VertexBufferLayoutBuilder {
    pub fn new(descriptor: &VertexBufferLayout) -> Self {
        let array_stride = descriptor.array_stride;
        let step_mode = descriptor.step_mode;
        let attributes = descriptor.attributes.clone();

        Self {
            array_stride,
            step_mode,
            attributes,
        }
    }
    pub fn build(&self) -> wgpu::VertexBufferLayout {
        wgpu::VertexBufferLayout {
            array_stride: self.array_stride,
            step_mode: self.step_mode,
            attributes: self.attributes.as_slice(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VertexStateBuilder {
    pub module: ShaderModuleHandle,
    pub entry_point: String,
    pub buffers: Vec<VertexBufferLayoutBuilder>,
}
impl VertexStateBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &VertexState,
    ) -> Result<Self, ResourceBuilderError> {
        let module = if let Some(module) =
            resource_manager.shader_module_handle_ref(&descriptor.module)
        {
            module.clone()
        } else {
            log::error!(target: "EntityManager","Failed to gather VertexState resources: ShaderModule {} not found",descriptor.module);
            return Err(ResourceBuilderError::MissingDependencies);
        };

        let entry_point = descriptor.entry_point.clone();
        let mut buffers = Vec::new();
        for vertex_buffer_layout in &descriptor.buffers {
            buffers.push(VertexBufferLayoutBuilder::new(vertex_buffer_layout));
        }

        Ok(Self {
            module,
            entry_point,
            buffers,
        })
    }
    pub fn build<'a>(
        &'a self,
        support: &'a mut Vec<wgpu::VertexBufferLayout<'a>>,
    ) -> wgpu::VertexState<'a> {
        self.buffers
            .iter()
            .for_each(|vertex_buffer_layout| support.push(vertex_buffer_layout.build()));

        wgpu::VertexState {
            module: self.module.as_ref(),
            entry_point: self.entry_point.as_str(),
            buffers: support.as_slice(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FragmentStateBuilder {
    pub module: ShaderModuleHandle,
    pub entry_point: String,
    pub targets: Vec<wgpu::ColorTargetState>,
}
impl FragmentStateBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &FragmentState,
    ) -> Result<Self, ResourceBuilderError> {
        let module = if let Some(module) =
            resource_manager.shader_module_handle_ref(&descriptor.module)
        {
            module.clone()
        } else {
            log::error!(target: "EntityManager","Failed to gather FragmentState resources: ShaderModule {} not found",descriptor.module);
            return Err(ResourceBuilderError::MissingDependencies);
        };

        let entry_point = descriptor.entry_point.clone();
        let targets = descriptor.targets.clone();

        Ok(Self {
            module,
            entry_point,
            targets,
        })
    }
    pub fn build(&self) -> wgpu::FragmentState {
        wgpu::FragmentState {
            module: self.module.as_ref(),
            entry_point: self.entry_point.as_str(),
            targets: self.targets.as_slice(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderPipelineBuilder {
    pub id: RenderPipelineId,
    pub device: DeviceHandle,
    pub label: String,
    pub layout: Option<PipelineLayoutHandle>,
    pub vertex: VertexStateBuilder,
    pub primitive: wgpu::PrimitiveState,
    pub depth_stencil: Option<wgpu::DepthStencilState>,
    pub multisample: wgpu::MultisampleState,
    pub fragment: Option<FragmentStateBuilder>,
}

impl RenderPipelineBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: RenderPipelineId,
        descriptor: &RenderPipelineDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let device = match resource_manager.device_handle_ref(&descriptor.device) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather RenderPipeline resources: parent Device of {} not found",id);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };
        let layout = match &descriptor.layout {
            Some(id) => match resource_manager.pipeline_layout_handle_ref(id) {
                Some(pipeline_layout) => Some(pipeline_layout.clone()),
                None => {
                    log::error!(target: "EntityManager","Failed to gather RenderPipeline resources: PipelineLayout {} not found",id);
                    return Err(ResourceBuilderError::MissingDependencies);
                }
            },
            None => None,
        };

        let vertex = match VertexStateBuilder::new(resource_manager, &descriptor.vertex) {
            Ok(vertex) => vertex,
            Err(_) => {
                log::error!(target: "EntityManager","Failed to gather RenderPipeline resources: VertexState failed to build");
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let depth_stencil = match &descriptor.depth_stencil {
            Some(depth_stencil_state) => {
                let depth_stencil = match resource_manager
                    .texture_view_descriptor_ref(&depth_stencil_state.id)
                {
                    Some(depth_stencil) => depth_stencil,
                    _ => {
                        log::error!(target: "EntityManager","Failed to gather RenderPipeline resources: DepthStencil {} not found",depth_stencil_state.id);
                        return Err(ResourceBuilderError::MissingDependencies);
                    }
                };

                Some(wgpu::DepthStencilState {
                    format: depth_stencil.format,
                    depth_write_enabled: depth_stencil_state.depth_write_enabled,
                    depth_compare: depth_stencil_state.depth_compare,
                    stencil: depth_stencil_state.stencil.clone(),
                    bias: depth_stencil_state.bias,
                })
            }
            None => None,
        };
        /*
                        Some(wgpu::DepthStencilState {
                            format: depth_stencil.format,
                            depth_write_enabled: false,
                            depth_compare: wgpu::CompareFunction::LessEqual,
                            stencil: wgpu::StencilState::default(),
                            bias: wgpu::DepthBiasState::default(),
                        })
        */

        /*
        let mut surface_color_target: wgpu::ColorTargetState = swapchain.format().into();
        surface_color_target.blend = Some(wgpu::BlendState {
            color: wgpu::BlendComponent::OVER,
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::Zero,
                operation: wgpu::BlendOperation::Add,
            },
        });*/

        let fragment = if let Some(fragment_state) = &descriptor.fragment {
            match FragmentStateBuilder::new(resource_manager, fragment_state) {
                Ok(fragment_state_builder) => Some(fragment_state_builder),
                Err(err) => {
                    log::error!(target: "EntityManager","Failed to gather RenderPipeline resources: FragmentState failed to build");
                    return Err(err);
                }
            }
        } else {
            None
        };

        let label = descriptor.label.clone();

        let primitive = descriptor.primitive;
        let multisample = descriptor.multisample;

        Ok(Self {
            id,
            device,
            label,
            layout,
            vertex,
            primitive,
            depth_stencil,
            multisample,
            fragment,
        })
    }
    pub fn build(&self) -> RenderPipelineHandle {
        let mut support = Vec::new();
        let descriptor = wgpu::RenderPipelineDescriptor {
            label: Some(self.label.as_str()),
            layout: self
                .layout
                .as_ref()
                .map(|pipeline_layout| pipeline_layout.as_ref()),
            vertex: self.vertex.build(&mut support),
            depth_stencil: self.depth_stencil.clone(),
            primitive: self.primitive,
            multisample: self.multisample,
            fragment: self
                .fragment
                .as_ref()
                .map(|fragment_state| fragment_state.build()),
        };

        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(self.device.1.create_render_pipeline(&descriptor))
    }
}

#[derive(Debug, Clone)]
pub struct ComputePipelineBuilder {
    pub id: ComputePipelineId,
    pub device: DeviceHandle,
    pub label: String,
    pub layout: Option<PipelineLayoutHandle>,
    pub module: ShaderModuleHandle,
    pub entry_point: String,
}

impl ComputePipelineBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: ComputePipelineId,
        descriptor: &ComputePipelineDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let device = match resource_manager.device_handle_ref(&descriptor.device) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather ComputePipeline resources: parent Device of {} not found",id);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };
        let layout = match &descriptor.layout {
            Some(id) => match resource_manager.pipeline_layout_handle_ref(id) {
                Some(pipeline_layout) => Some(pipeline_layout.clone()),
                None => {
                    log::error!(target: "EntityManager","Failed to gather ComputePipeline resources: PipelineLayout {} not found",id);
                    return Err(ResourceBuilderError::MissingDependencies);
                }
            },
            None => None,
        };

        let module = match resource_manager.shader_module_handle_ref(&descriptor.module) {
            Some(module) => module.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather ComputePipeline resources: ShaderModule {} not found",descriptor.module);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let label = descriptor.label.clone();
        let entry_point = descriptor.entry_point.clone();

        Ok(Self {
            id,
            device,
            label,
            layout,
            module,
            entry_point,
        })
    }
    pub fn build(&self) -> ComputePipelineHandle {
        let descriptor = wgpu::ComputePipelineDescriptor {
            label: Some(self.label.as_str()),
            layout: self
                .layout
                .as_ref()
                .map(|pipeline_layout| pipeline_layout.as_ref()),
            module: self.module.as_ref(),
            entry_point: self.entry_point.as_ref(),
        };
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(self.device.1.create_compute_pipeline(&descriptor))
    }
}

#[derive(Debug, Clone)]
pub enum ComputeCommandBuilder {}
impl ComputeCommandBuilder {
    pub fn new(
        _resource_manager: &ResourceManager,
        _descriptor: &ComputeCommand,
    ) -> Result<Self, ResourceBuilderError> {
        panic!()
    }
    pub fn build<'a>(&'a self, _encoder: &mut wgpu::ComputePass<'a>) -> bool {
        panic!()
    }
}

#[derive(Debug, Clone)]
pub enum RenderCommandBuilder {
    SetPipeline {
        pipeline: RenderPipelineHandle,
    },
    SetPushConstants {
        stages: wgpu::ShaderStage,
        offset: u32,
        data: Vec<u8>,
    },
    SetBindGroup {
        index: u32,
        bind_group: BindGroupHandle,
        offsets: Vec<wgpu::DynamicOffset>,
    },
    SetVertexBuffer {
        slot: u32,
        buffer: BufferHandle,
        slice: Slice<wgpu::BufferAddress>,
    },
    SetIndexBuffer {
        index_format: wgpu::IndexFormat,
        buffer: BufferHandle,
        slice: Slice<wgpu::BufferAddress>,
    },
    Draw {
        vertices: Range<u32>,
        instances: Range<u32>,
    },
    DrawIndexed {
        indices: Range<u32>,
        base_vertex: i32,
        instances: Range<u32>,
    },
}
impl RenderCommandBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &RenderCommand,
    ) -> Result<Self, ResourceBuilderError> {
        Ok(match descriptor {
            RenderCommand::SetPipeline { pipeline } => {
                let pipeline = match resource_manager.render_pipeline_handle_ref(pipeline) {
                    Some(pipeline) => pipeline.clone(),
                    None => {
                        log::error!(target: "EntityManager","Failed to gather RenderCommand::SetPipeline resources: Pipeline {} not found",pipeline);
                        return Err(ResourceBuilderError::MissingDependencies);
                    }
                };
                Self::SetPipeline { pipeline }
            }
            RenderCommand::SetPushConstants {
                stages,
                offset,
                data,
            } => {
                let stages = *stages;
                let offset = *offset;
                let data = data.clone();
                Self::SetPushConstants {
                    stages,
                    offset,
                    data,
                }
            }
            RenderCommand::SetBindGroup {
                index,
                bind_group,
                offsets,
            } => {
                let bind_group = match resource_manager.bind_group_handle_ref(bind_group) {
                    Some(bind_group) => bind_group.clone(),
                    None => {
                        log::error!(target: "EntityManager","Failed to gather RenderCommand::SetBindGroup resources: BindGroup {} not found",bind_group);
                        return Err(ResourceBuilderError::MissingDependencies);
                    }
                };
                let index = *index;
                let offsets = offsets.clone();
                Self::SetBindGroup {
                    index,
                    bind_group,
                    offsets,
                }
            }
            RenderCommand::SetVertexBuffer {
                slot,
                buffer,
                slice,
            } => {
                let buffer = match resource_manager.buffer_handle_ref(buffer) {
                    Some(buffer) => buffer.clone(),
                    None => {
                        log::error!(target: "EntityManager","Failed to gather RenderCommand::SetVertexBuffer resources: Buffer {} not found",buffer);
                        return Err(ResourceBuilderError::MissingDependencies);
                    }
                };
                let slot = *slot;
                let slice = slice.clone();
                Self::SetVertexBuffer {
                    slot,
                    buffer,
                    slice,
                }
            }
            RenderCommand::SetIndexBuffer {
                index_format,
                buffer,
                slice,
            } => {
                let buffer = match resource_manager.buffer_handle_ref(buffer) {
                    Some(buffer) => buffer.clone(),
                    None => {
                        log::error!(target: "EntityManager","Failed to gather RenderCommand::SetIndexBuffer resources: Buffer {} not found",buffer);
                        return Err(ResourceBuilderError::MissingDependencies);
                    }
                };
                let index_format = *index_format;
                let slice = slice.clone();
                Self::SetIndexBuffer {
                    index_format,
                    buffer,
                    slice,
                }
            }
            RenderCommand::Draw {
                vertices,
                instances,
            } => {
                let vertices = vertices.clone();
                let instances = instances.clone();
                Self::Draw {
                    vertices,
                    instances,
                }
            }
            RenderCommand::DrawIndexed {
                indices,
                base_vertex,
                instances,
            } => {
                let indices = indices.clone();
                let base_vertex = *base_vertex;
                let instances = instances.clone();
                Self::DrawIndexed {
                    indices,
                    base_vertex,
                    instances,
                }
            }
        })
    }
    pub fn build<'a>(&'a self, encoder: &mut wgpu::RenderPass<'a>) -> bool {
        match self {
            Self::SetPipeline { pipeline } => encoder.set_pipeline(pipeline),
            Self::SetPushConstants {
                stages,
                offset,
                data,
            } => encoder.set_push_constants(*stages, *offset, data.as_slice()),
            Self::SetBindGroup {
                index,
                bind_group,
                offsets,
            } => encoder.set_bind_group(*index, bind_group, offsets),
            Self::SetVertexBuffer {
                slot,
                buffer,
                slice,
            } => encoder.set_vertex_buffer(*slot, buffer.slice(slice.clone())),
            Self::SetIndexBuffer {
                index_format,
                buffer,
                slice,
            } => encoder.set_index_buffer(buffer.slice(slice.clone()), *index_format),
            Self::Draw {
                vertices,
                instances,
            } => encoder.draw(vertices.clone(), instances.clone()),
            Self::DrawIndexed {
                indices,
                base_vertex,
                instances,
            } => encoder.draw_indexed(indices.clone(), *base_vertex, instances.clone()),
        }
        true
    }
}

#[derive(Debug, Clone)]
pub struct TextureToBufferCopyBuilder {
    pub src_texture: TextureHandle,
    pub src_mip_level: u32,
    pub src_origin: wgpu::Origin3d,
    pub dst_buffer: BufferHandle,
    pub dst_layout: wgpu::ImageDataLayout,
    pub copy_size: wgpu::Extent3d,
}
impl TextureToBufferCopyBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &TextureToBufferCopy,
    ) -> Result<Self, ResourceBuilderError> {
        let src_texture = match resource_manager.texture_handle_ref(&descriptor.src_texture) {
            Some(texture) => texture.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather TextureToBufferCopy resources: Texture source {} not found",descriptor.src_texture);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let dst_buffer = match resource_manager.buffer_handle_ref(&descriptor.dst_buffer) {
            Some(buffer) => buffer.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather TextureToBufferCopy resources: Buffer destination {} not found",descriptor.dst_buffer);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let src_mip_level = descriptor.src_mip_level;
        let src_origin = descriptor.src_origin;
        let dst_layout = descriptor.dst_layout;
        let copy_size = descriptor.copy_size;

        Ok(Self {
            src_texture,
            src_mip_level,
            src_origin,
            dst_buffer,
            dst_layout,
            copy_size,
        })
    }
    pub fn build(&self, encoder: &mut wgpu::CommandEncoder) -> bool {
        let wgpu_src = wgpu::ImageCopyTexture {
            texture: self.src_texture.as_ref(),
            mip_level: self.src_mip_level,
            origin: self.src_origin,
        };
        let wgpu_dst = wgpu::ImageCopyBuffer {
            buffer: self.dst_buffer.as_ref(),
            layout: self.dst_layout,
        };
        encoder.copy_texture_to_buffer(wgpu_src, wgpu_dst, self.copy_size);
        true
    }
}

#[derive(Debug, Clone)]
pub struct TextureToTextureCopyBuilder {
    pub src_texture: TextureHandle,
    pub src_mip_level: u32,
    pub src_origin: wgpu::Origin3d,
    pub dst_texture: TextureHandle,
    pub dst_mip_level: u32,
    pub dst_origin: wgpu::Origin3d,
    pub copy_size: wgpu::Extent3d,
}
impl TextureToTextureCopyBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &TextureToTextureCopy,
    ) -> Result<Self, ResourceBuilderError> {
        let src_texture = match resource_manager.texture_handle_ref(&descriptor.src_texture) {
            Some(texture) => texture.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather TextureToTextureCopy resources: Texture source {} not found",descriptor.src_texture);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let dst_texture = match resource_manager.texture_handle_ref(&descriptor.dst_texture) {
            Some(texture) => texture.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather TextureToTextureCopy resources: Texture destination {} not found",descriptor.dst_texture);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let src_mip_level = descriptor.src_mip_level;
        let src_origin = descriptor.src_origin;
        let dst_mip_level = descriptor.dst_mip_level;
        let dst_origin = descriptor.dst_origin;
        let copy_size = descriptor.copy_size;

        Ok(Self {
            src_texture,
            src_mip_level,
            src_origin,
            dst_texture,
            dst_mip_level,
            dst_origin,
            copy_size,
        })
    }
    pub fn build(&self, encoder: &mut wgpu::CommandEncoder) -> bool {
        let wgpu_src = wgpu::ImageCopyTexture {
            texture: self.src_texture.as_ref(),
            mip_level: self.src_mip_level,
            origin: self.src_origin,
        };
        let wgpu_dst = wgpu::ImageCopyTexture {
            texture: self.dst_texture.as_ref(),
            mip_level: self.dst_mip_level,
            origin: self.dst_origin,
        };
        encoder.copy_texture_to_texture(wgpu_src, wgpu_dst, self.copy_size);
        true
    }
}

#[derive(Debug, Clone)]
pub struct BufferToTextureCopyBuilder {
    pub src_buffer: BufferHandle,
    pub src_layout: wgpu::ImageDataLayout,
    pub dst_texture: TextureHandle,
    pub dst_mip_level: u32,
    pub dst_origin: wgpu::Origin3d,
    pub copy_size: wgpu::Extent3d,
}
impl BufferToTextureCopyBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &BufferToTextureCopy,
    ) -> Result<Self, ResourceBuilderError> {
        let src_buffer = match resource_manager.buffer_handle_ref(&descriptor.src_buffer) {
            Some(buffer) => buffer.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather BufferToTextureCopy resources: Buffer source {} not found",descriptor.src_buffer);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let dst_texture = match resource_manager.texture_handle_ref(&descriptor.dst_texture) {
            Some(texture) => texture.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather BufferToTextureCopy resources: Texture destination {} not found",descriptor.dst_texture);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let src_layout = descriptor.src_layout;
        let dst_mip_level = descriptor.dst_mip_level;
        let dst_origin = descriptor.dst_origin;
        let copy_size = descriptor.copy_size;

        Ok(Self {
            src_buffer,
            src_layout,
            dst_texture,
            dst_mip_level,
            dst_origin,
            copy_size,
        })
    }
    pub fn build(&self, encoder: &mut wgpu::CommandEncoder) -> bool {
        let wgpu_src = wgpu::ImageCopyBuffer {
            buffer: self.src_buffer.as_ref(),
            layout: self.src_layout,
        };
        let wgpu_dst = wgpu::ImageCopyTexture {
            texture: self.dst_texture.as_ref(),
            mip_level: self.dst_mip_level,
            origin: self.dst_origin,
        };
        encoder.copy_buffer_to_texture(wgpu_src, wgpu_dst, self.copy_size);
        true
    }
}

#[derive(Debug, Clone)]
pub struct BufferToBufferCopyBuilder {
    pub src_buffer: BufferHandle,
    pub src_offset: wgpu::BufferAddress,
    pub dst_buffer: BufferHandle,
    pub dst_offset: wgpu::BufferAddress,
    pub size: wgpu::BufferAddress,
}
impl BufferToBufferCopyBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &BufferToBufferCopy,
    ) -> Result<Self, ResourceBuilderError> {
        let src_buffer = match resource_manager.buffer_handle_ref(&descriptor.src_buffer) {
            Some(buffer) => buffer.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather BufferToBufferCopy resources: Buffer source {} not found",descriptor.src_buffer);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let dst_buffer = match resource_manager.buffer_handle_ref(&descriptor.dst_buffer) {
            Some(buffer) => buffer.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather BufferToBufferCopy resources: Buffer destination {} not found",descriptor.dst_buffer);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };

        let src_offset = descriptor.src_offset;
        let dst_offset = descriptor.dst_offset;
        let size = descriptor.size;

        Ok(Self {
            src_buffer,
            src_offset,
            dst_buffer,
            dst_offset,
            size,
        })
    }
    pub fn build(&self, encoder: &mut wgpu::CommandEncoder) -> bool {
        encoder.copy_buffer_to_buffer(
            self.src_buffer.as_ref(),
            self.src_offset,
            self.dst_buffer.as_ref(),
            self.dst_offset,
            self.size,
        );
        true
    }
}

#[derive(Debug, Clone)]
pub enum ColorViewBuilder {
    TextureView(TextureViewHandle),
    Swapchain(SwapchainHandle),
}
impl ColorViewBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &ColorView,
    ) -> Result<Self, ResourceBuilderError> {
        match descriptor {
            ColorView::TextureView(ref id) => match resource_manager.texture_view_handle_ref(id) {
                Some(texture_view) => Ok(Self::TextureView(texture_view.clone())),
                None => {
                    log::error!(target: "EntityManager","Failed to gather Command::RenderPass resources: TextureView {} not found",id);
                    Err(ResourceBuilderError::MissingDependencies)
                }
            },
            ColorView::Swapchain(ref id) => match resource_manager.swapchain_handle_ref(id) {
                Some(swapchain) => {
                    if swapchain.current_frame().is_none(){return Err(ResourceBuilderError::MissingDependencies);}
                    Ok(Self::Swapchain(swapchain.clone()))
                },
                None => {
                    log::error!(target: "EntityManager","Failed to gather Command::RenderPass resources: Swapchain {} not found",id);
                    Err(ResourceBuilderError::MissingDependencies)
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderPassColorAttachmentBuilder {
    pub view: ColorViewBuilder,
    pub resolve_target: Option<TextureViewHandle>,
    pub ops: wgpu::Operations<wgpu::Color>,
}
impl RenderPassColorAttachmentBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &RenderPassColorAttachment,
    ) -> Result<Self, ResourceBuilderError> {
        let view = ColorViewBuilder::new(resource_manager, &descriptor.view)?;

        let resolve_target = match descriptor.resolve_target {
            Some(ref texture_view) => {
                match resource_manager.texture_view_handle_ref(texture_view) {
                    Some(texture_view) => Some(texture_view.clone()),
                    None => {
                        log::error!(target: "EntityManager","Failed to gather Command::RenderPass resources: TextureView {} not found",texture_view);
                        return Err(ResourceBuilderError::MissingDependencies);
                    }
                }
            }
            None => None,
        };

        let ops = descriptor.ops.clone();

        Ok(Self {
            view,
            resolve_target,
            ops,
        })
    }
    pub fn build<'a>(
        &'a self,
        support: &'a mut Option<MutexGuard<'a, Option<wgpu::SwapChainFrame>>>,
    ) -> wgpu::RenderPassColorAttachment<'a> {
        let view = match &self.view {
            ColorViewBuilder::TextureView(view) => view.as_ref(),
            ColorViewBuilder::Swapchain(swapchain) => {
                *support = Some(swapchain.current_frame());
                &support.as_ref().unwrap().as_ref().unwrap().output.view
            }
        };

        wgpu::RenderPassColorAttachment {
            view,
            resolve_target: self
                .resolve_target
                .as_ref()
                .map(|texture_view| texture_view.as_ref()),
            ops: self.ops.clone(),
        }
    }
}

/*
#[derive(Debug)]
pub enum ColorViewPreBuilder<'a> {
    TextureView(TextureViewHandle),
    Swapchain(MutexGuard<'a,Option<wgpu::SwapChainFrame>>)
}

#[derive(Debug)]
pub struct RenderPassColorAttachmentPreBuilder<'a> {
    pub view: ColorViewPreBuilder<'a>,
    pub resolve_target: Option<TextureViewHandle>,
    pub ops: wgpu::Operations<wgpu::Color>,
}
impl<'a> RenderPassColorAttachmentPreBuilder<'a> {
    pub fn new(descriptor: RenderPassColorAttachmentBuilder)-> Self {
        let view = match descriptor.view {
            ColorViewBuilder::TextureView(texture_view)=>ColorViewPreBuilder::TextureView(texture_view),
            ColorViewBuilder::Swapchain(swapchain)=>ColorViewPreBuilder::Swapchain(swapchain.current_frame())
        };
        Self {
            view,
            resolve_target: descriptor.resolve_target.clone(),
            ops: descriptor.ops.clone()
        }
    }
    pub fn build(&self) -> wgpu::RenderPassColorAttachment {
        let view = match &self.view {
            ColorViewPreBuilder::TextureView(view)=>view.as_ref(),
            ColorViewPreBuilder::Swapchain(swapchain)=>&swapchain.as_ref().unwrap().output.view
        };

        wgpu::RenderPassColorAttachment {
            view,
            resolve_target: self.resolve_target.as_ref().map(|texture_view|texture_view.as_ref()),
            ops: self.ops.clone()
        }
    }
}
*/

#[derive(Debug, Clone)]
pub enum ColorTargetBuilder {
    Swapchain(SwapchainHandle),
    TextureView(TextureViewHandle),
}
impl ColorTargetBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &ColorTarget,
    ) -> Result<Self, ResourceBuilderError> {
        match descriptor {
            ColorTarget::Swapchain(swapchain) => {
                match resource_manager.swapchain_handle_ref(&swapchain) {
                    Some(swapchain) => Ok(Self::Swapchain(swapchain.clone())),
                    None => {
                        log::error!(target: "EntityManager","Failed to gather Command::RenderPass resources: Swapchain {} not found",swapchain);
                        Err(ResourceBuilderError::MissingDependencies)
                    }
                }
            }
            ColorTarget::TextureView(texture_view) => {
                match resource_manager.texture_view_handle_ref(&texture_view) {
                    Some(texture_view) => Ok(Self::TextureView(texture_view.clone())),
                    None => {
                        log::error!(target: "EntityManager","Failed to gather Command::RenderPass resources: TextureView {} not found",texture_view);
                        Err(ResourceBuilderError::MissingDependencies)
                    }
                }
            }
        }
    }
    /*
    pub fn as_attachment(&self)->wgpu::RenderPassColorAttachment {
        let texture_view = match self {
            Self::Swapchain(swapchain)=>swapchain.current_frame().as_ref().unwrap().output.view,
            Self::TextureView(texture_view)=>texture_view.as_ref()
        };
        wgpu::RenderPassColorAttachment{
            view: texture_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            },
        }
    }*/
}

/*
if color_targets.is_none() {
    let current_color_targets = if let RenderCommand::SetPipeline { pipeline } =
        command
    {
        match resource_manager.render_pipeline_descriptor_ref(&pipeline) {
            Some(descriptor) => match &descriptor.fragment {
                Some(fragment) => {
                    let result: Result<_, _> = fragment
                        .targets
                        .iter()
                        .map(|target_state| {
                            ColorTargetBuilder::new(
                                resource_manager,
                                &target_state.target,
                            )
                        })
                        .collect();
                    match result {
                        Ok(color_targets) => color_targets,
                        Err(err) => return Err(err),
                    }
                }
                None => Vec::new(),
            },
            None => {
                log::error!(target: "EntityManager","Failed to gather Command::RenderPass resources: RenderPipeline {} not found",pipeline);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        }
    } else {
        Vec::new()
    };
    color_targets = Some(current_color_targets);
}
*/

#[derive(Debug, Clone)]
pub enum CommandBuilder {
    BufferToBuffer(BufferToBufferCopyBuilder),
    BufferToTexture(BufferToTextureCopyBuilder),
    TextureToTexture(TextureToTextureCopyBuilder),
    TextureToBuffer(TextureToBufferCopyBuilder),
    ComputePass {
        commands: Vec<ComputeCommandBuilder>,
    },
    RenderPass {
        label: String,
        color_attachments: Vec<RenderPassColorAttachmentBuilder>,
        depth_stencil: Option<TextureViewHandle>,
        commands: Vec<RenderCommandBuilder>,
    },
}
impl CommandBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        descriptor: &Command,
    ) -> Result<Self, ResourceBuilderError> {
        match descriptor {
            Command::BufferToBuffer(descriptor) => {
                match BufferToBufferCopyBuilder::new(resource_manager, descriptor) {
                    Ok(builder) => Ok(Self::BufferToBuffer(builder)),
                    Err(err) => Err(err),
                }
            }
            Command::BufferToTexture(descriptor) => {
                match BufferToTextureCopyBuilder::new(resource_manager, descriptor) {
                    Ok(builder) => Ok(Self::BufferToTexture(builder)),
                    Err(err) => Err(err),
                }
            }
            Command::TextureToTexture(descriptor) => {
                match TextureToTextureCopyBuilder::new(resource_manager, descriptor) {
                    Ok(builder) => Ok(Self::TextureToTexture(builder)),
                    Err(err) => Err(err),
                }
            }
            Command::TextureToBuffer(descriptor) => {
                match TextureToBufferCopyBuilder::new(resource_manager, descriptor) {
                    Ok(builder) => Ok(Self::TextureToBuffer(builder)),
                    Err(err) => Err(err),
                }
            }
            Command::ComputePass(commands) => {
                let mut command_builders = Vec::new();
                for command in commands {
                    match ComputeCommandBuilder::new(resource_manager, command) {
                        Ok(command_builder) => command_builders.push(command_builder),
                        Err(err) => return Err(err),
                    }
                }
                Ok(Self::ComputePass {
                    commands: command_builders,
                })
            }
            Command::RenderPass {
                label,
                color_attachments,
                depth_stencil,
                commands,
            } => {
                let label = label.clone();

                let depth_stencil = depth_stencil.map(|depth_stencil|{
                    match resource_manager.texture_view_handle_ref(&depth_stencil) {
                        Some(depth_stencil) => Ok(depth_stencil.clone()),
                        None => {
                            log::error!(target: "EntityManager","Failed to gather Command::RenderPass resources: DepthStencil {} not found",depth_stencil);
                            return Err(ResourceBuilderError::MissingDependencies);
                        },
                    }
                });

                let depth_stencil = match depth_stencil {
                    Some(Ok(depth_stencil)) => Some(depth_stencil),
                    Some(Err(err)) => return Err(err),
                    None => None,
                };

                let mut color_attachment_builders = Vec::new();
                for color_attachment in color_attachments {
                    let builder =
                        RenderPassColorAttachmentBuilder::new(resource_manager, color_attachment)?;
                    color_attachment_builders.push(builder);
                }

                let mut command_builders = Vec::new();
                for command in commands {
                    match RenderCommandBuilder::new(resource_manager, command) {
                        Ok(command_builder) => command_builders.push(command_builder),
                        Err(err) => return Err(err),
                    }
                }

                Ok(Self::RenderPass {
                    label,
                    depth_stencil,
                    color_attachments: color_attachment_builders,
                    commands: command_builders,
                })
            }
        }
    }
    pub fn build(&self, encoder: &mut wgpu::CommandEncoder) -> bool {
        match self {
            Self::BufferToBuffer(command_builder) => command_builder.build(encoder),
            Self::BufferToTexture(command_builder) => command_builder.build(encoder),
            Self::TextureToTexture(command_builder) => command_builder.build(encoder),
            Self::TextureToBuffer(command_builder) => command_builder.build(encoder),
            Self::ComputePass { commands } => {
                let mut compute_pass =
                    encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });

                for command in commands {
                    if !command.build(&mut compute_pass) {
                        return false;
                    }
                }
                true
            }

            Self::RenderPass {
                label,
                depth_stencil,
                color_attachments,
                commands,
            } => {
                enum Temp<'a> {
                    Lock(std::sync::MutexGuard<'a, Option<wgpu::SwapChainFrame>>),
                    View(&'a Arc<wgpu::TextureView>),
                }

                let color_attachments: Vec<_> = color_attachments
                    .iter()
                    .map(|attachment| {
                        let view = match &attachment.view {
                            ColorViewBuilder::TextureView(texture_view) => Temp::View(texture_view),
                            ColorViewBuilder::Swapchain(swapchain) => {
                                Temp::Lock(swapchain.current_frame())
                            }
                        };
                        (attachment, view)
                    })
                    .collect();
                let color_attachments: Vec<_> = color_attachments
                    .iter()
                    .map(|(attachment, view)| {
                        let view = match view {
                            Temp::View(view) => view.as_ref(),
                            Temp::Lock(lock) => &lock.as_ref().unwrap().output.view,
                        };

                        wgpu::RenderPassColorAttachment {
                            view,
                            resolve_target: attachment
                                .resolve_target
                                .as_ref()
                                .map(|texture_view| texture_view.as_ref()),
                            ops: attachment.ops.clone(),
                        }
                    })
                    .collect();

                let depth_stencil_attachment = depth_stencil.as_ref().map(|depth_stencil| {
                    wgpu::RenderPassDepthStencilAttachment {
                        view: depth_stencil.as_ref(),
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        }),
                        stencil_ops: None,
                    }
                });

                let render_pass_descriptor = wgpu::RenderPassDescriptor {
                    label: Some(label.as_str()),
                    color_attachments: &color_attachments,
                    depth_stencil_attachment,
                };

                let mut render_pass = encoder.begin_render_pass(&render_pass_descriptor);
                for command in commands {
                    if !command.build(&mut render_pass) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommandBufferBuilder {
    pub id: CommandBufferId,
    pub device: DeviceHandle,
    pub label: String,
    pub commands: Vec<CommandBuilder>,
}
impl CommandBufferBuilder {
    pub fn new(
        resource_manager: &ResourceManager,
        id: CommandBufferId,
        descriptor: &CommandBufferDescriptor,
    ) -> Result<Self, ResourceBuilderError> {
        let device = match resource_manager.device_handle_ref(&descriptor.device) {
            Some(device) => device.clone(),
            None => {
                log::error!(target: "EntityManager","Failed to gather Buffer resources: parent Device of {} not found",id);
                return Err(ResourceBuilderError::MissingDependencies);
            }
        };
        let mut commands = Vec::new();
        for command in &descriptor.commands {
            let command_builder = match CommandBuilder::new(resource_manager, command) {
                Ok(command_builder) => command_builder,
                Err(err) => return Err(err),
            };
            commands.push(command_builder);
        }
        let label = descriptor.label.clone();
        Ok(Self {
            id,
            device,
            label,
            commands,
        })
    }
    pub fn build(&self) -> CommandBufferHandle {
        let descriptor = wgpu::CommandEncoderDescriptor {
            label: Some(self.label.as_str()),
        };

        let mut encoder = self.device.1.create_command_encoder(&descriptor);
        for command in &self.commands {
            command.build(&mut encoder);
        }
        log::info!(target: "EntityManager","Building {}",self.id);
        Arc::new(encoder.finish())
    }
}
