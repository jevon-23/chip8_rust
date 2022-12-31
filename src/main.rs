use std::fs::File;
use std::io::Read;
use std::fs;
use std::env;
use std::process;
mod memory;
mod display;
mod cpu;

use winit::{
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
/* Pixels imports */
use log::error;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit_input_helper::WinitInputHelper;

const WIDTH: u32 = 220;
const HEIGHT: u32 = 140;

fn process_args(args : &Vec<String>) {
    if args.len() != 2 {
        println!("Not enough arguments");
        process::exit(-1);
    }
}

/* https://www.reddit.com/r/rust/comments/dekpl5/how_to_read_binary_data_from_a_file_into_a_vecu8/ */
fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    return buffer;
}

fn main() -> Result<(), Error>{
    let args : Vec<String> = env::args().collect();
    process_args(&args);

    /* Get the file path */
    let file_path : &String = &args[1];
    dbg!(file_path);

    let game : Vec<u8> = get_file_as_byte_vec(file_path);

    println!("Hello, world!");
    let mut mem: memory::Mem = memory::make_memory();
    mem.store_game(game);

    println!("Mem data_len: {}", mem.data.len());

    let mut _c : cpu::CPU = cpu::make_cpu(mem);

    /*
       https://github.com/parasyte/pixels/blob/864a9c3491cb2aa778a8c0ae5742f760bcfac622/examples/minimal-winit/src/main.rs
       */
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let world = cpu::World::new();
    let mut num_instructions_finished : u16 = 0;
    let mut sys_timer : cpu::Timer = cpu::Timer::new();
    event_loop.run(move |event, _, control_flow| {
        let mut input_key : u8 =  0xf0;
        _c.sound_timer.tick();
        _c.delay_timer.tick();

        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(&_c, pixels.get_frame_mut());
            if let Err(err) = pixels.render() {
                error!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            } else if input.key_pressed(VirtualKeyCode::Key1) || input.key_held(VirtualKeyCode::Key1) {
                println!("1 is button is pushed");
                input_key = 0x01;
            } else if input.key_pressed(VirtualKeyCode::Key2) || input.key_held(VirtualKeyCode::Key2){
                println!("2 is button is pushed");
                input_key = 0x02;
            } else if input.key_pressed(VirtualKeyCode::Key3) || input.key_held(VirtualKeyCode::Key3) {
                println!("3 is button is pushed");
                input_key = 0x03;
            } else if input.key_pressed(VirtualKeyCode::Key4) || input.key_held(VirtualKeyCode::Key4) {
                println!("4 is button is pushed");
                input_key = 0x0C;
            } else if input.key_pressed(VirtualKeyCode::Q) || input.key_held(VirtualKeyCode::Q) {
                println!("Q is button is pushed");
                input_key = 0x04;
            } else if input.key_pressed(VirtualKeyCode::W) || input.key_held(VirtualKeyCode::W) {
                println!("W is button is pushed");
                input_key = 0x05;
            } else if input.key_pressed(VirtualKeyCode::E) || input.key_held(VirtualKeyCode::E) {
                println!("E is button is pushed");
                input_key = 0x06;
            } else if input.key_pressed(VirtualKeyCode::R) || input.key_held(VirtualKeyCode::R) {
                println!("R is button is pushed");
                input_key = 0x0D;
            } else if input.key_pressed(VirtualKeyCode::A) || input.key_held(VirtualKeyCode::A) {
                println!("A is button is pushed");
                input_key = 0x07;
            } else if input.key_pressed(VirtualKeyCode::S) || input.key_held(VirtualKeyCode::S) {
                println!("S is button is pushed");
                input_key = 0x08;
            } else if input.key_pressed(VirtualKeyCode::D) || input.key_held(VirtualKeyCode::D) {
                println!("D is button is pushed");
                input_key = 0x09;
            } else if input.key_pressed(VirtualKeyCode::F) || input.key_held(VirtualKeyCode::F) {
                println!("F is button is pushed");
                input_key = 0x0e;
            } else if input.key_pressed(VirtualKeyCode::Z) || input.key_held(VirtualKeyCode::Z) {
                println!("Z is button is pushed");
                input_key = 0x0a;
            } else if input.key_pressed(VirtualKeyCode::X) || input.key_held(VirtualKeyCode::X) {
                println!("X is button is pushed");
                input_key = 0x00;
            } else if input.key_pressed(VirtualKeyCode::C) || input.key_held(VirtualKeyCode::C) {
                println!("C is button is pushed");
                input_key = 0x0b;
            } else if input.key_pressed(VirtualKeyCode::V) || input.key_held(VirtualKeyCode::V) {
                println!("V is button is pushed");
                input_key = 0x0f;
            }         

        // Resize the window
        if let Some(size) = input.window_resized() {
            if let Err(err) = pixels.resize_surface(size.width, size.height) {
                error!("pixels.resize_surface() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
        // println!("input key: {:#01x}", input_key);
        // Update internal state and request a redraw
        if num_instructions_finished % 700 == 0 {
            if sys_timer.get_elapsed_time() > sys_timer.curr_second {
                sys_timer.curr_second = sys_timer.get_elapsed_time();
                _c.exec(input_key);
                num_instructions_finished += 1;
            }
        } else {
            _c.exec(input_key);
            num_instructions_finished += 1;
        }
        window.request_redraw();
    }

});


}
