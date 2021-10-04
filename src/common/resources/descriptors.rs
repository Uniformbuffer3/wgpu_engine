use crate::common::*;
use crate::engine::resource_manager::ResourceManager;
use std::ops::Range;

pub trait HaveOwners {
    type O: Clone + PartialEq;
    fn owners(&self) -> Vec<Self::O>;
    fn owners_ref(&self) -> &Vec<Self::O>;
    fn owners_mut(&mut self) -> &mut Vec<Self::O>;
}

pub trait HaveDependencies {
    fn dependencies(&self) -> Vec<EntityId>;
}

pub trait HaveDescriptor: HaveDependencies {
    type D: Clone + PartialEq;
    fn descriptor(&self) -> Self::D;
    fn descriptor_ref(&self) -> &Self::D;
    fn descriptor_mut(&mut self) -> &mut Self::D;
    fn needs_update(&self, other: &Self::D) -> bool;
}

pub trait HaveHandle {
    type H;
    fn handle_ref(&self) -> &Self::H;
    fn handle_mut(&mut self) -> &mut Self::H;
}

pub trait HaveDescriptorAndHandle: HaveDescriptor + HaveHandle {}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceDescriptor {
    Instance(InstanceDescriptor),
    Device(DeviceDescriptor),
    Swapchain(SwapchainDescriptor),

    Buffer(BufferDescriptor),
    Texture(TextureDescriptor),
    TextureView(TextureViewDescriptor),
    Sampler(SamplerDescriptor),
    ShaderModule(ShaderModuleDescriptor),

    BindGroupLayout(BindGroupLayoutDescriptor),
    BindGroup(BindGroupDescriptor),

    PipelineLayout(PipelineLayoutDescriptor),
    RenderPipeline(RenderPipelineDescriptor),
    ComputePipeline(ComputePipelineDescriptor),
    CommandBuffer(CommandBufferDescriptor),
}
impl HaveDependencies for ResourceDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        match self {
            Self::Instance(descriptor) => descriptor.dependencies(),
            Self::Device(descriptor) => descriptor.dependencies(),
            Self::Swapchain(descriptor) => descriptor.dependencies(),

            Self::Buffer(descriptor) => descriptor.dependencies(),
            Self::Texture(descriptor) => descriptor.dependencies(),
            Self::TextureView(descriptor) => descriptor.dependencies(),
            Self::Sampler(descriptor) => descriptor.dependencies(),
            Self::ShaderModule(descriptor) => descriptor.dependencies(),

            Self::BindGroupLayout(descriptor) => descriptor.dependencies(),
            Self::BindGroup(descriptor) => descriptor.dependencies(),

