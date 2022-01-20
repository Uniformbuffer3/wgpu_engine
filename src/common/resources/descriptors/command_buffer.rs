//! CommandBuffer related structures and enumerations.

use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::{
    BindGroupId, BufferId, DeviceId, RenderPipelineId, SwapchainId, TextureId, TextureViewId,
};

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [CommandBufferHandle][crate::common::resources::handles::CommandBufferHandle]
*/
pub struct CommandBufferDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub commands: Vec<Command>,
}
impl CommandBufferDescriptor {
    pub fn swapchains(&self) -> Vec<(SwapchainId, Option<TextureViewId>)> {
        self.commands
            .iter()
            .filter_map(|command| command.swapchain())
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
impl HaveDescriptor for CommandBufferDescriptor {
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
        StateType::Stateless
    }
    fn needs_update(&self, _other: &Self::D) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Command to be written on [CommandBufferDescriptor][CommandBufferDescriptor] descriptor.
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
    pub fn swapchain(&self) -> Option<(SwapchainId, Option<TextureViewId>)> {
        if let Command::RenderPass {
            label: _,
            depth_stencil,
            color_attachments,
            commands: _,
        } = self
        {
            color_attachments.iter().find_map(|attachment| {
                attachment
                    .swapchain()
                    .map(|swapchain| (swapchain, depth_stencil.clone()))
            })
        } else {
            None
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
/// View of the object where colors are going to be written.
/// Required for the [RenderPassColorAttachment][RenderPassColorAttachment] object.
pub enum ColorView {
    TextureView(TextureViewId),
    Swapchain(SwapchainId),
}
impl ColorView {
    pub fn swapchain(&self) -> Option<SwapchainId> {
        match self {
            Self::Swapchain(id) => Some(*id),
            _ => None,
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
/// Parameters for a render pass attachment of a [Command::RenderPass][Command] object.
pub struct RenderPassColorAttachment {
    pub view: ColorView,
    pub resolve_target: Option<TextureViewId>,
    pub ops: crate::wgpu::Operations<crate::wgpu::Color>,
}
impl RenderPassColorAttachment {
    pub fn swapchain(&self) -> Option<SwapchainId> {
        self.view.swapchain()
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
/// Builder for commands to be written in a [ComputePass][crate::wgpu::ComputePass] object.
/// Never used nor implemented.
pub enum ComputeCommand {}
impl HaveDependencies for ComputeCommand {
    fn dependencies(&self) -> Vec<EntityId> {
        Vec::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Enumeration containing all the possible type of ranges.
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
/// Builder for commands to be written in a [RenderPass][crate::wgpu::RenderPass] object.
pub enum RenderCommand {
    SetPipeline {
        pipeline: RenderPipelineId,
    },
    SetPushConstants {
        stages: crate::wgpu::ShaderStage,
        offset: u32,
        data: Vec<u8>,
    },
    SetBindGroup {
        index: u32,
        bind_group: BindGroupId,
        offsets: Vec<crate::wgpu::DynamicOffset>,
    },
    SetVertexBuffer {
        slot: u32,
        buffer: BufferId,
        slice: Slice<crate::wgpu::BufferAddress>,
    },
    SetIndexBuffer {
        index_format: crate::wgpu::IndexFormat,
        buffer: BufferId,
        slice: Slice<crate::wgpu::BufferAddress>,
    },
    Draw {
        vertices: std::ops::Range<u32>,
        instances: std::ops::Range<u32>,
    },
    DrawIndexed {
        indices: std::ops::Range<u32>,
        base_vertex: i32,
        instances: std::ops::Range<u32>,
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
/// Buffer to buffer copy command.
pub struct BufferToBufferCopy {
    pub src_buffer: BufferId,
    pub src_offset: crate::wgpu::BufferAddress,
    pub dst_buffer: BufferId,
    pub dst_offset: crate::wgpu::BufferAddress,
    pub size: crate::wgpu::BufferAddress,
}
impl HaveDependencies for BufferToBufferCopy {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![
            self.src_buffer.id_ref().clone(),
            self.dst_buffer.id_ref().clone(),
        ]
    }
}

#[derive(Debug, Clone)]
/// Buffer to Texture copy command.
pub struct BufferToTextureCopy {
    pub src_buffer: BufferId,
    pub src_layout: crate::wgpu::ImageDataLayout,
    pub dst_texture: TextureId,
    pub dst_mip_level: u32,
    pub dst_origin: crate::wgpu::Origin3d,
    pub copy_size: crate::wgpu::Extent3d,
}
impl PartialEq for BufferToTextureCopy {
    fn eq(&self, other: &Self) -> bool {
        if self.src_buffer != other.src_buffer {
            return false;
        }
        if self.src_layout.offset != other.src_layout.offset {
            return false;
        }
        if self.src_layout.bytes_per_row != other.src_layout.bytes_per_row {
            return false;
        }
        if self.src_layout.rows_per_image != other.src_layout.rows_per_image {
            return false;
        }
        if self.dst_texture != other.dst_texture {
            return false;
        }
        if self.dst_mip_level != other.dst_mip_level {
            return false;
        }
        if self.dst_origin != other.dst_origin {
            return false;
        }
        if self.copy_size != other.copy_size {
            return false;
        }
        true
    }
}
impl HaveDependencies for BufferToTextureCopy {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![
            self.src_buffer.id_ref().clone(),
            self.dst_texture.id_ref().clone(),
        ]
    }
}

#[derive(Debug, Clone)]
/// Texture to buffer copy command.
pub struct TextureToBufferCopy {
    pub src_texture: TextureId,
    pub src_mip_level: u32,
    pub src_origin: crate::wgpu::Origin3d,
    pub dst_buffer: BufferId,
    pub dst_layout: crate::wgpu::ImageDataLayout,
    pub copy_size: crate::wgpu::Extent3d,
}
impl PartialEq for TextureToBufferCopy {
    fn eq(&self, other: &Self) -> bool {
        if self.src_texture != other.src_texture {
            return false;
        }
        if self.src_mip_level != other.src_mip_level {
            return false;
        }
        if self.src_origin != other.src_origin {
            return false;
        }
        if self.dst_buffer != other.dst_buffer {
            return false;
        }
        if self.dst_layout.offset != other.dst_layout.offset {
            return false;
        }
        if self.dst_layout.bytes_per_row != other.dst_layout.bytes_per_row {
            return false;
        }
        if self.dst_layout.rows_per_image != other.dst_layout.rows_per_image {
            return false;
        }
        if self.copy_size != other.copy_size {
            return false;
        }
        true
    }
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
/// Texture to texture copy command.
pub struct TextureToTextureCopy {
    pub src_texture: TextureId,
    pub src_mip_level: u32,
    pub src_origin: crate::wgpu::Origin3d,
    pub dst_texture: TextureId,
    pub dst_mip_level: u32,
    pub dst_origin: crate::wgpu::Origin3d,
    pub copy_size: crate::wgpu::Extent3d,
}
impl HaveDependencies for TextureToTextureCopy {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![
            self.src_texture.id_ref().clone(),
            self.dst_texture.id_ref().clone(),
        ]
    }
}

#[derive(Clone, PartialEq)]
/// Host to buffer copy command.
pub struct BufferWrite {
    pub buffer: BufferId,
    pub offset: crate::wgpu::BufferAddress,
    pub data: Vec<u8>,
}
impl std::fmt::Debug for BufferWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("buffer", &self.buffer)
            .field("offset", &self.offset)
            .field("data(len)", &self.data.len())
            .finish()
    }
}

#[derive(Clone)]
/// Host to texture copy command.
pub struct TextureWrite {
    pub texture: TextureId,
    pub mip_level: u32,
    pub origin: crate::wgpu::Origin3d,
    pub data: Vec<u8>,
    pub layout: crate::wgpu::ImageDataLayout,
    pub size: crate::wgpu::Extent3d,
}
impl std::fmt::Debug for TextureWrite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("texture", &self.texture)
            .field("mip_level", &self.mip_level)
            .field("origin", &self.origin)
            .field("data(len)", &self.data.len())
            .field("layout", &self.layout)
            .field("size", &self.size)
            .finish()
    }
}
impl PartialEq for TextureWrite {
    fn eq(&self, other: &Self) -> bool {
        if self.texture != other.texture {
            return false;
        }
        if self.mip_level != other.mip_level {
            return false;
        }
        if self.origin != other.origin {
            return false;
        }
        if self.data != other.data {
            return false;
        }
        if self.layout.offset != other.layout.offset {
            return false;
        }
        if self.layout.bytes_per_row != other.layout.bytes_per_row {
            return false;
        }
        if self.layout.rows_per_image != other.layout.rows_per_image {
            return false;
        }
        if self.size != other.size {
            return false;
        }
        true
    }
}
