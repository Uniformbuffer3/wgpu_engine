//! Descriptors for the resources.

use crate::common::*;
use crate::engine::resource_manager::ResourceManager;

pub use crate::wgpu::{
    AddressMode, BindGroupLayoutEntry, CompareFunction, ComputePass, DrmFormat,
    DrmFormatImageProperties, DrmModifier, Extent3d, Features, FilterMode, Limits, PlaneLayout,
    QuerySetDescriptor, RenderPass, Sampler, SamplerBorderColor, ShaderStage, SwapChainDescriptor,
    TextureAspect, TextureDimension, TextureFormat, TextureUsage, TextureViewDimension,
};

pub mod instance;
pub use instance::*;

pub mod device;
pub use device::*;

pub mod swapchain;
pub use swapchain::*;

pub mod buffer;
pub use buffer::*;

pub mod texture;
pub use texture::*;

pub mod texture_view;
pub use texture_view::*;

pub mod sampler;
pub use sampler::*;

mod shader_module;
pub use shader_module::*;

pub mod bind_group_layout;
pub use bind_group_layout::*;

pub mod bind_group;
pub use bind_group::*;

pub mod pipeline_layout;
pub use pipeline_layout::*;

pub mod render_pipeline;
pub use render_pipeline::*;

pub mod compute_pipeline;
pub use compute_pipeline::*;

pub mod command_buffer;
pub use command_buffer::*;

#[derive(Debug, Clone, Copy, PartialEq)]
/**
A stateless resource do no contains data or informations other than its descriptor.
A statefull data contains additional data other than its descriptor.
*/
pub enum StateType {
    Stateless,
    Statefull,
}

/// The implementor object have owers.
pub trait HaveOwners {
    type O: Clone + PartialEq;
    /// Returns the owner of the object.
    fn owners(&self) -> Vec<Self::O>;
    /// Returns a reference of the owner of the object.
    fn owners_ref(&self) -> &Vec<Self::O>;
    /// Returns a mutable reference of the owner of the object.
    fn owners_mut(&mut self) -> &mut Vec<Self::O>;
}

/// The implementor object have dependencies.
pub trait HaveDependencies {
    /// Returns the dependencies of the object.
    fn dependencies(&self) -> Vec<EntityId>;
}

/// The implementor object have a descriptor.
pub trait HaveDescriptor: HaveDependencies {
    type D: Clone + PartialEq;
    /// Returns the descriptor of the object.
    fn descriptor(&self) -> Self::D;
    /// Returns a reference of the descriptor of the object.
    fn descriptor_ref(&self) -> &Self::D;
    /// Returns a mutable reference of the descriptor of the object.
    fn descriptor_mut(&mut self) -> &mut Self::D;
    /// Returns the state type of the object.
    fn state_type(&self) -> StateType;
    /// Returns true if object needs to be updated.
    fn needs_update(&self, other: &Self::D) -> bool;
}

/// The implementor object have an handle.
pub trait HaveHandle {
    type H;
    fn handle_ref(&self) -> &Self::H;
    fn handle_mut(&mut self) -> &mut Self::H;
}

/// The implementor object have a descriptor and an handle.
pub trait HaveDescriptorAndHandle: HaveDescriptor + HaveHandle {}

#[derive(Debug, Clone, PartialEq)]
/**
A enum combining all the possible resource descriptors.
*/
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
    fn state_type(&self) -> StateType {
        match self {
            Self::Instance(descriptor) => descriptor.state_type(),
            Self::Device(descriptor) => descriptor.state_type(),
            Self::Swapchain(descriptor) => descriptor.state_type(),

            Self::Buffer(descriptor) => descriptor.state_type(),
            Self::Texture(descriptor) => descriptor.state_type(),
            Self::TextureView(descriptor) => descriptor.state_type(),
            Self::Sampler(descriptor) => descriptor.state_type(),
            Self::ShaderModule(descriptor) => descriptor.state_type(),

            Self::BindGroupLayout(descriptor) => descriptor.state_type(),
            Self::BindGroup(descriptor) => descriptor.state_type(),

            Self::PipelineLayout(descriptor) => descriptor.state_type(),
            Self::RenderPipeline(descriptor) => descriptor.state_type(),
            Self::ComputePipeline(descriptor) => descriptor.state_type(),
            Self::CommandBuffer(descriptor) => descriptor.state_type(),
        }
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
    pub fn record(&self, resources: &ResourceManager, queue: &crate::wgpu::Queue) {
        match self {
            Self::Buffer(write) => {
                let buffer = resources.buffer_handle_ref(&write.buffer).unwrap();
                queue.write_buffer(buffer, write.offset, write.data.as_slice());
            }
            Self::Texture(write) => {
                let wgpu_dst = crate::wgpu::ImageCopyTexture {
                    texture: resources.texture_handle_ref(&write.texture).unwrap(),
                    mip_level: write.mip_level,
                    origin: write.origin,
                };
                queue.write_texture(wgpu_dst, write.data.as_slice(), write.layout, write.size);
            }
        }
    }
}
