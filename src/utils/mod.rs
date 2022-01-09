//! Utility functions and structures.

pub mod buffer_manager;
pub use buffer_manager::*;

use crate::common::tasks::TaskTrait;
use crate::TaskId;
use crate::UpdateContext;

use crate::WGpuEngine;
use pal::definitions::*;

/// Allow to quickly initialize the engine and the event loop to run a single task.
pub fn quick_run<T: TaskTrait, C: Fn(TaskId, &tokio::runtime::Handle, &mut UpdateContext) -> T>(
    surface_count: usize,
    features: crate::wgpu::Features,
    limits: crate::wgpu::Limits,
    task_callback: C,
    mut loop_callback: impl FnMut(&mut T),
) {
    let mut wgpu_engine = WGpuEngine::new((features.clone(), limits.clone()))
        .expect("Failed to initialize the engine: {}");

    let mut platform = pal::Platform::new(vec![Box::new(wgpu_engine.wgpu_context())]);
    (0..surface_count).for_each(|_| {
        platform.requests(vec![Request::Surface {
            request: SurfaceRequest::Create(None),
        }]);
    });

    let task = wgpu_engine
        .create_task("Task".into(), (features, limits), task_callback)
        .unwrap();

    use std::os::unix::io::AsRawFd;
    let mut event_loop = calloop::EventLoop::try_new().unwrap();
    let interest = calloop::Interest {
        readable: true,
        writable: false,
    };
    event_loop
        .handle()
        .insert_source(
            calloop::generic::Generic::new(platform.as_raw_fd(), interest, calloop::Mode::Edge),
            move |_event, _metadata, _data: &mut ()| Ok(calloop::PostAction::Continue),
        )
        .unwrap();

    'main_loop: loop {
        match event_loop.dispatch(None, &mut ()) {
            Ok(_) => (),
            Err(_) => (),
        }
        for event in platform.events() {
            match event {
                pal::Event::Surface { time: _, id, event } => match &event {
                    pal::SurfaceEvent::Added(surface_info) => {
                        if let Surface::WGpu(surface) = &surface_info.surface {
                            wgpu_engine.create_surface(
                                id.into(),
                                String::from("MainSurface"),
                                surface.clone(),
                                surface_info.size.width,
                                surface_info.size.height,
                            );
                        } else {
                            panic!("It is not of WGpu type");
                        }
                    }
                    pal::SurfaceEvent::Resized(size) => {
                        wgpu_engine.resize_surface(id.into(), size.width, size.height);
                    }
                    pal::SurfaceEvent::Removed => {
                        wgpu_engine.destroy_surface(id.into());
                        if wgpu_engine.surface_count() == 0 {
                            break 'main_loop;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        wgpu_engine.dispatch_tasks();
        wgpu_engine.task_handle_cast_mut(&task, |task| loop_callback(task));
    }
}
