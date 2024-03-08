#![windows_subsystem = "windows"]
use std::num::NonZeroU32;

use softbuffer::{Context, Surface};
use soloud::{audio, AudioExt, LoadExt, Soloud};
use tiny_skia::{Color, Paint, Pixmap, PixmapPaint, Transform};
use winit::{dpi::LogicalSize, event::{Event, MouseButton, WindowEvent}, event_loop::EventLoop, keyboard::{Key, NamedKey}, platform::windows::WindowBuilderExtWindows, window::{WindowBuilder, WindowButtons}};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::default()
        .with_inner_size(LogicalSize::new(128, 128))
        .with_title("Cirno by Arrokoth233")
        .with_decorations(false)
        .with_skip_taskbar(true)
        .with_resizable(false)
        .with_enabled_buttons(WindowButtons::CLOSE)
        .with_transparent(true)
        .with_visible(false)
        .with_window_level(winit::window::WindowLevel::AlwaysOnTop)
        .build(&event_loop).unwrap();

    let context = Context::new(&window).unwrap();
    let mut surface = Surface::new(&context, &window).unwrap();
    let mut pixmap = Pixmap::new((128 as f64 * window.scale_factor()) as u32, (128 as f64 * window.scale_factor()) as u32).unwrap();
    let pixmap_btn = Pixmap::decode_png(include_bytes!("./cirno.png")).unwrap();
    let mut paint = Paint::default();
    paint.anti_alias = true;

    let mut pressing = false;

    let sl = Soloud::default().unwrap();
    let mut ba = audio::Wav::default();
    ba.load_mem(include_bytes!("./ba.wav")).unwrap();
    let mut ka = audio::Wav::default();
    ka.load_mem(include_bytes!("./ka.wav")).unwrap();
    
    event_loop.run(|event, elwt| {
        match event {
            Event::Resumed => {
                window.set_visible(true);
            },
            Event::WindowEvent {
                window_id, 
                ref event
            } => {
                if window.id() == window_id {
                    match event {
                        WindowEvent::KeyboardInput { 
                            device_id: _, 
                            event, 
                            is_synthetic: _
                        } => {
                            if event.logical_key == Key::Named(NamedKey::Escape) {
                                elwt.exit();
                            }
                        }
                        WindowEvent::MouseInput {
                            device_id: _, 
                            state, 
                            button
                        } => {
                            if state.is_pressed() {
                                if button == &MouseButton::Left {
                                    window.drag_window().unwrap();
                                }

                                if button == &MouseButton::Left {
                                    sl.stop_all();
                                    sl.play(&ba);
                                    pressing = true;
                                    window.request_redraw();
                                }
                            } else {
                                if button == &MouseButton::Left {
                                    sl.stop_all();
                                    sl.play(&ka);
                                    pressing = false;
                                    window.request_redraw();
                                }
                            }
                        }
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            pixmap.fill(Color::TRANSPARENT);
                            let scale_factor: f32 = window.scale_factor() as f32;
                            pixmap = Pixmap::new((128 as f32 * scale_factor) as u32, (128 as f32 * scale_factor) as u32).unwrap();

                            let mut transf = if pressing {
                                let x = (window.inner_size().width as f32 - window.inner_size().width as f32 * 0.9) / 2.0;
                                Transform::identity().pre_translate(x / scale_factor, x / scale_factor).pre_scale(0.9, 0.9)
                            } else {
                                Transform::identity().pre_scale(1.0, 1.0)
                            };
                            transf = transf.post_scale(scale_factor, scale_factor);

                            pixmap.draw_pixmap(0, 0, pixmap_btn.as_ref(), &PixmapPaint {
                                opacity: 1.0,
                                blend_mode: tiny_skia::BlendMode::Color,
                                quality: tiny_skia::FilterQuality::Bicubic,
                            }, transf, None);

                            // 渲染
                            surface.resize(NonZeroU32::new(window.inner_size().width).unwrap(), NonZeroU32::new(window.inner_size().height).unwrap()).unwrap();
                            let mut buffer = surface.buffer_mut().unwrap();
                            let data = pixmap.data();
                            let (width, height) = (pixmap.width(), pixmap.height());
                            let buffer_ref = &mut buffer;
                    
                            for i in 0..(width * height) as usize {
                                buffer_ref[i] = (data[i * 4 + 2] as u32)
                                    | (data[i * 4 + 1] as u32) << 8
                                    | (data[i * 4 + 0] as u32) << 16
                                    | (data[i * 4 + 3] as u32) << 24;
                            }

                            buffer.present().unwrap();
                        }
                        _ => {
                            // window.focus_window();
                        }
                    }
                }
            }
            _ => {}
        }
    }).unwrap();
}
