use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::DeviceId;

#[derive(Debug, Clone)]
/**
Descriptor of [SwapchainHandle][crate::common::resources::handles::SwapchainHandle]
*/
pub struct SwapchainDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub surface: std::sync::Arc<crate::wgpu::Surface>,
    pub usage: crate::wgpu::TextureUsage,
    pub format: crate::wgpu::TextureFormat,
    pub width: u32,
    pub height: u32,
    pub present_mode: crate::wgpu::PresentMode,
}
impl HaveDependencies for SwapchainDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![self.device.id_ref().clone()]
    }
}
impl HaveDescriptor for SwapchainDescriptor {
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

impl PartialEq for SwapchainDescriptor {
    fn eq(&self, other: &Self) -> bool {
        if self.label != other.label {
            return false;
        }
        if self.device != other.device {
            return false;
        }
        if !std::sync::Arc::ptr_eq(&self.surface, &other.surface) {
            return false;
        }
        if self.usage != other.usage {
            return false;
        }
        if self.format != other.format {
            return false;
        }
        if self.width != other.width {
            return false;
        }
        if self.height != other.height {
            return false;
        }
        if self.present_mode != other.present_mode {
            return false;
        }
        true
    }
}
