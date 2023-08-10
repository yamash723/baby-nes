use nes::ppu::frame::Frame;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub mod cli;
pub mod nes;
pub mod ui;

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate arrayref;

use nes::Nes;
use sdl2::pixels::PixelFormatEnum;
use clap::Parser;

const SCALE: f32 = 3.0;
const APPLICATION_NAME: &str = "BabyNES";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    rom_file_path: String,
}

fn main() {
    // ------------------------------------------------------------
    // Args check
    // ------------------------------------------------------------
    let args = Args::parse();
    let rom_file_path = &args.rom_file_path;

    // ------------------------------------------------------------
    // Initialize UI
    // ------------------------------------------------------------
    let window_width = (nes::ppu::frame::Frame::WIDTH as f32 * SCALE) as u32;
    let window_height = (nes::ppu::frame::Frame::HIGHT as f32 * SCALE) as u32;

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window(APPLICATION_NAME, window_width, window_height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    canvas.set_scale(SCALE, SCALE).unwrap();

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGB24, 256, 240)
        .unwrap();

    // ------------------------------------------------------------
    // Initialize NES
    // ------------------------------------------------------------
    let mut nes = Nes::new(rom_file_path).unwrap();

    let render_callback = move |frame: &Frame| {
        texture.update(None, &frame.data, 256 * 3).unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    };

    let input_callback = || {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => std::process::exit(0),
                _ => { /* do nothing */ }
            }
        }
    };

    nes.run(render_callback, input_callback)
}
