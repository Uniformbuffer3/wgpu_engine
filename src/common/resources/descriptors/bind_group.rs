//! Bind group related structures and enumerations.

use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::{BindGroupLayoutId, BufferId, DeviceId, SamplerId, TextureViewId};

#[derive(Debug, Clone, PartialEq)]
/// Buffer binding for the [BindingResource][BindingResource] object.
pub struct BufferBinding {
    pub buffer: BufferId, //Arc<crate::wgpu::Buffer>
    pub offset: crate::wgpu::BufferAddress,
    pub size: Option<crate::wgpu::BufferSize>,
}
impl HaveDependencies for BufferBinding {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![self.buffer.id_ref().clone()]
    }
}
impl HaveDescriptor for BufferBinding {
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
        StateType::Statefull
    }
    fn needs_update(&self, _other: &Self::D) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Binding resource for the [BindGroupEntry][BindGroupEntry] object.
pub enum BindingResource {
    Buffer(BufferBinding),
    BufferArray(Vec<BufferBinding>),
    Sampler(SamplerId),                   //Arc<crate::wgpu::Sampler>
    TextureView(TextureViewId),           //Arc<crate::wgpu::TextureView>
    TextureViewArray(Vec<TextureViewId>), //Arc<crate::wgpu::TextureView>
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
            Self::TextureView(id) => vec![id.id_ref().clone()], //Arc<crate::wgpu::TextureView>
            Self::TextureViewArray(ids) => ids.iter().map(|id| id.id_ref().clone()).collect(), //Arc<crate::wgpu::TextureView>
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Entry for the [BindGroupDescriptor][BindGroupDescriptor]
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
/**
Descriptor of [BindGroupHandle][crate::common::resources::handles::BindGroupHandle]
*/
pub struct BindGroupDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub layout: BindGroupLayoutId, //Arc<crate::wgpu::BindGroupLayout>
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
impl HaveDescriptor for BindGroupDescriptor {
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
