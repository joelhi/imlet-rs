use std::fmt::Debug;

use imlet_engine::types::{
        computation::Model,
        geometry::BoundingBox,
    };
use num_traits::Float;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{scene::{ModelData, Scene}, state::State};

pub struct Viewer {
}

impl Viewer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run<T: Float + Debug + Send + Sync + 'static>(model: Model<T>, bounds: BoundingBox<T>) {
        pollster::block_on(Viewer::run_internal(model, bounds));
    }

    #[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
    async fn run_internal<T: Float + Debug + Send + Sync + 'static>(
        model: Model<T>,
        bounds: BoundingBox<T>,
    ) {
        let window_icon = None;
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("ImLET viewer")
            .with_window_icon(window_icon)
            .build(&event_loop)
            .unwrap();

        let model_data = ModelData::new(model, bounds);

        let scene = Scene::new();

        let mut state = State::new(
            window,
            model_data,
            scene
        )
        .await;

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
                            WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::P),
                                        ..
                                    },
                                ..
                            } => { 
                                state.smooth_geometry(1, T::one());
                                state.update_scene();
                                state.write_geometry();
                            }
                            WindowEvent::KeyboardInput {
                                input:
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::G),
                                        ..
                                    },
                                ..
                            } => { 
                                state.compute_field(T::from(0.15).unwrap());
                                state.update_scene();
                                state.write_geometry();
                            }
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