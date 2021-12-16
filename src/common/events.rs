use crate::common::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceEvent {
    SwapchainCreated {
        external_id: usize,
        swapchain: SwapchainId,
    },
    SwapchainDestroyed(SwapchainId),
    SwapchainUpdated(SwapchainId),
}
