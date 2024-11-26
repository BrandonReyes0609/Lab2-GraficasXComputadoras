mod framebuffer;
use framebuffer::Framebuffer;
use pixels::{Pixels, SurfaceTexture};
use rand::Rng;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const SCALE: usize = 10; // Escala para agrandar las figuras
const DEAD_COLOR: u32 = 0x000000FF;  // Color negro para las células muertas (opaco)

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Conway's Game of Life")
        .with_inner_size(LogicalSize::new(WIDTH as f64, HEIGHT as f64))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap();

    let scaled_width = WIDTH / SCALE;
    let scaled_height = HEIGHT / SCALE;
    let mut framebuffer = Framebuffer::new(scaled_width, scaled_height);

    initialize_random_particles(&mut framebuffer);
    println!("Framebuffer dimensions: {}x{}", framebuffer.width, framebuffer.height);
    println!("{:?}", &framebuffer.buffer[0..10]); // Imprime los primeros 10 valores

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        winit::event::KeyboardInput {
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // Actualizar el estado del framebuffer
                update_game_of_life(&mut framebuffer);

                // Obtener el buffer de píxeles
                let frame = pixels.get_frame_mut();

                // Transferir los datos del framebuffer al buffer de píxeles
                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = (i % WIDTH) / SCALE;
                    let y = (i / WIDTH) / SCALE;
                    let color = framebuffer.buffer[y * scaled_width + x];

                    // Convertir de formato ARGB a RGBA
                    pixel[0] = ((color >> 16) & 0xFF) as u8; // Rojo
                    pixel[1] = ((color >> 8) & 0xFF) as u8;  // Verde
                    pixel[2] = (color & 0xFF) as u8;         // Azul
                    pixel[3] = 255;                          // Alfa opaco
                }

                // Renderizar los píxeles
                if pixels.render().is_err() {
                    eprintln!("Error rendering pixels!");
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}

fn initialize_random_particles(framebuffer: &mut Framebuffer) {
    let mut rng = rand::thread_rng();
    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // 30% de probabilidad de estar viva
            if rng.gen_bool(0.3) {
                framebuffer.set_pixel(x, y, generate_random_color());
            } else {
                framebuffer.set_pixel(x, y, DEAD_COLOR);
            }
        }
    }
}

fn update_game_of_life(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let mut new_buffer = vec![DEAD_COLOR; width * height];

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let live_neighbors = count_live_neighbors(framebuffer, x, y);
            new_buffer[idx] = match (framebuffer.buffer[idx] != DEAD_COLOR, live_neighbors) {
                (true, 2) | (true, 3) => framebuffer.buffer[idx], // Sobrevive con el mismo color
                (false, 3) => generate_random_color(), // Reproducción con un nuevo color
                _ => DEAD_COLOR, // Muere
            };
        }
    }

    framebuffer.buffer = new_buffer;
}

fn count_live_neighbors(framebuffer: &Framebuffer, x: usize, y: usize) -> u8 {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
            let ny = (y as isize + dy).rem_euclid(height as isize) as usize;
            if framebuffer.buffer[ny * width + nx] != DEAD_COLOR {
                count += 1;
            }
        }
    }

    count
}

fn generate_random_color() -> u32 {
    let mut rng = rand::thread_rng();
    // Generar un color RGB aleatorio con alfa fijo (0xFF para opaco)
    let r = rng.gen_range(0..256) as u32;
    let g = rng.gen_range(0..256) as u32;
    let b = rng.gen_range(0..256) as u32;
    (0xFF << 24) | (r << 16) | (g << 8) | b // Formato ARGB
}
