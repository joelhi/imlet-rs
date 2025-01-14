#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly, so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    view_proj: [[f32; 4]; 4],
    camera_location: [f32; 3],
    _padding: u32,
}

impl CameraUniform {
    pub fn new(eye: cgmath::Point3<f32>) -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
            camera_location: eye.into(),
            _padding: 0,
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
        self.camera_location = camera.eye.into();
    }
}

use cgmath::Point3;
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseScrollDelta},
    keyboard::KeyCode,
};

pub struct CameraController {
    pub speed: f32,
    pub is_up_pressed: bool,
    pub is_down_pressed: bool,
    pub is_forward_pressed: bool,
    pub is_backward_pressed: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub is_reset: bool,
    pub is_scroll: bool,
    pub scroll_speed: f32,
    pub is_orbit: bool,
    pub orbit_horizontal: f32,
    pub orbit_vertical: f32,
    pub default_position: Point3<f32>,
    pub default_target: Point3<f32>,
}

impl CameraController {
    pub fn new(speed: f32, default_position: Point3<f32>, default_target: Point3<f32>) -> Self {
        Self {
            speed,
            is_up_pressed: false,
            is_down_pressed: false,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_reset: false,
            is_scroll: false,
            scroll_speed: 0.,
            is_orbit: false,
            orbit_horizontal: 0.,
            orbit_vertical: 0.,
            default_position: default_position,
            default_target: default_target,
        }
    }

    pub fn process_keyboard(&mut self, key: KeyCode, state: ElementState) -> bool {
        match key {
            KeyCode::KeyW | KeyCode::ArrowUp => {
                self.is_up_pressed = state == ElementState::Pressed;
                true
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {
                self.is_down_pressed = state == ElementState::Pressed;
                true
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {
                self.is_left_pressed = state == ElementState::Pressed;
                true
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {
                self.is_right_pressed = state == ElementState::Pressed;
                true
            }
            KeyCode::Space => {
                self.is_reset = state == ElementState::Pressed;
                true
            }
            KeyCode::KeyQ => {
                self.is_forward_pressed = state == ElementState::Pressed;
                true
            }
            KeyCode::KeyE => {
                self.is_backward_pressed = state == ElementState::Pressed;
                true
            }
            _ => false,
        }
    }

    pub fn process_mouse(&mut self, mouse_dx: f64, mouse_dy: f64, is_mouse_pressed: bool) {
        if is_mouse_pressed {
            self.orbit_horizontal += mouse_dx as f32;
            self.orbit_vertical += mouse_dy as f32;
            self.is_orbit = true;
        } else {
            self.is_orbit = false;
        }
    }

    pub fn process_scroll(&mut self, delta: &MouseScrollDelta) {
        let scroll = match delta {
            // I'm assuming a line is about 100 pixels
            MouseScrollDelta::LineDelta(_, scroll) => -scroll * 0.5,
            MouseScrollDelta::PixelDelta(PhysicalPosition { y: scroll, .. }) => -*scroll as f32,
        };
        if scroll != 0. {
            self.is_scroll = true;
            self.scroll_speed = scroll;
        } else {
            self.is_scroll = false;
            self.scroll_speed = 0.;
        }
    }

    pub fn update_camera(&mut self, camera: &mut Camera) {
        use cgmath::InnerSpace;
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }
        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }

        if self.is_up_pressed {
            camera.eye =
                camera.target - (forward + camera.up * self.speed).normalize() * forward_mag;
        }

        if self.is_down_pressed {
            camera.eye =
                camera.target - (forward - camera.up * self.speed).normalize() * forward_mag;
        }
        if self.is_reset {
            camera.eye = self.default_position;
            camera.target = self.default_target;
        }
        if self.is_scroll && forward_mag > self.scroll_speed * 0.15 {
            camera.eye += forward_norm * (self.scroll_speed * 0.15);
        }
        if self.is_orbit {
            let delta= ((forward - camera.up * self.orbit_vertical) + (forward + right * self.orbit_horizontal)).normalize();
            camera.eye = camera.target - delta * forward_mag;

            self.orbit_horizontal = 0.;
            self.orbit_vertical = 0.;
        }
        
    }
}