            Self::PipelineLayout(descriptor) => descriptor.dependencies(),
            Self::RenderPipeline(descriptor) => descriptor.dependencies(),
            Self::ComputePipeline(descriptor) => descriptor.dependencies(),
            Self::CommandBuffer(descriptor) => descriptor.dependencies(),
        }
    }
}
impl HaveDescriptor for ResourceDescriptor {
    type D = Self;
    fn descriptor(&self) -> Self::D {
        self.clone()
    }
    fn descriptor_ref(&self) -> &Self::D {
        self
    }
    fn descriptor_mut(&mut self) -> &mut Self::D {
        self
    }
    fn needs_update(&self, _other: &Self::D) -> bool {
        true
    }
}
impl From<InstanceDescriptor> for ResourceDescriptor {
    fn from(descriptor: InstanceDescriptor) -> Self {
        Self::Instance(descriptor)
    }
}
impl From<DeviceDescriptor> for ResourceDescriptor {
    fn from(descriptor: DeviceDescriptor) -> Self {
        Self::Device(descriptor)
    }
}
impl From<SwapchainDescriptor> for ResourceDescriptor {
    fn from(descriptor: SwapchainDescriptor) -> Self {
        Self::Swapchain(descriptor)
    }
}
impl From<BufferDescriptor> for ResourceDescriptor {
    fn from(descriptor: BufferDescriptor) -> Self {
        Self::Buffer(descriptor)
    }
}
impl From<TextureDescriptor> for ResourceDescriptor {
    fn from(descriptor: TextureDescriptor) -> Self {
        Self::Texture(descriptor)
    }
}
impl From<TextureViewDescriptor> for ResourceDescriptor {
    fn from(descriptor: TextureViewDescriptor) -> Self {
        Self::TextureView(descriptor)
    }
}
impl From<ShaderModuleDescriptor> for ResourceDescriptor {
    fn from(descriptor: ShaderModuleDescriptor) -> Self {
        Self::ShaderModule(descriptor)
    }
}
impl From<SamplerDescriptor> for ResourceDescriptor {
    fn from(descriptor: SamplerDescriptor) -> Self {
        Self::Sampler(descriptor)
    }
}
impl From<BindGroupDescriptor> for ResourceDescriptor {
    fn from(descriptor: BindGroupDescriptor) -> Self {
        Self::BindGroup(descriptor)
    }
}
impl From<BindGroupLayoutDescriptor> for ResourceDescriptor {
    fn from(descriptor: BindGroupLayoutDescriptor) -> Self {
        Self::BindGroupLayout(descriptor)
    }
}
impl From<PipelineLayoutDescriptor> for ResourceDescriptor {
    fn from(descriptor: PipelineLayoutDescriptor) -> Self {
        Self::PipelineLayout(descriptor)
    }
}
impl From<RenderPipelineDescriptor> for ResourceDescriptor {
    fn from(descriptor: RenderPipelineDescriptor) -> Self {
        Self::RenderPipeline(descriptor)
    }
}
impl From<ComputePipelineDescriptor> for ResourceDescriptor {
    fn from(descriptor: ComputePipelineDescriptor) -> Self {
        Self::ComputePipeline(descriptor)
    }
}
impl From<CommandBufferDescriptor> for ResourceDescriptor {
    fn from(descriptor: CommandBufferDescriptor) -> Self {
        Self::CommandBuffer(descriptor)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstanceDescriptor {
    pub label: String,
    pub backend: wgpu::BackendBit,
}
impl HaveDependencies for InstanceDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeviceDescriptor {
    pub label: String,
    pub instance: InstanceId,
    pub backend: wgpu::BackendBit,
    pub pci_id: usize,
    pub features: wgpu::Features,
    pub limits: wgpu::Limits,
}
impl HaveDependencies for DeviceDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![self.instance.id_ref().clone()]
    }
}

