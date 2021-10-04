use crate::common::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResourceEvent {
    SwapchainCreated(SwapchainId),
    SwapchainDestroyed(SwapchainId),
    SwapchainUpdated(SwapchainId),
}
