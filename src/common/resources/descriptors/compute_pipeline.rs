use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::{DeviceId, PipelineLayoutId, ShaderModuleId};

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [ComputePipelineHandle][crate::common::resources::handles::ComputePipelineHandle]
*/
pub struct ComputePipelineDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub layout: Option<PipelineLayoutId>, //Arc<crate::wgpu::PipelineLayout>
    pub module: ShaderModuleId,           //Arc<crate::wgpu::ShaderModule>
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
impl HaveDescriptor for ComputePipelineDescriptor {
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
