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
    pub following_target: Option<Vector3>,
    pub yaw: f32,
    pub pitch: f32,
    pub follow_distance: f32,
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
            following_target: None,
            yaw: 0.0,
            pitch: 0.0,
            follow_distance: 15.0,
        }
    }

    pub fn handle_input(&mut self, window: &raylib::RaylibHandle, delta_time: f32) {
        if !self.free_mode {
            return;
        }

        let move_speed = self.speed * delta_time;
        let rotate_speed = self.rotation_speed * delta_time;

        // Obtener vectores de dirección
        let forward = self.get_forward_vector();
        let right = forward.cross(self.up).normalized();
        let up_dir = self.up;

        // Movimiento WASD
        if window.is_key_down(KeyboardKey::KEY_W) {
            self.position = self.position + forward * move_speed;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            self.position = self.position - forward * move_speed;
        }
        if window.is_key_down(KeyboardKey::KEY_A) {
            self.position = self.position - right * move_speed;
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            self.position = self.position + right * move_speed;
        }

        // Movimiento vertical Q/E
        if window.is_key_down(KeyboardKey::KEY_Q) {
            self.position = self.position + up_dir * move_speed;
        }
        if window.is_key_down(KeyboardKey::KEY_E) {
            self.position = self.position - up_dir * move_speed;
        }

        // Rotación con mouse (click derecho)
        if window.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let mouse_delta = window.get_mouse_delta();
            self.yaw -= mouse_delta.x * rotate_speed * 0.1;
            self.pitch -= mouse_delta.y * rotate_speed * 0.1;
            
            // Limitar el pitch para evitar voltear la cámara
            self.pitch = self.pitch.clamp(-89.0f32.to_radians(), 89.0f32.to_radians());
        }

        // Actualizar target basado en yaw y pitch
        self.update_target_from_angles();
    }

    fn get_forward_vector(&self) -> Vector3 {
        Vector3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos()
        ).normalized()
    }

    fn update_target_from_angles(&mut self) {
        let forward = self.get_forward_vector();
        self.target = self.position + forward;
    }

    pub fn get_view_direction(&self) -> Vector3 {
        (self.target - self.position).normalized()
    }

    pub fn look_at(&mut self, target: Vector3) {
        let direction = (target - self.position).normalized();
        self.pitch = direction.y.asin();
        self.yaw = direction.x.atan2(direction.z);
        self.target = target;
        self.following_target = Some(target);
        self.free_mode = false;
    }

    pub fn set_free_mode(&mut self) {
        self.free_mode = true;
        self.following_target = None;
    }

    pub fn warp_to(&mut self, target_position: Vector3, offset_distance: f32) {
        // Calcular dirección desde el sol hacia el planeta para posicionar la cámara
        let direction_to_sun = (Vector3::zero() - target_position).normalized();
        
        // Posicionar la cámara detrás del planeta (desde la perspectiva del sol)
        self.position = target_position + direction_to_sun * offset_distance;
        
        // Mirar directamente al planeta
        self.target = target_position;
        
        // Actualizar ángulos de la cámara
        let direction = (self.target - self.position).normalized();
        self.pitch = direction.y.asin();
        self.yaw = direction.x.atan2(direction.z);
        
        self.following_target = Some(target_position);
        self.follow_distance = offset_distance;
        self.free_mode = false;
    }

    pub fn update_following(&mut self, target_position: Vector3) {
        if let Some(_) = self.following_target {
            if !self.free_mode {
                // Mantener la cámara en una posición relativa al planeta
                let direction_to_sun = (Vector3::zero() - target_position).normalized();
                self.position = target_position + direction_to_sun * self.follow_distance;
                self.target = target_position;
                
                // Actualizar ángulos para mantener la orientación
                let direction = (self.target - self.position).normalized();
                self.pitch = direction.y.asin();
                self.yaw = direction.x.atan2(direction.z);
            }
        }
    }
}
