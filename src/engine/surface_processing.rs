use super::WGpuEngine;

use crate::engine::engine_task::EngineTask;
use std::sync::Arc;

impl WGpuEngine {
    pub fn create_surface(
        &mut self,
        external_id: usize,
        label: String,
        surface: Arc<crate::wgpu::Surface>,
        width: u32,
        height: u32,
    ) {
        assert!(self
            .task_manager
            .task_handle_cast_mut(&self.engine_task, |engine_task: &mut EngineTask| {
                engine_task.create_swapchain(external_id, label, surface, width, height);
            },)
            .is_some());
    }

    pub fn resize_surface(&mut self, external_id: usize, width: u32, height: u32) {
        assert!(self
            .task_manager
            .task_handle_cast_mut(&self.engine_task, |engine_task: &mut EngineTask| {
                engine_task.resize_swapchain(external_id, width, height);
            },)
            .is_some())
    }

    pub fn remove_surface(&mut self, external_id: usize) {
        assert!(self
            .task_manager
            .task_handle_cast_mut(&self.engine_task, |engine_task: &mut EngineTask| {
                engine_task.remove_swapchain(external_id);
            },)
            .is_some());
    }

    pub fn surface_count(&self) -> usize {
        self.task_manager
            .task_handle_cast_ref(&self.engine_task, |engine_task: &EngineTask| {
                engine_task.swapchains().count()
            })
            .unwrap()
    }
}
