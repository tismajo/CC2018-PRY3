# Proyecto 3: Space Travel
Una simulación 3D interactiva del sistema solar construida en Rust con renderizado personalizado. Navega libremente por el espacio, observa planetas orbitando y explora el cosmos.

## Video funcionamiento
https://github.com/user-attachments/assets/36b2d892-bbc7-45cb-8623-6d7b8b2ed9fd

## 🚀 Características Principales

- **🌌 Vista General Completa**: Observa todo el sistema solar desde una perspectiva panorámica
- **🪐 Planetas Detallados**: 9 cuerpos celestes con texturas procedurales únicas
- **🎮 Control Intuitivo**: Múltiples modos de cámara y navegación fluida
- **⚡ Renderizado Eficiente**: Motor gráfico personalizado con Z-buffer
- **🔭 Seguimiento Orbital**: Observa los planetas orbitando alrededor del sol

## 📋 Requisitos del Sistema

- **Rust**: Versión 1.70 o superior
- **OpenGL**: Versión 3.3 o superior
- **Memoria RAM**: 4GB mínimo, 8GB recomendado
- **GPU**: Compatible con OpenGL 3.3

## 🛠️ Instalación

### 1. Clonar el Repositorio
```bash
git clone https://github.com/tu-usuario/sistema-solar-3d.git
cd sistema-solar-3d
```

### 2. Instalar Dependencias
```bash
# Instalar Rust (si no está instalado)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Actualizar Rust
rustup update
```

### 3. Compilar el Proyecto
```bash
# Compilar en modo release (recomendado)
cargo build --release

# O compilar en modo debug (para desarrollo)
cargo build
```

### 4. Ejecutar la Simulación
```bash
# Ejecutar desde código compilado
cargo run --release

# O ejecutar el binario directamente
./target/release/sistema-solar-3d
```

## 🎮 Controles de Navegación
Vista General (Modo Libre)
Q/E: Rotar izquierda/derecha alrededor del sistema

R/F: Inclinar vista arriba/abajo

FLECHAS ↑/↓: Zoom in/out

SHIFT + WASD: Movimiento rápido

ESPACIO: Volver a vista general

### Teletransporte Rápido
1: Sol (Vista panorámica)

2: Mercurio

3: Venus

4: Tierra

5: Marte

6: Ceres (cinturón de asteroides)

7: Júpiter

8: Saturno

9: Urano

0: Neptuno

### Controles Adicionales
T: Mostrar/ocultar órbitas
U: Mostrar/ocultar interfaz
Y: Mostrar/ocultar información debug
SHIFT + R/F: Aumentar/disminuir velocidad del tiempo
P: Información detallada de debug
ESC: Salir de la aplicación

## Estructura del Proyecto
sistema-solar-3d/
├── src/
│   ├── main.rs              # Punto de entrada principal
│   ├── camera.rs           # Sistema de cámara y navegación
│   ├── solar_system.rs     # Lógica del sistema solar
│   ├── framebuffer.rs      # Renderizado personalizado
│   ├── triangle.rs         # Rasterización de triángulos
│   ├── shader.rs           # Shaders procedurales
│   ├── obj_loader.rs       # Carga de modelos 3D
│   ├── line.rs             # Algoritmos de dibujo de líneas
│   ├── procedural_geometry.rs # Generación de geometría
│   ├── skybox.rs           # Fondo estelar
│   └── utils.rs            # Utilidades matemáticas
├── assets/
│   └── sphere-1.obj        # Modelo base esférico
├── Cargo.toml              # Configuración de Rust
└── README.md               # Este archivo
