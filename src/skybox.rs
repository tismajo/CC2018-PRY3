use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use rand::Rng;

pub struct Star {
    pub direction: Vector3,
    pub brightness: f32,
    pub twinkle_speed: f32,
    pub twinkle_phase: f32,
}

pub struct Skybox {
    pub stars: Vec<Star>,
    pub time: f32,
}

impl Skybox {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut stars = Vec::new();
        
        // CREAR MUCHAS ESTRELLAS
        for _ in 0..4000 {
            let theta = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
            let phi = rng.gen_range(0.0..std::f32::consts::PI);
            
            let x = phi.sin() * theta.cos();
            let y = phi.cos();
            let z = phi.sin() * theta.sin();
            
            stars.push(Star {
                direction: Vector3::new(x, y, z),
                brightness: rng.gen_range(0.2..1.0),
                twinkle_speed: rng.gen_range(0.1..2.0),
                twinkle_phase: rng.gen_range(0.0..std::f32::consts::PI * 2.0),
            });
        }

        Self { stars, time: 0.0 }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, camera: &Camera) {
        let width = framebuffer.width as f32;
        let height = framebuffer.height as f32;
        
        let view_dir = camera.get_view_direction();
        let right = view_dir.cross(camera.up).normalized();
        let up = right.cross(view_dir).normalized();
        
        let fov = 60.0f32.to_radians(); // Reducir FOV para mejor perspectiva
        let aspect = width / height;

        for star in &self.stars {
            // Proyectar la estrella directamente en coordenadas de pantalla
            let dot = star.direction.dot(view_dir);
            
            // Mostrar estrellas en todo el campo de visión
            if dot > -0.3 {  // Umbral más permisivo
                let twinkle = (self.time * star.twinkle_speed + star.twinkle_phase).sin() * 0.3 + 0.7;
                let brightness = (star.brightness * twinkle * 255.0) as u8;
                
                if brightness > 20 {
                    // Proyección más simple y directa
                    let screen_x = (width / 2.0 + star.direction.x * width / 2.0) as i32;
                    let screen_y = (height / 2.0 + star.direction.y * height / 2.0) as i32;
                    
                    if screen_x >= 0 && screen_x < width as i32 && screen_y >= 0 && screen_y < height as i32 {
                        let color = Color::new(brightness, brightness, brightness, 255);
                        framebuffer.set_pixel_with_color(screen_x, screen_y, color);
                        
                        // Hacer estrellas más brillantes más grandes
                        if brightness > 150 {
                            for dx in -1..=1 {
                                for dy in -1..=1 {
                                    if dx == 0 && dy == 0 { continue; }
                                    framebuffer.set_pixel_with_color(screen_x + dx, screen_y + dy, color);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub trait Vector3Normalized {
    fn normalized(self) -> Self;
    fn cross(self, other: Self) -> Self;
    fn dot(self, other: Self) -> f32;
}

impl Vector3Normalized for Vector3 {
    fn normalized(self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 0.0 { Vector3::new(self.x / len, self.y / len, self.z / len) } else { self }
    }

    fn cross(self, other: Self) -> Self {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}
