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

        for _ in 0..800 {
            let theta = rng.r#gen::<f32>() * std::f32::consts::PI * 2.0;
            let phi = rng.r#gen::<f32>() * std::f32::consts::PI;
            
            let x = phi.sin() * theta.cos();
            let y = phi.sin() * theta.sin();
            let z = phi.cos();

            stars.push(Star {
                direction: Vector3::new(x, y, z).normalized(),
                brightness: rng.r#gen::<f32>() * 0.5 + 0.5,
                twinkle_speed: rng.r#gen::<f32>() * 2.0 + 1.0,
                twinkle_phase: rng.r#gen::<f32>() * std::f32::consts::PI * 2.0,
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
            
            if dot > 0.0 {
                let relative = star.direction - view_dir * dot;
                let x_offset = relative.dot(right);
                let y_offset = relative.dot(up);

                let screen_x = (width / 2.0 + x_offset * width / (2.0 * fov.tan() * aspect)) as i32;
                let screen_y = (height / 2.0 - y_offset * height / (2.0 * fov.tan())) as i32;

                let twinkle = ((self.time * star.twinkle_speed + star.twinkle_phase).sin() * 0.3 + 0.7).max(0.0).min(1.0);
                let final_brightness = (star.brightness * twinkle * 255.0) as u8;

                let color = Color::new(final_brightness, final_brightness, final_brightness, 255);

                if screen_x >= 0 && screen_x < width as i32 && screen_y >= 0 && screen_y < height as i32 {
                    framebuffer.set_pixel_with_color(screen_x, screen_y, color);
                    if final_brightness > 200 {
                        framebuffer.set_pixel_with_color(screen_x + 1, screen_y, color);
                        framebuffer.set_pixel_with_color(screen_x, screen_y + 1, color);
                    }
                }
            }
        }
    }
}

trait Vector3Normalized {
    fn normalized(self) -> Self;
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
}
