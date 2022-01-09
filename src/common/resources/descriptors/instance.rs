//! Instance related structures and enumerations.

use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [InstanceHandle][crate::common::resources::handles::InstanceHandle]
*/
pub struct InstanceDescriptor {
    pub label: String,
    pub backend: crate::wgpu::BackendBit,
}
impl HaveDependencies for InstanceDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![]
    }
}
impl HaveDescriptor for InstanceDescriptor {
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
