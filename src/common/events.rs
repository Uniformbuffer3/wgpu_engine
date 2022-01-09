//! Resource events related structures and enumerations.

use crate::common::*;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Resource event shared among tasks.
pub enum ResourceEvent {
    SwapchainCreated {
        external_id: usize,
        swapchain: SwapchainId,
    },
    SwapchainDestroyed(SwapchainId),
    SwapchainUpdated(SwapchainId),
}
