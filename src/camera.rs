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
    pub following_target: Option<usize>,
    pub follow_distance: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: Vector3::new(0.0, 5.0, 25.0),
            target: Vector3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            speed: 25.0,
            rotation_speed: 2.0,
            free_mode: true,
            following_target: None,
            follow_distance: 15.0,
        }
    }

    pub fn handle_input(&mut self, window: &raylib::RaylibHandle, delta_time: f32, solar_system: &crate::solar_system::SolarSystem) {
        let move_speed = self.speed * delta_time;
        let rotation_speed = self.rotation_speed * delta_time;

        if self.free_mode {
            // MOVIMIENTO LIBRE - WASD + FLECHAS
            let forward = (self.target - self.position).normalized();
            let right = forward.cross(self.up).normalized();
            let up_vector = self.up.normalized();

            // Movimiento forward/backward (W/S)
            if window.is_key_down(KeyboardKey::KEY_W) {
                self.position = self.position + forward * move_speed;
                self.target = self.target + forward * move_speed;
            }
            if window.is_key_down(KeyboardKey::KEY_S) {
                self.position = self.position - forward * move_speed;
                self.target = self.target - forward * move_speed;
            }
            
            // Movimiento lateral (A/D)
            if window.is_key_down(KeyboardKey::KEY_A) {
                self.position = self.position - right * move_speed;
                self.target = self.target - right * move_speed;
            }
            if window.is_key_down(KeyboardKey::KEY_D) {
                self.position = self.position + right * move_speed;
                self.target = self.target + right * move_speed;
            }

            // Movimiento vertical (UP/DOWN)
            if window.is_key_down(KeyboardKey::KEY_UP) {
                self.position = self.position + up_vector * move_speed;
                self.target = self.target + up_vector * move_speed;
            }
            if window.is_key_down(KeyboardKey::KEY_DOWN) {
                self.position = self.position - up_vector * move_speed;
                self.target = self.target - up_vector * move_speed;
            }

            // Rotación de cámara (Q/E)
            if window.is_key_down(KeyboardKey::KEY_Q) {
                self.rotate_around_position(rotation_speed);
            }
            if window.is_key_down(KeyboardKey::KEY_E) {
                self.rotate_around_position(-rotation_speed);
            }

            // Rotación vertical (R/F)
            if window.is_key_down(KeyboardKey::KEY_R) {
                self.rotate_vertical(rotation_speed * 0.5);
            }
            if window.is_key_down(KeyboardKey::KEY_F) {
                self.rotate_vertical(-rotation_speed * 0.5);
            }
        }

        // Verificar colisiones en modo libre
        if self.free_mode {
            self.avoid_collisions(solar_system);
        }
    }

    fn rotate_around_position(&mut self, angle: f32) {
        let direction = self.target - self.position;
        let distance = direction.length();
        
        // Rotar alrededor del eje Y
        let new_direction = Vector3::new(
            direction.x * angle.cos() - direction.z * angle.sin(),
            direction.y,
            direction.x * angle.sin() + direction.z * angle.cos()
        ).normalized();
        
        self.target = self.position + new_direction * distance;
    }

    fn rotate_vertical(&mut self, angle: f32) {
        let direction = self.target - self.position;
        let right = direction.cross(self.up).normalized();
        
        // Rotar verticalmente usando el eje derecho
        let new_direction = direction * angle.cos() + self.up * angle.sin();
        let new_up = self.up * angle.cos() - direction * angle.sin();
        
        self.target = self.position + new_direction.normalized() * direction.length();
        self.up = new_up.normalized();
    }

    fn avoid_collisions(&mut self, solar_system: &crate::solar_system::SolarSystem) {
        for body in &solar_system.bodies {
            let distance = (self.position - body.position).length();
            let safe_distance = body.scale * 2.0 + 3.0;
            
            if distance < safe_distance {
                // Empujar la cámara fuera del cuerpo
                let push_direction = (self.position - body.position).normalized();
                self.position = body.position + push_direction * safe_distance;
                self.target = self.target + push_direction * safe_distance;
            }
        }
    }

    pub fn get_view_direction(&self) -> Vector3 {
        (self.target - self.position).normalized()
    }

    pub fn set_free_mode(&mut self) {
        self.free_mode = true;
        self.following_target = None;
        println!("Modo cámara libre activado");
    }

    pub fn warp_to(&mut self, target_position: Vector3, target_index: usize, offset_distance: f32) {
        println!("WARP: Posición objetivo: {:?}, Índice: {}, Distancia: {}", 
                 target_position, target_index, offset_distance);
        
        if target_index == 0 {
            // Para el sol, posición fija arriba
            self.position = Vector3::new(0.0, 10.0, offset_distance);
            self.target = Vector3::zero();
        } else {
            // Para planetas, calcular dirección desde el sol
            let direction = if target_position.length() < 0.1 {
                Vector3::new(0.0, 0.2, 1.0) // Dirección por defecto
            } else {
                (target_position - Vector3::zero()).normalized()
            };
            
            self.position = target_position + direction * offset_distance;
            self.target = target_position;
        }
        
        self.following_target = Some(target_index);
        self.follow_distance = offset_distance;
        self.free_mode = false;
        
        println!("Cámara teletransportada a índice {} en posición {:?}", target_index, self.position);
    }

    pub fn update_following(&mut self, solar_system: &crate::solar_system::SolarSystem) {
        if let Some(target_index) = self.following_target {
            if !self.free_mode && target_index < solar_system.bodies.len() {
                let body = &solar_system.bodies[target_index];
                
                // DEBUG: Verificar posición del cuerpo
                if rand::random::<f32>() < 0.02 {
                    println!("SIGUIENDO: {} en posición {:?}", body.name, body.position);
                }
                
                if target_index == 0 {
                    // Para el sol, mantener posición fija
                    self.target = Vector3::zero();
                    // También actualizar la posición de la cámara para el sol
                    let direction = (self.position - self.target).normalized();
                    self.position = self.target + direction * self.follow_distance;
                } else {
                    // Para planetas, seguir manteniendo la distancia relativa
                    let direction_from_sun = (body.position - Vector3::zero()).normalized();
                    self.position = body.position + direction_from_sun * self.follow_distance;
                    self.target = body.position;
                }
            }
        }
    }
}
