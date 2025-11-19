use raylib::prelude::*;
use crate::utils::Vector3Ext;

#[derive(Clone, Copy)]
pub struct Camera {
    pub position: Vector3,
    pub target: Vector3,
    pub up: Vector3,
    pub speed: f32,
    pub rotation_speed: f32,
    pub free_mode: bool,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 5.0, 25.0),
            target: Vector3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            speed: 25.0,
            rotation_speed: 1.5,
            free_mode: true,
        }
    }

    pub fn handle_input(&mut self, window: &raylib::RaylibHandle, delta_time: f32) {
        let move_speed = self.speed * delta_time;
        let rotate_speed = self.rotation_speed * delta_time;

        // Dirección forward y right
        let forward = (self.target - self.position).normalized();
        let right = forward.cross(self.up).normalized();

        // Movimiento WASD - Mejorado
        if window.is_key_down(KeyboardKey::KEY_W) {
            self.position = self.position + forward * move_speed;
            if self.free_mode {
                self.target = self.position + forward;
            }
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            self.position = self.position - forward * move_speed;
            if self.free_mode {
                self.target = self.position + forward;
            }
        }
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.position = self.position - right * move_speed;
            if self.free_mode {
                self.target = self.position + forward;
            }
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.position = self.position + right * move_speed;
            if self.free_mode {
                self.target = self.position + forward;
            }
        }

        // Movimiento vertical Q/E
        if window.is_key_down(KeyboardKey::KEY_Q) {
            self.position.y += move_speed;
            if self.free_mode {
                self.target.y += move_speed;
            }
        }
        if window.is_key_down(KeyboardKey::KEY_E) {
            self.position.y -= move_speed;
            if self.free_mode {
                self.target.y -= move_speed;
            }
        }

        // Rotación con mouse - Mejorada
        if window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let mouse_delta = window.get_mouse_delta();
            self.rotate_free(mouse_delta.x * rotate_speed, mouse_delta.y * rotate_speed);
        }
    }

    fn rotate_free(&mut self, yaw: f32, pitch: f32) {
        let forward = (self.target - self.position).normalized();
        let right = forward.cross(self.up).normalized();
        
        // Rotación Yaw alrededor del eje Y
        let yaw_rotation = Matrix::rotate_y(yaw);
        let mut new_forward = forward.transform(&yaw_rotation);
        
        // Rotación Pitch usando Matrix::rotate con el eje right
        let pitch_rotation = Matrix::rotate(right, pitch);
        new_forward = new_forward.transform(&pitch_rotation);
        
        self.target = self.position + new_forward;
    }

    pub fn get_view_direction(&self) -> Vector3 {
        (self.target - self.position).normalized()
    }

    pub fn look_at(&mut self, target: Vector3) {
        self.target = target;
        self.free_mode = false;
    }

    pub fn set_free_mode(&mut self) {
        self.free_mode = true;
        // Mantener la dirección actual pero en modo libre
        let forward = (self.target - self.position).normalized();
        self.target = self.position + forward;
    }

    pub fn warp_to(&mut self, target_position: Vector3, offset_distance: f32) {
        let direction_to_target = (target_position - self.position).normalized();
        self.position = target_position - direction_to_target * offset_distance;
        self.target = target_position;
        self.free_mode = false;
    }
}
