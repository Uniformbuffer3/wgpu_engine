pub mod buffer_manager;
pub use buffer_manager::*;

use crate::TaskId;
use crate::UpdateContext;
use crate::common::tasks::TaskTrait;

use std::collections::HashSet;
use crate::WGpuEngine;
use pal::definitions::*;

pub fn quick_run<T: TaskTrait,C: Fn(TaskId, &tokio::runtime::Handle, &mut UpdateContext) -> T>(
    surface_count: usize,
    features: crate::wgpu::Features,
    limits: crate::wgpu::Limits,
    task_callback: C
) {
    let mut wgpu_engine =
        WGpuEngine::new((features.clone(), limits.clone())).expect("Failed to initialize the engine: {}");

    let mut platform = pal::Platform::new(vec![Box::new(wgpu_engine.wgpu_context())]);
    (0..surface_count).for_each(|_|{
        platform.request(vec![Request::from(SurfaceRequest::Create(None))]);
    });

    let _task = wgpu_engine
        .create_task(
            "TriangleTask".into(),
            (features,limits),
            task_callback,
        )
        .unwrap();

    let mut surfaces = HashSet::new();

    'main_loop: loop {
        for event in platform.events() {
            match event {
                pal::Event::Surface(ref surface_event) => {
                    let surface_id = surface_event.id;
                    match &surface_event.event_type {
                        pal::SurfaceEventType::Added(surface_info) => {
                            if let Surface::WGpu(surface) = &surface_info.surface {
                                wgpu_engine.create_surface(
                                    surface_id.id() as usize,
                                    String::from("MainSurface"),
                                    surface.clone(),
                                    surface_info.size.width,
                                    surface_info.size.height,
                                );
                                surfaces.insert(surface_id.id() as usize);
                            } else {
                                panic!("It is not of WGpu type");
                            }
                        }
                        pal::SurfaceEventType::Resized(size) => {
                            wgpu_engine.resize_surface(
                                surface_id.id() as usize,
                                size.width,
                                size.height,
                            );
                        }
                        pal::SurfaceEventType::Removed => {
                            let id = surface_id.id() as usize;
                            wgpu_engine.remove_surface(id);
                            surfaces.remove(&id);
                            if surfaces.is_empty() {
                                break 'main_loop;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        wgpu_engine.dispatch_tasks();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

}
