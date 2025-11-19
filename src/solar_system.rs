use raylib::prelude::*;
use crate::obj_loader::ObjModel;
use crate::triangle::ShaderType;
use crate::procedural_geometry::{transform_model, generate_rings};
use crate::utils::Vector3Ext;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct CelestialBody {
    pub name: String,
    pub body_type: BodyType,
    pub position: Vector3,
    pub rotation: f32,
    pub scale: f32,
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub rotation_speed: f32,
    pub orbit_angle: f32,
    pub shader_type: ShaderType,
    pub has_rings: bool,
    pub color: Color,
}

#[derive(Clone)]
pub enum BodyType {
    Star,
    Planet,
    GasGiant,
    IceGiant,
    DwarfPlanet,
}

pub struct SolarSystem {
    pub bodies: Vec<CelestialBody>,
    pub time: f32,
}

impl SolarSystem {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            time: 0.0,
        }
    }

    pub fn initialize_system(&mut self, _sphere_model: &ObjModel) {
        // Sol - Estrella central
        self.bodies.push(CelestialBody {
            name: "Sol Nebularis".to_string(),
            body_type: BodyType::Star,
            position: Vector3::zero(),
            rotation: 0.0,
            scale: 3.0,
            orbit_radius: 0.0,
            orbit_speed: 0.0,
            rotation_speed: 0.02,
            orbit_angle: 0.0,
            shader_type: ShaderType::Lava,
            has_rings: false,
            color: Color::YELLOW,
        });

        // Planetas rocosos internos
        self.bodies.push(CelestialBody {
            name: "Mercurio Primus".to_string(),
            body_type: BodyType::Planet,
            position: Vector3::zero(),
            rotation: 0.0,
            scale: 0.4,
            orbit_radius: 8.0,
            orbit_speed: 0.8,
            rotation_speed: 0.04,
            orbit_angle: 0.0,
            shader_type: ShaderType::Rocky,
            has_rings: false,
            color: Color::GRAY,
        });

        self.bodies.push(CelestialBody {
            name: "Venusia".to_string(),
            body_type: BodyType::Planet,
            position: Vector3::zero(),
            rotation: 0.0,
            scale: 0.7,
            orbit_radius: 12.0,
            orbit_speed: 0.6,
            rotation_speed: 0.02,
            orbit_angle: PI * 0.3,
            shader_type: ShaderType::Lava,
            has_rings: false,
            color: Color::ORANGE,
        });

        self.bodies.push(CelestialBody {
            name: "Terra Nova".to_string(),
            body_type: BodyType::Planet,
            position: Vector3::zero(),
            rotation: 0.0,
            scale: 0.8,
            orbit_radius: 16.0,
            orbit_speed: 0.5,
            rotation_speed: 0.03,
            orbit_angle: PI * 0.6,
            shader_type: ShaderType::Rocky,
            has_rings: false,
            color: Color::BLUE,
        });

        self.bodies.push(CelestialBody {
            name: "Marte Secundus".to_string(),
            body_type: BodyType::Planet,
            position: Vector3::zero(),
            rotation: 0.0,
            scale: 0.6,
            orbit_radius: 20.0,
            orbit_speed: 0.4,
            rotation_speed: 0.035,
            orbit_angle: PI * 0.9,
            shader_type: ShaderType::Rocky,
            has_rings: false,
            color: Color::RED,
        });

        // Cinturón de asteroides
        self.bodies.push(CelestialBody {
            name: "Ceres Minor".to_string(),
            body_type: BodyType::DwarfPlanet,
            position: Vector3::zero(),
            rotation: 0.0,
            scale: 0.2,
            orbit_radius: 25.0,
            orbit_speed: 0.3,
            rotation_speed: 0.05,
            orbit_angle: PI * 1.2,
            shader_type: ShaderType::Rocky,
            has_rings: false,
            color: Color::BROWN,
        });

        // Gigantes gaseosos
        self.bodies.push(CelestialBody {
            name: "Jupiter Magnus".to_string(),
            body_type: BodyType::GasGiant,
            position: Vector3::zero(),
            rotation: 0.0,
            scale: 1.8,
            orbit_radius: 32.0,
            orbit_speed: 0.25,
            rotation_speed: 0.04,
            orbit_angle: PI * 1.5,
            shader_type: ShaderType::Gas,
            has_rings: true,
            color: Color::ORANGE,
        });

        self.bodies.push(CelestialBody {
            name: "Saturnus".to_string(),
            body_type: BodyType::GasGiant,
            position: Vector3::zero(),
            rotation: 0.0,
            scale: 1.5,
            orbit_radius: 40.0,
            orbit_speed: 0.2,
            rotation_speed: 0.035,
            orbit_angle: PI * 1.8,
            shader_type: ShaderType::Gas,
            has_rings: true,
            color: Color::GOLD,
        });

        // Gigantes de hielo
        self.bodies.push(CelestialBody {
            name: "Urania".to_string(),
            body_type: BodyType::IceGiant,
            position: Vector3::zero(),
            rotation: 0.0,
            scale: 1.2,
            orbit_radius: 48.0,
            orbit_speed: 0.15,
            rotation_speed: 0.025,
            orbit_angle: PI * 2.1,
            shader_type: ShaderType::Ice,
            has_rings: false,
            color: Color::SKYBLUE,
        });

        self.bodies.push(CelestialBody {
            name: "Neptunus".to_string(),
            body_type: BodyType::IceGiant,
            position: Vector3::zero(),
            rotation: 0.0,
            scale: 1.2,
            orbit_radius: 56.0,
            orbit_speed: 0.12,
            rotation_speed: 0.03,
            orbit_angle: PI * 2.4,
            shader_type: ShaderType::Ice,
            has_rings: false,
            color: Color::BLUE,
        });
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time += delta_time;

        for body in &mut self.bodies {
            // Actualizar órbita
            body.orbit_angle += body.orbit_speed * delta_time;
            body.position.x = body.orbit_radius * body.orbit_angle.cos();
            body.position.z = body.orbit_radius * body.orbit_angle.sin();

            // Actualizar rotación
            body.rotation += body.rotation_speed * delta_time;
        }
    }

    pub fn render(&self, framebuffer: &mut crate::framebuffer::Framebuffer, 
                 camera: &crate::camera::Camera, show_orbits: bool) {
        
        // Renderizar órbitas primero
        if show_orbits {
            self.render_orbits(framebuffer, camera);
        }

        // Renderizar cuerpos celestes
        for body in &self.bodies {
            self.render_body(framebuffer, camera, body);
        }
    }

    fn render_orbits(&self, framebuffer: &mut crate::framebuffer::Framebuffer, 
                    camera: &crate::camera::Camera) {
        use crate::line::line;
        use raylib::prelude::Vector2;

        for body in &self.bodies {
            if body.orbit_radius > 0.0 {
                let orbit_color = Color::new(100, 100, 100, 100);
                framebuffer.set_current_color(orbit_color);

                // Dibujar órbita circular
                let segments = 64;
                let mut prev_point = self.project_to_screen(
                    Vector3::new(body.orbit_radius, 0.0, 0.0), 
                    camera, 
                    framebuffer.width as f32, 
                    framebuffer.height as f32
                );

                for i in 1..=segments {
                    let angle = 2.0 * PI * (i as f32) / (segments as f32);
                    let point = Vector3::new(
                        body.orbit_radius * angle.cos(),
                        0.0,
                        body.orbit_radius * angle.sin()
                    );
                    
                    let screen_point = self.project_to_screen(point, camera, 
                        framebuffer.width as f32, framebuffer.height as f32);
                    
                    line(framebuffer, prev_point, screen_point);
                    prev_point = screen_point;
                }
            }
        }
    }

    fn render_body(&self, framebuffer: &mut crate::framebuffer::Framebuffer,
                  camera: &crate::camera::Camera, body: &CelestialBody) {
        // Usar sphere-1.obj como modelo base para todos los cuerpos
        let model = crate::obj_loader::ObjModel::load("sphere-1.obj")
            .unwrap_or_else(|_| {
                // Fallback: crear una esfera simple
                self.create_fallback_sphere()
            });

        // Transformar modelo
        let transformed = transform_model(
            &model,
            body.position,
            body.rotation,
            0.0,
            body.scale
        );

        // Renderizar triángulos
        for face in &model.faces {
            if face.len() < 3 { continue; }
            
            for i in 1..(face.len() - 1) {
                let v0 = transformed[face[0]];
                let v1 = transformed[face[i]];
                let v2 = transformed[face[i + 1]];
                
                crate::triangle::draw_filled_triangle(
                    framebuffer, v0, v1, v2, body.shader_type, self.time
                );
            }
        }

        // Renderizar anillos si los tiene
        if body.has_rings {
            self.render_rings(framebuffer, camera, body);
        }
    }

    fn render_rings(&self, framebuffer: &mut crate::framebuffer::Framebuffer,
                   camera: &crate::camera::Camera, body: &CelestialBody) {
        let rings_model = generate_rings(body.scale * 1.3, body.scale * 2.2, 32);
        let rings_transformed = transform_model(
            &rings_model,
            body.position,
            body.rotation * 0.5,
            0.4,
            body.scale
        );

        for face in &rings_model.faces {
            if face.len() < 3 { continue; }
            
            for i in 1..(face.len() - 1) {
                let v0 = rings_transformed[face[0]];
                let v1 = rings_transformed[face[i]];
                let v2 = rings_transformed[face[i + 1]];
                
                crate::triangle::draw_filled_triangle(
                    framebuffer, v0, v1, v2, ShaderType::Crystal, self.time
                );
            }
        }
    }

    fn project_to_screen(&self, point: Vector3, camera: &crate::camera::Camera, 
                        width: f32, height: f32) -> raylib::prelude::Vector2 {
        // Proyección simple para las órbitas
        let fov = 90.0f32.to_radians();
        
        let view_dir = camera.get_view_direction();
        let right = view_dir.cross(camera.up).normalized();
        let up = right.cross(view_dir).normalized();
        
        let relative_pos = point - camera.position;
        let distance = relative_pos.dot(view_dir);
        
        if distance > 0.0 {
            let scale = 1.0 / (distance * fov.tan());
            let screen_x = width / 2.0 + relative_pos.dot(right) * scale * width / 2.0;
            let screen_y = height / 2.0 - relative_pos.dot(up) * scale * height / 2.0;
            
            raylib::prelude::Vector2::new(screen_x, screen_y)
        } else {
            raylib::prelude::Vector2::new(-1.0, -1.0)
        }
    }

    fn create_fallback_sphere(&self) -> crate::obj_loader::ObjModel {
        use crate::procedural_geometry::generate_moon;
        generate_moon(1.0, 16)
    }
}
