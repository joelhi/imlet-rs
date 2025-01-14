use std::time::Instant;

use winit::{
    event::{DeviceEvent, ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use crate::types::geometry::{BoundingBox, Mesh};

use super::state::State;

/// Show a mesh object in an interactive window.
pub fn show_mesh(mesh: &Mesh<f32>, bounds: BoundingBox<f32>) {
    pollster::block_on(run_internal(mesh, bounds)).unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn run_internal(
    mesh: &Mesh<f32>,
    bounds: BoundingBox<f32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let window_icon = None;
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Imlet viewer")
        .with_window_icon(window_icon)
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(&window, mesh).await;

    state.write_mesh_buffers(&[mesh]);
    let lines = mesh.edges();
    state.write_line_buffers(&bounds.as_wireframe());

    let mut last_render_time = Instant::now();
    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta },
                    ..
                } => {
                    if state.mouse_pressed {
                        println!("Process mouse");
                        //state.camera_controller.process_mouse(delta.0, delta.1);
                    }
                }
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() && !state.input(event) => {
                    match event {
                        #[cfg(not(target_arch = "wasm32"))]
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state: ElementState::Pressed,
                                    physical_key: PhysicalKey::Code(KeyCode::Escape),
                                    ..
                                },
                            ..
                        } => control_flow.exit(),
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => {
                            state.window().request_redraw();
                            let now = Instant::now();
                            let dt = now - last_render_time;
                            last_render_time = now;
                            state.update();
                            match state.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if it's lost or outdated
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    state.resize(state.size())
                                }
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => control_flow.exit(),
                                // We're ignoring timeouts
                                Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .unwrap();

    Ok(())
}
