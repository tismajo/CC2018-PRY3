#![allow(unused_imports)]
mod framebuffer;
mod line;
mod obj_loader;
mod shader;
mod triangle;
mod procedural_geometry;
mod camera;
mod solar_system;
mod utils;
mod skybox;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use obj_loader::ObjModel;
use triangle::ShaderType;
use camera::Camera;
use solar_system::{SolarSystem, CelestialBody};
use skybox::Skybox;
use utils::MatrixExt;
use std::f32::consts::PI;

fn main() {
    let (mut window, thread) = raylib::init()
        .size(1200, 800)
        .title("Sistema Solar - Renderer Personalizado")
        .build();

    let mut fb = Framebuffer::new(1200, 800, Color::new(5, 5, 15, 255));

    println!("Cargando modelos...");
    let model_sphere = ObjModel::load("sphere-1.obj")
        .expect("No se pudo cargar sphere-1.obj");
    println!("✓ Modelos cargados correctamente");

    let mut solar_system = SolarSystem::new();
    solar_system.initialize_system(&model_sphere);

    let mut camera = Camera::new();
    let mut skybox = Skybox::new();

    let mut time_scale = 1.0;
    let mut show_orbits = true;
    let mut show_ui = true;
    let mut current_target_index = usize::MAX;

    window.set_target_fps(60);
    window.hide_cursor(); // Ocultar cursor para mejor control de cámara

    println!("\n=== CONTROLES DEL SISTEMA SOLAR ===");
    println!("WASD: Movimiento de cámara libre");
    println!("Q/E: Subir/Bajar cámara");
    println!("Click derecho + mouse: Rotar cámara (arriba/abajo/izquierda/derecha)");
    println!("R/F: Acelerar/Desacelerar tiempo");
    println!("T: Mostrar/ocultar órbitas");
    println!("O: Mostrar/ocultar UI");
    println!("1-9: Teletransporte a cuerpo celeste (la cámara seguirá al planeta)");
    println!("ESPACIO: Modo libre de cámara");
    println!("ESC: Salir");

    while !window.window_should_close() {
        let delta_time = window.get_frame_time();
        
        // Actualizar sistema solar
        solar_system.update(delta_time * time_scale);
        skybox.update(delta_time);
        
        // Manejar entrada
        handle_input(&window, &mut camera, &mut time_scale, &mut show_orbits, 
                    &mut show_ui, &mut current_target_index, &solar_system);

        // Actualizar seguimiento de cámara si está siguiendo un cuerpo
        if current_target_index < solar_system.bodies.len() {
            let body = &solar_system.bodies[current_target_index];
            camera.update_following(body.position);
        }

        // Renderizar
        fb.clear();
        skybox.render(&mut fb, &camera);
        solar_system.render(&mut fb, &camera, show_orbits);

        // Actualizar textura
        if fb.texture.is_none() {
            fb.init_texture(&mut window, &thread);
        }

        if let Some(tex) = &mut fb.texture {
            let pixels: Vec<Color> = fb.color_buffer.get_image_data().to_vec();
            let mut raw: Vec<u8> = Vec::with_capacity(pixels.len() * 4);
            for c in pixels {
                raw.push(c.r);
                raw.push(c.g);
                raw.push(c.b);
                raw.push(c.a);
            }

            tex.update_texture_rec(
                Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: tex.width() as f32,
                    height: tex.height() as f32,
                },
                &raw,
            );

            let mut d = window.begin_drawing(&thread);
            d.clear_background(Color::BLACK);
            d.draw_texture(tex, 0, 0, Color::WHITE);
            
            if show_ui {
                draw_ui(&mut d, &solar_system, time_scale, show_orbits, 
                       current_target_index, &camera);
            }
        }
    }

    println!("\n¡Simulación completada!");
}

