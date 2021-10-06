use std::convert::TryInto;
use std::sync::{Arc, Mutex, MutexGuard};

pub type InstanceHandle = Arc<crate::wgpu::Instance>;
pub type DeviceHandle = Arc<(crate::wgpu::Adapter, crate::wgpu::Device, crate::wgpu::Queue)>;
pub type SwapchainHandle = Arc<Swapchain>;

pub type BufferHandle = Arc<crate::wgpu::Buffer>;
pub type TextureHandle = Arc<crate::wgpu::Texture>;
pub type TextureViewHandle = Arc<crate::wgpu::TextureView>;
pub type SamplerHandle = Arc<crate::wgpu::Sampler>;
pub type ShaderModuleHandle = Arc<crate::wgpu::ShaderModule>;

pub type BindGroupLayoutHandle = Arc<crate::wgpu::BindGroupLayout>;
pub type BindGroupHandle = Arc<crate::wgpu::BindGroup>;

pub type PipelineLayoutHandle = Arc<crate::wgpu::PipelineLayout>;
pub type RenderPipelineHandle = Arc<crate::wgpu::RenderPipeline>;
pub type ComputePipelineHandle = Arc<crate::wgpu::ComputePipeline>;
pub type CommandBufferHandle = Arc<crate::wgpu::CommandBuffer>;

#[derive(Debug, Clone)]
pub enum ResourceHandle {
    Instance(InstanceHandle),
    Device(DeviceHandle),
    Swapchain(SwapchainHandle),

    Buffer(BufferHandle),
    Texture(TextureHandle),
    TextureView(TextureViewHandle),
    Sampler(SamplerHandle),
    ShaderModule(ShaderModuleHandle),

    BindGroupLayout(BindGroupLayoutHandle),
    BindGroup(BindGroupHandle),

    PipelineLayout(PipelineLayoutHandle),
    RenderPipeline(RenderPipelineHandle),
    ComputePipeline(ComputePipelineHandle),
    CommandBuffer(CommandBufferHandle),
}
impl From<InstanceHandle> for ResourceHandle {
    fn from(resource: InstanceHandle) -> Self {
        Self::Instance(resource)
    }
}
impl From<DeviceHandle> for ResourceHandle {
    fn from(resource: DeviceHandle) -> Self {
        Self::Device(resource)
    }
}
impl From<SwapchainHandle> for ResourceHandle {
    fn from(resource: SwapchainHandle) -> Self {
        Self::Swapchain(resource)
    }
}

