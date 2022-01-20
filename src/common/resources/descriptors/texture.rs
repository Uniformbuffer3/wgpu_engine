//! Texture related structures and enumerations.

use crate::common::resources::descriptors::{HaveDependencies, HaveDescriptor, StateType};
use crate::entity_manager::EntityId;
use crate::resources::DeviceId;

#[derive(Debug, Clone, PartialEq)]
/// Possible sources of a texture.
pub enum TextureSource {
    #[cfg(feature = "wgpu_custom")]
    DmaBuf {
        fd: std::os::unix::io::RawFd,
        drm_properties: Option<crate::wgpu::DrmFormatImageProperties>,
        offset: u64,
    },
    OpaqueFd {
        fd: std::os::unix::io::RawFd,
        offset: u64,
    },
    //Ptr(std::sync::Arc<std::ffi::c_void>),
    Local,
}

#[derive(Debug, Clone, PartialEq)]
/**
Descriptor of [TextureHandle][crate::common::resources::handles::TextureHandle]
*/
pub struct TextureDescriptor {
    pub label: String,
    pub device: DeviceId,
    pub source: TextureSource,
    pub usage: crate::wgpu::TextureUsage,
    pub size: crate::wgpu::Extent3d,
    pub format: crate::wgpu::TextureFormat,
    pub dimension: crate::wgpu::TextureDimension,
    pub mip_level_count: u32,
    pub sample_count: u32,
}
impl HaveDependencies for TextureDescriptor {
    fn dependencies(&self) -> Vec<EntityId> {
        vec![*self.device.id_ref()]
    }
}
impl HaveDescriptor for TextureDescriptor {
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
