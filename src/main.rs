// main.rs

pub mod bus;
pub mod buttons;
pub mod clock;
pub mod memory;
pub mod vdp;

use bus::Bus;
use buttons::{Button, Buttons};
use clock::TClock;
use memory::Memory;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::{fs::read, env::args, thread::Builder};
use vdp::{Vdp, WIDTH, HEIGHT};
use winit::event::{Event, WindowEvent, ElementState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use z80emu::*;

const WINDOW_SCALE: usize = 2;

fn main() {
    //let version_string = format!("zconsole {} ({})", env!("VERGEN_BUILD_SEMVER"), env!("VERGEN_GIT_SHA_SHORT"));
    let version_string = "zconsole".to_string();

    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        println!("usage: {} <cart image>", args[0]);
        exit(1);
    }

    let buttons = Arc::new(Mutex::new(Buttons::new()));
    let mut clock = TClock::new(8_000_000); // TODO: actually use this
    let mut cpu = Z80CMOS::default();
    let mut memory = Memory {
        cart: [0; 32*1024],
        cart_enabled: true,
        ram: [0; 64*1024],
    };
    let cart = read(&args[1]).expect("failed to open the cart image!");
    for (i, x) in cart.iter().enumerate() {
        memory.cart[i] = *x;
    }
    let vdp = Arc::new(Mutex::new(Vdp::new()));

    let builder = Builder::new().name("cpu".to_string());
    builder.spawn({
        let mut bus = Bus { buttons: buttons.clone(), memory, vdp: vdp.clone(), reset: false };
        move || {
            cpu.reset();
            loop {
                match cpu.execute_next(&mut bus, &mut clock, Some(|_| {})) {
                    Err(BreakCause::Halt) => {
                        println!("CPU halted");
                        break;
                    },
                    _ => {}
                }
            }
        }
    }).unwrap();

    let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new((WIDTH * WINDOW_SCALE) as f64, (HEIGHT * WINDOW_SCALE) as f64);
            WindowBuilder::new()
                .with_title(version_string)
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap()
        };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // draw the current frame
        if let Event::MainEventsCleared = event {
            // update internal state and request a redraw
            let mut vdp_lock = vdp.lock().unwrap();
            vdp_lock.update();
            window.request_redraw();

            vdp_lock.draw(pixels.frame_mut());
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // handle input events
        if let Event::WindowEvent { ref event, .. } = event {
            if let WindowEvent::KeyboardInput { input, .. } = event {
                let mut buttons_lock = buttons.lock().unwrap();
                let scancode = input.scancode;
                let button = match scancode {
                    0x67 => Some(Button::Up),
                    0x6C => Some(Button::Down),
                    0x69 => Some(Button::Left),
                    0x6A => Some(Button::Right),
                    0x2C => Some(Button::Select),
                    0x2D => Some(Button::Back),
                    _ => None,
                };
                if button.is_some() {
                    match input.state {
                        ElementState::Pressed => buttons_lock.press(button.unwrap()),
                        ElementState::Released => buttons_lock.release(button.unwrap()),
                    }
                }
            }
        }

        if input.update(&event) {
            // close events
            if input.close_requested() || input.destroyed() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // resize the window
            if let Some(size) = input.window_resized() {
                _ = pixels.resize_surface(size.width, size.height);
            }
        }
    });

}
