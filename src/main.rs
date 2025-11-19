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

    // FONDO MUY OSCURO para mejor contraste con estrellas
    let mut fb = Framebuffer::new(1200, 800, Color::new(5, 5, 15, 255));

    println!("Cargando modelos...");
    let model_sphere = ObjModel::load("sphere-1.obj")
        .expect("No se pudo cargar sphere-1.obj");
    println!("✓ Modelos cargados correctamente");

    let mut solar_system = SolarSystem::new();
    solar_system.initialize_system(&model_sphere);

    let mut camera = Camera::new();
    let mut skybox = Skybox::new();

    // DEBUG: Verificar que el skybox se creó correctamente
    println!("Skybox inicializado con {} estrellas", skybox.stars.len());

    let mut time_scale = 1.0;
    let mut show_orbits = true;
    let mut show_ui = true;

    window.set_target_fps(60);

    println!("\n=== CONTROLES ===");
    println!("WASD: Movimiento libre");
    println!("FLECHAS ↑/↓: Subir/Bajar");
    println!("Q/E: Rotar vista horizontal");
    println!("R/F: Rotar vista vertical");
    println!("R/F (mantener): Velocidad tiempo +/-");
    println!("T: Mostrar/ocultar órbitas");
    println!("1-9: Teletransporte a cuerpo");
    println!("ESPACIO: Modo libre");
    println!("ESC: Salir");

    while !window.window_should_close() {
        let delta_time = window.get_frame_time();
        
        // ACTUALIZACIONES
        solar_system.update(delta_time * time_scale);
        skybox.update(delta_time);
        
        // ENTRADA - siempre procesar input
        handle_input(&window, &mut camera, &mut time_scale, &mut show_orbits, 
                    &mut show_ui, &solar_system);

        // ACTUALIZAR SEGUIMIENTO - siempre llamar
        camera.update_following(&solar_system);

        // RENDERIZADO
        fb.clear();
        
        // 1. Skybox primero (fondo)
        skybox.render(&mut fb, &camera);
        
        // 2. Sistema solar después
        solar_system.render(&mut fb, &camera, show_orbits);

        // DIBUJADO EN VENTANA
        display_framebuffer(&mut window, &thread, &mut fb, &solar_system, 
                           time_scale, show_orbits, show_ui, &camera);
    }

    println!("¡Simulación terminada!");
}

fn handle_input(
    window: &RaylibHandle,
    camera: &mut Camera,
    time_scale: &mut f32,
    show_orbits: &mut bool,
    show_ui: &mut bool,
    solar_system: &SolarSystem,
) {
    let delta_time = window.get_frame_time();
    
    // MOVIMIENTO - siempre procesar
    camera.handle_input(window, delta_time, solar_system);

    // CONTROL TIEMPO
    if window.is_key_down(KeyboardKey::KEY_R) && window.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
        *time_scale = (*time_scale * 1.1).min(20.0);
    }
    if window.is_key_down(KeyboardKey::KEY_F) && window.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
        *time_scale = (*time_scale / 1.1).max(0.05);
    }

    // TELETRANSPORTE - verificar cada tecla
    let number_keys = [
        (KeyboardKey::KEY_ONE, 0),
        (KeyboardKey::KEY_TWO, 1),
        (KeyboardKey::KEY_THREE, 2),
        (KeyboardKey::KEY_FOUR, 3),
        (KeyboardKey::KEY_FIVE, 4),
        (KeyboardKey::KEY_SIX, 5),
        (KeyboardKey::KEY_SEVEN, 6),
        (KeyboardKey::KEY_EIGHT, 7),
        (KeyboardKey::KEY_NINE, 8),
    ];

    for (key, index) in number_keys.iter() {
        if window.is_key_pressed(*key) {
            println!("Tecla {} presionada - intentando teletransporte a índice {}", index + 1, index);
            if *index < solar_system.bodies.len() {
                let body = &solar_system.bodies[*index];
                let offset_distance = if *index == 0 { 
                    30.0 
                } else { 
                    12.0 + body.scale * 3.0 
                };
                
                println!("PRESIONADA TECLA {} - Teletransportando a {}", index + 1, body.name);
                camera.warp_to(body.position, *index, offset_distance);
            }
        }
    }

    // MODO LIBRE
    if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
        camera.set_free_mode();
    }

    // TOGGLES
    if window.is_key_pressed(KeyboardKey::KEY_T) {
        *show_orbits = !*show_orbits;
        println!("Órbitas: {}", if *show_orbits { "VISIBLES" } else { "OCULTAS" });
    }
    if window.is_key_pressed(KeyboardKey::KEY_O) {
        *show_ui = !*show_ui;
        println!("UI: {}", if *show_ui { "VISIBLE" } else { "OCULTA" });
    }
}

