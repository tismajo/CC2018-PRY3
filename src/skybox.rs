use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub struct Skybox {
    stars: Vec<Star>,
    time: f32,
}

#[derive(Clone)]
struct Star {
    position: Vector3,
    brightness: f32,
    flicker_speed: f32,
    size: f32,
}

impl Skybox {
    pub fn new() -> Self {
        let mut stars = Vec::new();
        
        // Generar estrellas proceduralmente
        for i in 0..500 {
            let theta = (i as f32 * 137.5).to_radians(); // Ángulo áureo para distribución uniforme
            let phi = (i as f32 * 0.618).acos(); // Distribución esférica
            
            let x = phi.sin() * theta.cos();
            let y = phi.sin() * theta.sin();
            let z = phi.cos();
            
            let distance = 100.0 + (i as f32 % 50.0); // Variar distancias
            let position = Vector3::new(x * distance, y * distance, z * distance);
            
            // Usar operador módulo con enteros y luego convertir a float
            let i_mod_7 = (i % 7) as f32;
            let i_mod_5 = (i % 5) as f32;
            let i_mod_3 = (i % 3) as f32;
            
            stars.push(Star {
                position,
                brightness: 0.3 + i_mod_7 * 0.1,
                flicker_speed: 0.5 + i_mod_5 * 0.2,
                size: 0.5 + i_mod_3 * 0.3,
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
    
    pub fn render(&self, framebuffer: &mut Framebuffer, camera: &crate::camera::Camera) {
        let width = framebuffer.width as f32;
        let height = framebuffer.height as f32;
        
        for star in &self.stars {
            // Proyectar estrella a pantalla
            let screen_pos = self.project_star(star.position, camera, width, height);
            
            if screen_pos.x >= 0.0 && screen_pos.x < width && 
               screen_pos.y >= 0.0 && screen_pos.y < height {
                
                // Calcular brillo titilante
                let flicker = (self.time * star.flicker_speed).sin() * 0.3 + 0.7;
                let brightness = star.brightness * flicker;
                
                // Color de la estrella (ligeramente azulado/blanco)
                let base_color = if brightness > 0.8 {
                    Color::new(255, 255, 255, (brightness * 255.0) as u8)
                } else if brightness > 0.6 {
                    Color::new(200, 220, 255, (brightness * 255.0) as u8)
                } else {
                    Color::new(150, 180, 220, (brightness * 255.0) as u8)
                };
                
                // Dibujar estrella como un pequeño punto
                let size = (star.size * brightness) as i32;
                self.draw_star(framebuffer, screen_pos.x as i32, screen_pos.y as i32, size, base_color);
            }
        }
    }
    
    fn project_star(&self, star_pos: Vector3, camera: &crate::camera::Camera, width: f32, height: f32) -> Vector2 {
        let view_dir = camera.get_view_direction();
        let right = view_dir.cross(camera.up).normalized();
        let up = right.cross(view_dir).normalized();
        
        let relative_pos = star_pos - camera.position;
        let distance = relative_pos.dot(view_dir);
        
        if distance > 0.0 {
            let fov = 60.0f32.to_radians();
            let scale = 1.0 / (distance * fov.tan());
            
            let screen_x = width / 2.0 + relative_pos.dot(right) * scale * width / 2.0;
            let screen_y = height / 2.0 - relative_pos.dot(up) * scale * height / 2.0;
            
            Vector2::new(screen_x, screen_y)
        } else {
            Vector2::new(-1.0, -1.0)
        }
    }
    
    fn draw_star(&self, framebuffer: &mut Framebuffer, x: i32, y: i32, size: i32, color: Color) {
        if size == 1 {
            framebuffer.set_pixel_with_color(x, y, color);
        } else {
            // Dibujar estrella como un pequeño círculo
            for dy in -size..=size {
                for dx in -size..=size {
                    if dx * dx + dy * dy <= size * size {
                        let px = x + dx;
                        let py = y + dy;
                        if px >= 0 && px < framebuffer.width as i32 && 
                           py >= 0 && py < framebuffer.height as i32 {
                            framebuffer.set_pixel_with_color(px, py, color);
                        }
                    }
                }
            }
        }
    }
}
