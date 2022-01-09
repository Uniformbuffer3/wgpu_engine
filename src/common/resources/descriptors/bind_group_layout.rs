//! Bind group layout related structures and enumerations.

use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::DeviceId;

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [BindGroupLayoutHandle][crate::common::resources::handles::BindGroupLayoutHandle]
*/
pub struct BindGroupLayoutDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub entries: Vec<crate::wgpu::BindGroupLayoutEntry>,
}
impl HaveDependencies for BindGroupLayoutDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![*self.device.id_ref()]
    }
}
impl HaveDescriptor for BindGroupLayoutDescriptor {
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
