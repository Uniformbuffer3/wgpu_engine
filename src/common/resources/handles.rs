use std::convert::TryInto;
use std::sync::{Arc, Mutex, MutexGuard};

pub type InstanceHandle = Arc<wgpu::Instance>;
pub type DeviceHandle = Arc<(wgpu::Adapter, wgpu::Device, wgpu::Queue)>;
pub type SwapchainHandle = Arc<Swapchain>;

pub type BufferHandle = Arc<wgpu::Buffer>;
pub type TextureHandle = Arc<wgpu::Texture>;
pub type TextureViewHandle = Arc<wgpu::TextureView>;
pub type SamplerHandle = Arc<wgpu::Sampler>;
pub type ShaderModuleHandle = Arc<wgpu::ShaderModule>;

pub type BindGroupLayoutHandle = Arc<wgpu::BindGroupLayout>;
pub type BindGroupHandle = Arc<wgpu::BindGroup>;

pub type PipelineLayoutHandle = Arc<wgpu::PipelineLayout>;
pub type RenderPipelineHandle = Arc<wgpu::RenderPipeline>;
pub type ComputePipelineHandle = Arc<wgpu::ComputePipeline>;
pub type CommandBufferHandle = Arc<wgpu::CommandBuffer>;

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

impl TryInto<Arc<wgpu::Buffer>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<wgpu::Buffer>, Self::Error> {
        if let ResourceHandle::Buffer(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<wgpu::Buffer>> for ResourceHandle {
    fn from(resource: Arc<wgpu::Buffer>) -> Self {
        Self::Buffer(resource)
    }
}

impl TryInto<Arc<wgpu::Texture>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<wgpu::Texture>, Self::Error> {
        if let ResourceHandle::Texture(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<wgpu::Texture>> for ResourceHandle {
    fn from(resource: Arc<wgpu::Texture>) -> Self {
        Self::Texture(resource)
    }
}

impl TryInto<Arc<wgpu::TextureView>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<wgpu::TextureView>, Self::Error> {
        if let ResourceHandle::TextureView(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<wgpu::TextureView>> for ResourceHandle {
    fn from(resource: Arc<wgpu::TextureView>) -> Self {
        Self::TextureView(resource)
    }
}

impl TryInto<Arc<wgpu::Sampler>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<wgpu::Sampler>, Self::Error> {
        if let ResourceHandle::Sampler(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<wgpu::Sampler>> for ResourceHandle {
    fn from(resource: Arc<wgpu::Sampler>) -> Self {
        Self::Sampler(resource)
    }
}

impl TryInto<Arc<wgpu::ShaderModule>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<wgpu::ShaderModule>, Self::Error> {
        if let ResourceHandle::ShaderModule(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<wgpu::ShaderModule>> for ResourceHandle {
    fn from(resource: Arc<wgpu::ShaderModule>) -> Self {
        Self::ShaderModule(resource)
    }
}

impl TryInto<Arc<wgpu::BindGroup>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<wgpu::BindGroup>, Self::Error> {
        if let ResourceHandle::BindGroup(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<wgpu::BindGroup>> for ResourceHandle {
    fn from(resource: Arc<wgpu::BindGroup>) -> Self {
        Self::BindGroup(resource)
    }
}

impl TryInto<Arc<wgpu::BindGroupLayout>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<wgpu::BindGroupLayout>, Self::Error> {
        if let ResourceHandle::BindGroupLayout(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<wgpu::BindGroupLayout>> for ResourceHandle {
    fn from(resource: Arc<wgpu::BindGroupLayout>) -> Self {
        Self::BindGroupLayout(resource)
    }
}

impl TryInto<Arc<wgpu::PipelineLayout>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<wgpu::PipelineLayout>, Self::Error> {
        if let ResourceHandle::PipelineLayout(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<wgpu::PipelineLayout>> for ResourceHandle {
    fn from(resource: Arc<wgpu::PipelineLayout>) -> Self {
        Self::PipelineLayout(resource)
    }
}

impl TryInto<Arc<wgpu::RenderPipeline>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<wgpu::RenderPipeline>, Self::Error> {
        if let ResourceHandle::RenderPipeline(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<wgpu::RenderPipeline>> for ResourceHandle {
    fn from(resource: Arc<wgpu::RenderPipeline>) -> Self {
        Self::RenderPipeline(resource)
    }
}

impl TryInto<Arc<wgpu::ComputePipeline>> for ResourceHandle {
    type Error = Self;
    fn try_into(self) -> Result<Arc<wgpu::ComputePipeline>, Self::Error> {
        if let ResourceHandle::ComputePipeline(handle) = self {
            Ok(handle)
        } else {
            Err(self)
        }
    }
}
impl From<Arc<wgpu::ComputePipeline>> for ResourceHandle {
    fn from(resource: Arc<wgpu::ComputePipeline>) -> Self {
        Self::ComputePipeline(resource)
    }
}
impl From<Arc<wgpu::CommandBuffer>> for ResourceHandle {
    fn from(resource: Arc<wgpu::CommandBuffer>) -> Self {
        Self::CommandBuffer(resource)
    }
}

#[derive(Debug, Clone)]
pub struct Swapchain {
    swapchain_descriptor: wgpu::SwapChainDescriptor,
    swapchain: Arc<wgpu::SwapChain>,

    current_frame: Arc<Mutex<Option<wgpu::SwapChainFrame>>>,
}

impl Swapchain {
    pub fn new(
        device: &Arc<(wgpu::Adapter, wgpu::Device, wgpu::Queue)>,
        surface: Arc<wgpu::Surface>,
        width: u32,
        height: u32,
    ) -> Option<Self> {
        // Create swapchain
        let swapchain_descriptor = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: device.0.get_swap_chain_preferred_format(&surface).unwrap(),
            present_mode: wgpu::PresentMode::Mailbox,
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

    pub fn current_frame(&self) -> MutexGuard<Option<wgpu::SwapChainFrame>> {
        self.current_frame.lock().unwrap()
    }
}