#[derive(Debug, Clone)]
pub struct SwapchainDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub surface: std::sync::Arc<wgpu::Surface>,
    pub usage: wgpu::TextureUsage,
    pub format: wgpu::TextureFormat,
    pub width: u32,
    pub height: u32,
    pub present_mode: wgpu::PresentMode,
}
impl HaveDependencies for SwapchainDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![self.device.id_ref().clone()]
    }
}
impl PartialEq for SwapchainDescriptor {
    fn eq(&self, other: &Self) -> bool {
        if self.label != other.label {return false;}
        if self.device != other.device {return false;}
        if !std::sync::Arc::ptr_eq(&self.surface,&other.surface) {return false;}
        if self.usage != other.usage {return false;}
        if self.format != other.format {return false;}
        if self.width != other.width {return false;}
        if self.height != other.height {return false;}
        if self.present_mode != other.present_mode {return false;}
        true
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShaderSource {
    SpirV(Vec<u32>),
    Wgsl(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShaderModuleDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub source: ShaderSource,
    pub flags: wgpu::ShaderFlags,
}
impl HaveDependencies for ShaderModuleDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![*self.device.id_ref()]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandEncoderDescriptor {
    pub label: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderBundleEncoderDescriptor {
    pub label: String,
    pub color_formats: Vec<wgpu::TextureFormat>,
    pub depth_stencil_format: Option<wgpu::TextureFormat>,
    pub sample_count: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BufferBinding {
    pub buffer: BufferId, //Arc<wgpu::Buffer>
    pub offset: wgpu::BufferAddress,
    pub size: Option<wgpu::BufferSize>,
}
impl HaveDependencies for BufferBinding {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![self.buffer.id_ref().clone()]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BindingResource {
    Buffer(BufferBinding),
    BufferArray(Vec<BufferBinding>),
    Sampler(SamplerId),                   //Arc<wgpu::Sampler>
    TextureView(TextureViewId),           //Arc<wgpu::TextureView>
    TextureViewArray(Vec<TextureViewId>), //Arc<wgpu::TextureView>
}
impl HaveDependencies for BindingResource {
    fn dependencies(&self) -> Vec<EntityId> {
        match self {
            Self::Buffer(descriptor) => descriptor.dependencies(),
            Self::BufferArray(descriptors) => descriptors
                .iter()
                .map(|descriptor| descriptor.dependencies())
                .flatten()
                .collect(),
            Self::Sampler(id) => vec![id.id_ref().clone()],
            Self::TextureView(id) => vec![id.id_ref().clone()], //Arc<wgpu::TextureView>
            Self::TextureViewArray(ids) => ids.iter().map(|id| id.id_ref().clone()).collect(), //Arc<wgpu::TextureView>
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BindGroupEntry {
    pub binding: u32,
    pub resource: BindingResource,
}
impl HaveDependencies for BindGroupEntry {
    fn dependencies(&self) -> Vec<EntityId> {
        self.resource.dependencies()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BindGroupDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub layout: BindGroupLayoutId, //Arc<wgpu::BindGroupLayout>
    pub entries: Vec<BindGroupEntry>,
}
impl HaveDependencies for BindGroupDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        std::iter::once(*self.device.id_ref())
        .chain(std::iter::once(*self.layout.id_ref()))
        .chain(
            self.entries
                .iter()
                .map(|descriptor| descriptor.dependencies())
                .flatten(),
        )
        .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BindGroupLayoutDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub entries: Vec<wgpu::BindGroupLayoutEntry>,
}
impl HaveDependencies for BindGroupLayoutDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![*self.device.id_ref()]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PipelineLayoutDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub bind_group_layouts: Vec<BindGroupLayoutId>, //Arc<wgpu::BindGroupLayout>
    pub push_constant_ranges: Vec<wgpu::PushConstantRange>,
}
impl HaveDependencies for PipelineLayoutDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        std::iter::once(*self.device.id_ref())
            .chain(self.bind_group_layouts.iter().map(|id| id.id_ref().clone()))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VertexBufferLayout {
    pub array_stride: wgpu::BufferAddress,
    pub step_mode: wgpu::InputStepMode,
    pub attributes: Vec<wgpu::VertexAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VertexState {
    pub module: ShaderModuleId, //Arc<wgpu::ShaderModule>
    pub entry_point: String,
    pub buffers: Vec<VertexBufferLayout>,
}
impl HaveDependencies for VertexState {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![self.module.id_ref().clone()]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorTarget {
    Swapchain(SwapchainId),
    TextureView(TextureViewId),
}
impl HaveDependencies for ColorTarget {
    fn dependencies(&self) -> Vec<EntityId> {
        match self {
            Self::Swapchain(id) => vec![id.id_ref().clone()],
            Self::TextureView(id) => vec![id.id_ref().clone()],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorTargetState {
    pub target: ColorTarget,
    pub blend: Option<wgpu::BlendState>,
    pub write_mask: wgpu::ColorWrite,
}
impl HaveDependencies for ColorTargetState {
    fn dependencies(&self) -> Vec<EntityId> {
        self.target.dependencies()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FragmentState {
    pub module: ShaderModuleId, //Arc<wgpu::ShaderModule>
    pub entry_point: String,
    pub targets: Vec<wgpu::ColorTargetState>,
}
impl HaveDependencies for FragmentState {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![self.module.id_ref().clone()]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DepthStencilState {
    pub id: TextureViewId,
    pub depth_write_enabled: bool,
    pub depth_compare: wgpu::CompareFunction,
    pub stencil: wgpu::StencilState,
    pub bias: wgpu::DepthBiasState,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderPipelineDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub layout: Option<PipelineLayoutId>, //Arc<wgpu::PipelineLayout>
    pub vertex: VertexState,
    pub primitive: wgpu::PrimitiveState,
    pub depth_stencil: Option<DepthStencilState>,
    pub multisample: wgpu::MultisampleState,
    pub fragment: Option<FragmentState>,
}

impl HaveDependencies for RenderPipelineDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        std::iter::once(*self.device.id_ref())
            .chain(self.layout.map(|id| id.id_ref().clone()))
            .chain(self.vertex.dependencies())
            .chain(
                self.fragment
                    .iter()
                    .map(|fragment| fragment.dependencies())
                    .flatten(),
            )
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComputePipelineDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub layout: Option<PipelineLayoutId>, //Arc<wgpu::PipelineLayout>
    pub module: ShaderModuleId,           //Arc<wgpu::ShaderModule>
    pub entry_point: String,
}
impl HaveDependencies for ComputePipelineDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        std::iter::once(*self.device.id_ref())
            .chain(self.layout.map(|id| id.id_ref().clone()))
            .chain(std::iter::once(*self.module.id_ref()))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BufferDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub size: wgpu::BufferAddress,
    pub usage: wgpu::BufferUsage,
}
impl HaveDependencies for BufferDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![*self.device.id_ref()]
    }
}
use std::os::unix::io::RawFd;
#[derive(Debug, Clone, PartialEq)]
pub enum TextureSource {
    DmaBuf(RawFd, Option<wgpu::DrmFormatImageProperties>),
    //Ptr(std::sync::Arc<std::ffi::c_void>),
    Local,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub source: TextureSource,
    pub usage: wgpu::TextureUsage,
    pub size: wgpu::Extent3d,
    pub format: wgpu::TextureFormat,
    pub dimension: wgpu::TextureDimension,
    pub mip_level_count: u32,
    pub sample_count: u32,
}
impl HaveDependencies for TextureDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![*self.device.id_ref()]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureViewDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub texture: TextureId,
    pub format: wgpu::TextureFormat,
    pub dimension: wgpu::TextureViewDimension,
    pub aspect: wgpu::TextureAspect,
    pub base_mip_level: u32,
    pub mip_level_count: Option<std::num::NonZeroU32>,
    pub base_array_layer: u32,
    pub array_layer_count: Option<std::num::NonZeroU32>,
}
impl HaveDependencies for TextureViewDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        std::iter::once(*self.device.id_ref())
            .chain(std::iter::once(*self.texture.id_ref()))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SamplerDescriptor {
    pub label: String,
    pub device: DeviceId,
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
impl HaveDependencies for SamplerDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![*self.device.id_ref()]
    }
}

pub use std::num::NonZeroU8;
pub use wgpu::{
    AddressMode, BindGroupLayoutEntry, CompareFunction, ComputePass, FilterMode,
    QuerySetDescriptor, RenderPass, Sampler, SamplerBorderColor, SwapChainDescriptor,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CommandBufferDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub commands: Vec<Command>,
}
impl CommandBufferDescriptor {
    pub fn swapchains(&self) -> Vec<SwapchainId> {
        self.commands
            .iter()
            .map(|command| command.swapchains())
            .flatten()
            .collect()
    }
}
impl HaveDependencies for CommandBufferDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        std::iter::once(*self.device.id_ref())
            .chain(
                self.commands
                    .iter()
                    .map(|descriptor| descriptor.dependencies())
                    .flatten(),
            )
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    BufferToBuffer(BufferToBufferCopy),
    BufferToTexture(BufferToTextureCopy),
    TextureToTexture(TextureToTextureCopy),
    TextureToBuffer(TextureToBufferCopy),
    ComputePass(Vec<ComputeCommand>),
    RenderPass {
        label: String,
        depth_stencil: Option<TextureViewId>,
        color_attachments: Vec<RenderPassColorAttachment>,
        commands: Vec<RenderCommand>,
    },
}
impl Command {
    pub fn swapchains(&self) -> Vec<SwapchainId> {
        if let Command::RenderPass {
            label: _,
            depth_stencil: _,
            color_attachments,
            commands: _,
        } = self
        {
            color_attachments
                .iter()
                .map(|attachment| attachment.swapchains())
                .flatten()
                .collect()
        } else {
            Vec::new()
        }
    }
}
impl HaveDependencies for Command {
    fn dependencies(&self) -> Vec<EntityId> {
        match self {
            Self::BufferToBuffer(descriptor) => descriptor.dependencies(),
            Self::BufferToTexture(descriptor) => descriptor.dependencies(),
            Self::TextureToTexture(descriptor) => descriptor.dependencies(),
            Self::TextureToBuffer(descriptor) => descriptor.dependencies(),
            Self::ComputePass(descriptors) => descriptors
                .iter()
                .map(|descriptor| descriptor.dependencies())
                .flatten()
                .collect(),
            Self::RenderPass {
                label: _,
                depth_stencil,
                color_attachments,
                commands,
            } => std::iter::empty()
                .chain(
                    depth_stencil
                        .iter()
                        .map(|depth_stencil| *depth_stencil.id_ref()),
                )
                .chain(
                    color_attachments
                        .iter()
                        .map(|attachment| attachment.dependencies())
                        .flatten(),
                )
                .chain(
                    commands
                        .iter()
                        .map(|descriptor| descriptor.dependencies())
                        .flatten(),
                )
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ColorView {
    TextureView(TextureViewId),
    Swapchain(SwapchainId),
}
impl ColorView {
    pub fn swapchains(&self) -> Vec<SwapchainId> {
        match self {
            Self::Swapchain(id) => vec![*id],
            _ => Vec::new(),
        }
    }
}
impl HaveDependencies for ColorView {
    fn dependencies(&self) -> Vec<EntityId> {
        match self {
            Self::TextureView(id) => vec![*id.id_ref()],
            Self::Swapchain(id) => vec![*id.id_ref()],
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderPassColorAttachment {
    pub view: ColorView,
    pub resolve_target: Option<TextureViewId>,
    pub ops: wgpu::Operations<wgpu::Color>,
}
impl RenderPassColorAttachment {
    pub fn swapchains(&self) -> Vec<SwapchainId> {
        self.view.swapchains()
    }
}
impl HaveDependencies for RenderPassColorAttachment {
    fn dependencies(&self) -> Vec<EntityId> {
        self.view
            .dependencies()
            .into_iter()
            .chain(self.resolve_target.iter().map(|id| *id.id_ref()))
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComputeCommand {}
impl HaveDependencies for ComputeCommand {
    fn dependencies(&self) -> Vec<EntityId> {
        Vec::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Slice<T> {
    Range(std::ops::Range<T>),
    RangeFrom(std::ops::RangeFrom<T>),
    RangeTo(std::ops::RangeTo<T>),
    RangeFull(std::ops::RangeFull),
    RangeInclusive(std::ops::RangeInclusive<T>),
    RangeToInclusive(std::ops::RangeToInclusive<T>),
}
impl<T> std::ops::RangeBounds<T> for Slice<T> {
    fn start_bound(&self) -> std::ops::Bound<&T> {
        match self {
            Self::Range(range) => range.start_bound(),
            Self::RangeFrom(range) => range.start_bound(),
            Self::RangeTo(range) => range.start_bound(),
            Self::RangeFull(range) => range.start_bound(),
            Self::RangeInclusive(range) => range.start_bound(),
            Self::RangeToInclusive(range) => range.start_bound(),
        }
    }
    fn end_bound(&self) -> std::ops::Bound<&T> {
        match self {
            Self::Range(range) => range.end_bound(),
            Self::RangeFrom(range) => range.end_bound(),
            Self::RangeTo(range) => range.end_bound(),
            Self::RangeFull(range) => range.end_bound(),
            Self::RangeInclusive(range) => range.end_bound(),
            Self::RangeToInclusive(range) => range.end_bound(),
        }
    }
}
impl<T> From<std::ops::Range<T>> for Slice<T> {
    fn from(range: std::ops::Range<T>) -> Self {
        Self::Range(range)
    }
}
impl<T> From<std::ops::RangeFrom<T>> for Slice<T> {
    fn from(range: std::ops::RangeFrom<T>) -> Self {
        Self::RangeFrom(range)
    }
}
impl<T> From<std::ops::RangeTo<T>> for Slice<T> {
    fn from(range: std::ops::RangeTo<T>) -> Self {
        Self::RangeTo(range)
    }
}
impl<T> From<std::ops::RangeFull> for Slice<T> {
    fn from(range: std::ops::RangeFull) -> Self {
        Self::RangeFull(range)
    }
}
impl<T> From<std::ops::RangeInclusive<T>> for Slice<T> {
    fn from(range: std::ops::RangeInclusive<T>) -> Self {
        Self::RangeInclusive(range)
    }
}
impl<T> From<std::ops::RangeToInclusive<T>> for Slice<T> {
    fn from(range: std::ops::RangeToInclusive<T>) -> Self {
        Self::RangeToInclusive(range)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RenderCommand {
    SetPipeline {
        pipeline: RenderPipelineId,
    },
    SetPushConstants {
        stages: wgpu::ShaderStage,
        offset: u32,
        data: Vec<u8>,
    },
    SetBindGroup {
        index: u32,
        bind_group: BindGroupId,
        offsets: Vec<wgpu::DynamicOffset>,
    },
    SetVertexBuffer {
        slot: u32,
        buffer: BufferId,
        slice: Slice<wgpu::BufferAddress>,
    },
    SetIndexBuffer {
        index_format: wgpu::IndexFormat,
        buffer: BufferId,
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
impl HaveDependencies for RenderCommand {
    fn dependencies(&self) -> Vec<EntityId> {
        match self {
            Self::SetPipeline { pipeline } => vec![pipeline.id_ref().clone()],
            Self::SetPushConstants { .. } => Vec::new(),
            Self::SetBindGroup { bind_group, .. } => vec![bind_group.id_ref().clone()],
            Self::SetVertexBuffer { buffer, .. } => vec![buffer.id_ref().clone()],
            Self::SetIndexBuffer { buffer, .. } => vec![buffer.id_ref().clone()],
            Self::Draw { .. } => Vec::new(),
            Self::DrawIndexed { .. } => Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BufferToBufferCopy {
    pub src_buffer: BufferId,
    pub src_offset: wgpu::BufferAddress,
    pub dst_buffer: BufferId,
    pub dst_offset: wgpu::BufferAddress,
    pub size: wgpu::BufferAddress,
}
impl HaveDependencies for BufferToBufferCopy {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![
            self.src_buffer.id_ref().clone(),
            self.dst_buffer.id_ref().clone(),
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BufferToTextureCopy {
    pub src_buffer: BufferId,
    pub src_layout: wgpu::ImageDataLayout,
    pub dst_texture: TextureId,
    pub dst_mip_level: u32,
    pub dst_origin: wgpu::Origin3d,
    pub copy_size: wgpu::Extent3d,
}
impl HaveDependencies for BufferToTextureCopy {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![
            self.src_buffer.id_ref().clone(),
            self.dst_texture.id_ref().clone(),
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureToBufferCopy {
    pub src_texture: TextureId,
    pub src_mip_level: u32,
    pub src_origin: wgpu::Origin3d,
    pub dst_buffer: BufferId,
    pub dst_layout: wgpu::ImageDataLayout,
    pub copy_size: wgpu::Extent3d,
}
impl HaveDependencies for TextureToBufferCopy {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![
            self.src_texture.id_ref().clone(),
            self.dst_buffer.id_ref().clone(),
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureToTextureCopy {
    pub src_texture: TextureId,
    pub src_mip_level: u32,
    pub src_origin: wgpu::Origin3d,
    pub dst_texture: TextureId,
    pub dst_mip_level: u32,
    pub dst_origin: wgpu::Origin3d,
    pub copy_size: wgpu::Extent3d,
}
impl HaveDependencies for TextureToTextureCopy {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![
            self.src_texture.id_ref().clone(),
            self.dst_texture.id_ref().clone(),
        ]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BufferWrite {
    pub buffer: BufferId,
    pub offset: wgpu::BufferAddress,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureWrite {
    pub texture: TextureId,
    pub mip_level: u32,
    pub origin: wgpu::Origin3d,
    pub data: Vec<u8>,
    pub layout: wgpu::ImageDataLayout,
    pub size: wgpu::Extent3d,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceWrite {
    Buffer(BufferWrite),
    Texture(TextureWrite),
}
impl ResourceWrite {
    pub fn device(&self, resource_manager: &ResourceManager) -> DeviceId {
        match self {
            Self::Buffer(write) => {
                resource_manager
                    .buffer_descriptor_ref(&write.buffer)
                    .unwrap()
                    .device
            }
            Self::Texture(write) => {
                resource_manager
                    .texture_descriptor_ref(&write.texture)
                    .unwrap()
                    .device
            }
        }
    }
    pub fn record(&self, resources: &ResourceManager, queue: &wgpu::Queue) {
        match self {
            Self::Buffer(write) => {
                let buffer = resources.buffer_handle_ref(&write.buffer).unwrap();
                queue.write_buffer(buffer, write.offset, write.data.as_slice());
            }
            Self::Texture(write) => {
                let wgpu_dst = wgpu::ImageCopyTexture {
                    texture: resources.texture_handle_ref(&write.texture).unwrap(),
                    mip_level: write.mip_level,
                    origin: write.origin,
                };
                queue.write_texture(wgpu_dst, write.data.as_slice(), write.layout, write.size);
            }
        }
    }
}