use cgmath::{InnerSpace, Point3};
use winit::{
    dpi::PhysicalPosition,
    event::{ElementState, MouseScrollDelta},
    keyboard::KeyCode,
};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct OrbitCamera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl OrbitCamera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
        let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct OrbitCameraUniform {
    view_proj: [[f32; 4]; 4],
    camera_location: [f32; 3],
    _padding: u32,
}

impl OrbitCameraUniform {
    pub fn new(eye: cgmath::Point3<f32>) -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
            camera_location: eye.into(),
            _padding: 0,
        }
    }

    pub fn update_view_proj(&mut self, camera: &OrbitCamera) {
        self.view_proj = camera.build_view_projection_matrix().into();
        self.camera_location = camera.eye.into();
    }
}

pub struct OrbitCameraController {
    pub is_reset: bool,
    pub is_scroll: bool,
    pub scroll_speed: f32,
    pub is_orbit: bool,
    pub orbit_horizontal: f32,
    pub orbit_vertical: f32,
    pub default_position: Point3<f32>,
    pub default_target: Point3<f32>,
}

impl OrbitCameraController {
    pub fn new(default_position: Point3<f32>, default_target: Point3<f32>) -> Self {
        Self {
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
            KeyCode::Space => {
                self.is_reset = state == ElementState::Pressed;
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

    pub fn update_camera(&mut self, camera: &mut OrbitCamera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let right = forward_norm.cross(camera.up);
        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_reset {
            camera.eye = self.default_position;
            camera.target = self.default_target;
        }
        if self.is_scroll && forward_mag > self.scroll_speed * 0.15 {
            camera.eye += forward_norm * (self.scroll_speed * 0.15);
        }
        if self.is_orbit {
            let delta = ((forward - camera.up * self.orbit_vertical)
                + (forward + right * self.orbit_horizontal))
                .normalize();
            camera.eye = camera.target - delta * forward_mag;

            self.orbit_horizontal = 0.;
            self.orbit_vertical = 0.;
        }
    }
}
