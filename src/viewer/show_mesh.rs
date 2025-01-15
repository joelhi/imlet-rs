use std::time::Instant;

use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use crate::{
    types::geometry::{BoundingBox, Mesh},
    IMLET_VERSION,
};

use super::{state::State, DisplaySettings};

/// Show a mesh object in an interactive window.
///
/// # Arguments
/// * `mesh` - The geometry to show.
/// * `bounds` - Optional bounding box.
pub fn show_mesh(mesh: &Mesh<f32>, bounds: Option<BoundingBox<f32>>) {
    pollster::block_on(run_internal(mesh, bounds, &DisplaySettings::new())).unwrap()
}

/// Show a mesh object in an interactive window, with custom settings.
///
///
/// # Arguments
/// * `mesh` - The geometry to show.
/// * `bounds` - Optional bounding box.
/// * `display_settings` - The settings used when rendering the mesh.
pub fn show_mesh_with_settings(
    mesh: &Mesh<f32>,
    bounds: Option<BoundingBox<f32>>,
    display_settings: &DisplaySettings,
) {
    pollster::block_on(run_internal(mesh, bounds, display_settings)).unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
async fn run_internal(
    mesh: &Mesh<f32>,
    bounds: Option<BoundingBox<f32>>,
    display_settings: &DisplaySettings,
) -> Result<(), Box<dyn std::error::Error>> {
    let window_icon = None;
    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title(format!("Imlet viewer v{}", IMLET_VERSION))
        .with_window_icon(window_icon)
        .build(&event_loop)
        .unwrap();

    let mut state = State::new(&window, mesh, &display_settings.mesh_material).await;

    state.write_mesh_buffers(&[mesh]);

    if display_settings.show_mesh_edges {
        state.write_line_buffers(&mesh.edges());
    }

    if let Some(bounds) = bounds {
        if display_settings.show_bounds {
            state.write_line_buffers(&bounds.as_wireframe())
        }
    }

    let mut last_render_time = Instant::now();
    event_loop
        .run(move |event, control_flow| {
            match event {
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
                            let _ = now - last_render_time;
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