fn display_framebuffer(
    window: &mut RaylibHandle,
    thread: &RaylibThread,
    fb: &mut Framebuffer,
    solar_system: &SolarSystem,
    time_scale: f32,
    show_orbits: bool,
    show_ui: bool,
    camera: &Camera,
) {
    // Inicializar textura si es necesario
    if fb.texture.is_none() {
        fb.init_texture(window, thread);
    }

    // Actualizar textura
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
            draw_ui(&mut d, solar_system, time_scale, show_orbits, camera);
        }
    }
}

fn draw_ui(
    d: &mut RaylibDrawHandle,
    solar_system: &SolarSystem,
    time_scale: f32,
    show_orbits: bool,
    camera: &Camera,
) {
    let width = d.get_screen_width();
    
    // Panel información
    d.draw_rectangle(10, 10, 300, 180, Color::new(0, 0, 0, 180));
    d.draw_rectangle_lines(10, 10, 300, 180, Color::WHITE);
    
    d.draw_text("SISTEMA SOLAR", 20, 20, 18, Color::YELLOW);
    d.draw_text(&format!("Tiempo: {:.1}x", time_scale), 20, 45, 16, Color::WHITE);
    d.draw_text(&format!("Cuerpos: {}", solar_system.bodies.len()), 20, 65, 16, Color::WHITE);
    d.draw_text(&format!("Órbitas: {}", if show_orbits { "ON" } else { "OFF" }), 20, 85, 16, Color::WHITE);
    
    let mode_text = if camera.free_mode { "LIBRE" } else { "SEGUIMIENTO" };
    let mode_color = if camera.free_mode { Color::GREEN } else { Color::BLUE };
    d.draw_text(&format!("Modo: {}", mode_text), 20, 105, 16, mode_color);
    
    if let Some(target_index) = camera.following_target {
        if target_index < solar_system.bodies.len() {
            let body = &solar_system.bodies[target_index];
            d.draw_text(&format!("Observando: {}", body.name), 20, 125, 14, Color::GREEN);
        }
    }

    // Controles
    d.draw_rectangle(10, 750, width - 20, 40, Color::new(0, 0, 0, 180));
    d.draw_text("WASD: Mover | FLECHAS: Vertical | Q/E: Rotar | 1-9: Teletransporte | ESPACIO: Libre", 
                15, 760, 14, Color::LIGHTGRAY);

    // Lista cuerpos
    d.draw_rectangle(width - 250, 10, 240, 150, Color::new(0, 0, 0, 180));
    d.draw_rectangle_lines(width - 250, 10, 240, 150, Color::WHITE);
    d.draw_text("CUERPOS:", width - 240, 20, 16, Color::YELLOW);
    
    for (i, body) in solar_system.bodies.iter().enumerate().take(6) {
        let y_pos = 40 + (i as i32) * 20;
        let color = if Some(i) == camera.following_target { 
            Color::GREEN 
        } else { 
            Color::WHITE 
        };
        d.draw_text(&format!("{}: {}", i + 1, body.name), width - 240, y_pos, 12, color);
    }
}