impl TryInto<Arc<crate::wgpu::Buffer>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<crate::wgpu::Buffer>, Self::Error> {
        if let ResourceHandle::Buffer(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<crate::wgpu::Buffer>> for ResourceHandle {
    fn from(resource: Arc<crate::wgpu::Buffer>) -> Self {
        Self::Buffer(resource)
    }
}

impl TryInto<Arc<crate::wgpu::Texture>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<crate::wgpu::Texture>, Self::Error> {
        if let ResourceHandle::Texture(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<crate::wgpu::Texture>> for ResourceHandle {
    fn from(resource: Arc<crate::wgpu::Texture>) -> Self {
        Self::Texture(resource)
    }
}

impl TryInto<Arc<crate::wgpu::TextureView>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<crate::wgpu::TextureView>, Self::Error> {
        if let ResourceHandle::TextureView(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<crate::wgpu::TextureView>> for ResourceHandle {
    fn from(resource: Arc<crate::wgpu::TextureView>) -> Self {
        Self::TextureView(resource)
    }
}

impl TryInto<Arc<crate::wgpu::Sampler>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<crate::wgpu::Sampler>, Self::Error> {
        if let ResourceHandle::Sampler(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<crate::wgpu::Sampler>> for ResourceHandle {
    fn from(resource: Arc<crate::wgpu::Sampler>) -> Self {
        Self::Sampler(resource)
    }
}

impl TryInto<Arc<crate::wgpu::ShaderModule>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<crate::wgpu::ShaderModule>, Self::Error> {
        if let ResourceHandle::ShaderModule(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<crate::wgpu::ShaderModule>> for ResourceHandle {
    fn from(resource: Arc<crate::wgpu::ShaderModule>) -> Self {
        Self::ShaderModule(resource)
    }
}

impl TryInto<Arc<crate::wgpu::BindGroup>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<crate::wgpu::BindGroup>, Self::Error> {
        if let ResourceHandle::BindGroup(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<crate::wgpu::BindGroup>> for ResourceHandle {
    fn from(resource: Arc<crate::wgpu::BindGroup>) -> Self {
        Self::BindGroup(resource)
    }
}

impl TryInto<Arc<crate::wgpu::BindGroupLayout>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<crate::wgpu::BindGroupLayout>, Self::Error> {
        if let ResourceHandle::BindGroupLayout(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<crate::wgpu::BindGroupLayout>> for ResourceHandle {
    fn from(resource: Arc<crate::wgpu::BindGroupLayout>) -> Self {
        Self::BindGroupLayout(resource)
    }
}

impl TryInto<Arc<crate::wgpu::PipelineLayout>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<crate::wgpu::PipelineLayout>, Self::Error> {
        if let ResourceHandle::PipelineLayout(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<crate::wgpu::PipelineLayout>> for ResourceHandle {
    fn from(resource: Arc<crate::wgpu::PipelineLayout>) -> Self {
        Self::PipelineLayout(resource)
    }
}

impl TryInto<Arc<crate::wgpu::RenderPipeline>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<crate::wgpu::RenderPipeline>, Self::Error> {
        if let ResourceHandle::RenderPipeline(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<crate::wgpu::RenderPipeline>> for ResourceHandle {
    fn from(resource: Arc<crate::wgpu::RenderPipeline>) -> Self {
        Self::RenderPipeline(resource)
    }
}

impl TryInto<Arc<crate::wgpu::ComputePipeline>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<crate::wgpu::ComputePipeline>, Self::Error> {
        if let ResourceHandle::ComputePipeline(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<crate::wgpu::ComputePipeline>> for ResourceHandle {
    fn from(resource: Arc<crate::wgpu::ComputePipeline>) -> Self {
        Self::ComputePipeline(resource)
    }
}
impl From<Arc<crate::wgpu::CommandBuffer>> for ResourceHandle {
    fn from(resource: Arc<crate::wgpu::CommandBuffer>) -> Self {
        Self::CommandBuffer(resource)
    }
}

#[derive(Debug, Clone)]
pub struct Swapchain {
    swapchain_descriptor: crate::wgpu::SwapChainDescriptor,
    swapchain: Arc<crate::wgpu::SwapChain>,

    current_frame: Arc<Mutex<Option<crate::wgpu::SwapChainFrame>>>,
}

impl Swapchain {
    pub fn new(
        device: &Arc<(crate::wgpu::Adapter, crate::wgpu::Device, crate::wgpu::Queue)>,
        surface: Arc<crate::wgpu::Surface>,
        width: u32,
        height: u32,
    ) -> Option<Self> {
        // Create swapchain
        let swapchain_descriptor = crate::wgpu::SwapChainDescriptor {
            usage: crate::wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: device.0.get_swap_chain_preferred_format(&surface).unwrap(),
            present_mode: crate::wgpu::PresentMode::Mailbox,
            width,
            height,
        };
        let swapchain = Arc::new(device.1.create_swap_chain(&surface, &swapchain_descriptor));

        let current_frame = match swapchain.get_current_frame() {
            Ok(current_frame) => Arc::new(Mutex::new(Some(current_frame))),
            Err(_) => return None,
        };

        Some(Self {
            swapchain_descriptor,
            swapchain,
            current_frame,
        })
    }

    pub fn prepare_frame(&self) {
        let mut current_frame = self.current_frame.lock().unwrap();

        if current_frame.is_none() {
            *current_frame = match self.swapchain.get_current_frame() {
                Ok(current_frame) => Some(current_frame),
                Err(err) => panic!("{:#?}", err),
            };
        }
    }

    pub fn present(&self) {
        let mut current_frame = self.current_frame.lock().unwrap();
        current_frame.take();
    }

    pub fn current_frame(&self) -> MutexGuard<Option<crate::wgpu::SwapChainFrame>> {
        self.current_frame.lock().unwrap()
    }
}