fn handle_input(
    window: &RaylibHandle,
    camera: &mut Camera,
    time_scale: &mut f32,
    show_orbits: &mut bool,
    show_ui: &mut bool,
    current_target_index: &mut usize,
    solar_system: &SolarSystem,
) {
    let delta_time = window.get_frame_time();
    
    camera.handle_input(window, delta_time);

    // Control de escala de tiempo
    if window.is_key_down(KeyboardKey::KEY_R) {
        *time_scale = (*time_scale * 1.1).min(10.0);
    }
    if window.is_key_down(KeyboardKey::KEY_F) {
        *time_scale = (*time_scale / 1.1).max(0.1);
    }

    // Teletransporte a cuerpos celestes (1-9)
    for key in [KeyboardKey::KEY_ONE, KeyboardKey::KEY_TWO, KeyboardKey::KEY_THREE, 
                KeyboardKey::KEY_FOUR, KeyboardKey::KEY_FIVE, KeyboardKey::KEY_SIX,
                KeyboardKey::KEY_SEVEN, KeyboardKey::KEY_EIGHT, KeyboardKey::KEY_NINE] {
        if window.is_key_pressed(key) {
            let index = (key as i32 - KeyboardKey::KEY_ONE as i32) as usize;
            if index < solar_system.bodies.len() {
                let body = &solar_system.bodies[index];
                let offset_distance = if index == 0 { 
                    20.0 // Más distancia para el sol
                } else { 
                    8.0 + body.scale * 2.0 // Distancia proporcional al tamaño
                };
                
                camera.warp_to(body.position, offset_distance);
                *current_target_index = index;
                println!("Teletransportado a: {}", body.name);
            }
        }
    }

    // Modo libre
    if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
        camera.set_free_mode();
        *current_target_index = usize::MAX;
        println!("Modo cámara libre activado");
    }

    // Toggle órbitas
    if window.is_key_pressed(KeyboardKey::KEY_T) {
        *show_orbits = !*show_orbits;
        println!("Órbitas: {}", if *show_orbits { "ON" } else { "OFF" });
    }

    // Toggle UI
    if window.is_key_pressed(KeyboardKey::KEY_O) {
        *show_ui = !*show_ui;
        println!("UI: {}", if *show_ui { "ON" } else { "OFF" });
    }
}

fn draw_ui(
    d: &mut RaylibDrawHandle,
    solar_system: &SolarSystem,
    time_scale: f32,
    show_orbits: bool,
    current_target_index: usize,
    camera: &Camera,
) {
    let width = d.get_screen_width();
    
    // Panel principal de información
    d.draw_rectangle(10, 10, 350, 220, Color::new(0, 0, 0, 180));
    d.draw_rectangle_lines(10, 10, 350, 220, Color::WHITE);
    
    d.draw_text("SISTEMA SOLAR NEBULARIS", 20, 20, 18, Color::YELLOW);
    d.draw_text(&format!("Tiempo: {:.1}x", time_scale), 20, 50, 16, Color::WHITE);
    d.draw_text(&format!("Cuerpos: {}", solar_system.bodies.len()), 20, 70, 16, Color::WHITE);
    d.draw_text(&format!("Órbitas: {}", if show_orbits { "ON" } else { "OFF" }), 20, 90, 16, Color::WHITE);
    d.draw_text(&format!("Modo: {}", if camera.free_mode { "LIBRE" } else { "SEGUIMIENTO" }), 20, 110, 16, 
                if camera.free_mode { Color::GREEN } else { Color::BLUE });
    
    // Información del cuerpo seguido
    if current_target_index < solar_system.bodies.len() {
        let body = &solar_system.bodies[current_target_index];
        d.draw_text(&format!("Observando: {}", body.name), 20, 130, 16, Color::GREEN);
        d.draw_text(&format!("Distancia al sol: {:.1} u", body.orbit_radius), 20, 150, 14, Color::LIGHTGRAY);
        d.draw_text(&format!("Velocidad orbital: {:.2}", body.orbit_speed), 20, 170, 14, Color::LIGHTGRAY);
    } else {
        d.draw_text("Cámara en modo libre", 20, 130, 16, Color::GREEN);
    }

    // Panel de controles inferior
    d.draw_rectangle(10, 750, width - 20, 40, Color::new(0, 0, 0, 180));
    d.draw_text("WASD: Mover | Q/E: Altura | Click derecho + Mouse: Rotar | R/F: Tiempo | 1-9: Teletransporte | ESPACIO: Libre | T: Órbitas | O: UI", 
                20, 760, 14, Color::LIGHTGRAY);

    // Panel de cuerpos celestes
    let mut y_pos = 10;
    d.draw_rectangle(width - 310, 10, 300, 200, Color::new(0, 0, 0, 180));
    d.draw_rectangle_lines(width - 310, 10, 300, 200, Color::WHITE);
    d.draw_text("CUERPOS CELESTES", width - 300, 20, 16, Color::YELLOW);
    
    for (i, body) in solar_system.bodies.iter().enumerate() {
        if y_pos < 180 {
            let color = if i == current_target_index { 
                Color::GREEN 
            } else { 
                Color::WHITE 
            };
            d.draw_text(&format!("{}: {}", i + 1, body.name), width - 300, 40 + y_pos, 14, color);
            y_pos += 20;
        }
    }
}
