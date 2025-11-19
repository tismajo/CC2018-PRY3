use raylib::prelude::*;
use crate::framebuffer::Framebuffer;
use crate::camera::Camera;
use rand::Rng;

pub struct Star {
    pub direction: Vector3,
    pub brightness: f32,
    pub twinkle_speed: f32,
    pub twinkle_phase: f32,
    pub size: f32,
}

pub struct Skybox {
    pub stars: Vec<Star>,
    pub time: f32,
}

impl Skybox {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut stars = Vec::new();
        
        // Crear más estrellas para mejor efecto
        for _ in 0..1500 {
            // Distribución esférica uniforme
            let theta: f32 = rng.gen_range(0.0..std::f32::consts::PI * 2.0);
            let random_val: f32 = rng.gen_range(-1.0..1.0);
            let phi = random_val.acos();
            
            let x = phi.sin() * theta.cos();
            let y = phi.sin() * theta.sin();
            let z = phi.cos();
            
            stars.push(Star {
                direction: Vector3::new(x, y, z),
                brightness: rng.gen_range(0.3..1.0), // Rango más amplio
                twinkle_speed: rng.gen_range(0.5..3.5), // Velocidades más variadas
                twinkle_phase: rng.gen_range(0.0..std::f32::consts::PI * 2.0),
                size: rng.gen_range(0.5..2.0), // Tamaños variados
            });
        }

        Self {
            stars,
            time: 0.0,
        }
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
        
        let fov = 90.0f32.to_radians();
        let aspect = width / height;

        for star in &self.stars {
            let dot = star.direction.dot(view_dir);
            
            // Solo renderizar estrellas en el campo de visión
            if dot > 0.2 { // Umbral más bajo para más estrellas visibles
                // Proyección más precisa
                let relative_pos = star.direction - view_dir * dot;
                let x_offset = relative_pos.dot(right);
                let y_offset = relative_pos.dot(up);
                
                let screen_x = (width / 2.0 + x_offset * width / (2.0 * fov.tan() * aspect)) as i32;
                let screen_y = (height / 2.0 - y_offset * height / (2.0 * fov.tan())) as i32;
                
                // Efecto de titileo más pronunciado
                let twinkle = ((self.time * star.twinkle_speed + star.twinkle_phase).sin() * 0.5 + 0.5)
                    .max(0.0).min(1.0);
                
                let final_brightness = (star.brightness * twinkle * 255.0) as u8;
                let color = Color::new(final_brightness, final_brightness, final_brightness, 255);
                
                if screen_x >= 0 && screen_x < width as i32 && screen_y >= 0 && screen_y < height as i32 {
                    // Dibujar estrella principal
                    framebuffer.set_pixel_with_color(screen_x, screen_y, color);
                    
                    // Estrellas más brillantes son más grandes
                    if final_brightness > 150 {
                        framebuffer.set_pixel_with_color(screen_x + 1, screen_y, color);
                        framebuffer.set_pixel_with_color(screen_x - 1, screen_y, color);
                        framebuffer.set_pixel_with_color(screen_x, screen_y + 1, color);
                        framebuffer.set_pixel_with_color(screen_x, screen_y - 1, color);
                    }
                    
                    // Estrellas muy brillantes son aún más grandes
                    if final_brightness > 220 {
                        framebuffer.set_pixel_with_color(screen_x + 1, screen_y + 1, color);
                        framebuffer.set_pixel_with_color(screen_x - 1, screen_y - 1, color);
                        framebuffer.set_pixel_with_color(screen_x + 1, screen_y - 1, color);
                        framebuffer.set_pixel_with_color(screen_x - 1, screen_y + 1, color);
                    }
                }
            }
        }
    }
}

// Traits necesarios para Vector3
pub trait Vector3Normalized {
    fn normalized(self) -> Self;
    fn cross(self, other: Self) -> Self;
    fn dot(self, other: Self) -> f32;
}

impl Vector3Normalized for Vector3 {
    fn normalized(self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        if len > 0.0 {
            Vector3::new(self.x / len, self.y / len, self.z / len)
        } else {
            self
        }
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
