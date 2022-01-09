//! Buffer related structures and enumerations.

use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::DeviceId;

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [BufferHandle][crate::common::resources::handles::BufferHandle]
*/
pub struct BufferDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub size: crate::wgpu::BufferAddress,
    pub usage: crate::wgpu::BufferUsage,
}
impl HaveDependencies for BufferDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![*self.device.id_ref()]
    }
}
impl HaveDescriptor for BufferDescriptor {
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
