use std::fmt::Debug;

use imlet_engine::types::{
    computation::Model,
    geometry::{BoundingBox, Line, Mesh},
};
use num_traits::Float;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{material::Material, state::State};

pub struct Viewer<T: Float + Debug + Send + Sync> {
    model: Option<Model<T>>,
    bounds: Option<BoundingBox<T>>,
    meshes: Vec<Mesh<T>>,
    lines: Vec<Line<T>>,
    settings: ViewerSettings,
}

impl<T: Float + Debug + Send + Sync> Viewer<T> {
    pub fn new() -> Self {
        Self {
            model: None,
            bounds: None,
            meshes: Vec::new(),
            lines: Vec::new(),
            settings: ViewerSettings::new(),
        }
    }

    pub fn with_model(model: Model<T>) -> Self {
        Self {
            model: Some(model),
            bounds: None,
            meshes: Vec::new(),
            lines: Vec::new(),
            settings: ViewerSettings::new(),
        }
    }

    pub fn with_settings(settings: ViewerSettings) -> Self {
        Self {
            model: None,
            bounds: None,
            meshes: Vec::new(),
            lines: Vec::new(),
            settings: settings,
        }
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
    async fn run(&self) {
        let window_icon = None;
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("ImLET viewer")
            .with_window_icon(window_icon)
            .build(&event_loop)
            .unwrap();

        let bounds = if self.bounds.is_some() {
            self.bounds.unwrap()
        } else {
            BoundingBox::from_meshes(&self.meshes)
        };
        let mut state = State::new(
            window,
            BoundingBox::new(bounds.min.to_f32(), bounds.max.to_f32()),
            &self.settings.mesh_material,
        )
        .await;

        state.set_meshes(&self.meshes, false);

        event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == state.window().id() => {
                    if !state.input(event) {
                        match event {
                            WindowEvent::CloseRequested
                            | WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    },
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            WindowEvent::Resized(physical_size) => {
                                state.resize(*physical_size);
                            }
                            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                // new_inner_size is &mut so w have to dereference it twice
                                state.resize(**new_inner_size);
                            }
                            _ => {}
                        }
                    }
                }
                Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                    state.update();
                    match state.render() {
                        Ok(_) => {}
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            state.resize(state.size())
                        }
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                        Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                    }
                }
                Event::MainEventsCleared => {
                    state.window().request_redraw();
                }
                _ => {}
            }
        });
    }
}

pub struct ViewerSettings {
    mesh_material: Material,
    show_bounds: bool,
    show_edges: bool,
}

impl ViewerSettings {
    pub fn new() -> Self {
        Self {
            mesh_material: Material::Normal,
            show_bounds: true,
            show_edges: false,
        }
    }
}
