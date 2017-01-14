#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]


extern crate glium;
extern crate glium_text;
extern crate cgmath;
#[macro_use]
extern crate clap;

use std::io::Read;
use std::fs::File;
use std::time::Duration;
use std::thread;
use clap::{Arg, App};

use glium::{DisplayBuild, Surface};
use glium::glutin;

fn main() {
    let matches = App::new("Helium")
        .version(crate_version!())
        .author(crate_authors!())
        .about("The awesomest of editors.")
        .arg(Arg::with_name("file")
            .help("Input file")
            .index(1))
        .get_matches();

    let mut string = String::new();

    if let Some(file) = matches.value_of("file") {
        println!("Reading file: {}", file);
        File::open(file)
            .expect("Failed to open file")
            .read_to_string(&mut string)
            .expect("Failed to read file.");
    }


    let display = glutin::WindowBuilder::new()
        .with_vsync()
        .with_dimensions(800, 600)
        .with_title("Helium")
        .build_glium()
        .unwrap();
    let system = glium_text::TextSystem::new(&display);

    let font = glium_text::FontTexture::new(&display, &include_bytes!("../DejaVuSansMono.ttf")[..], 70).unwrap();

    let mut buffer = String::new();

    let sleep_duration = Duration::from_millis(17);

    // let mut text: String = "A japanese poem:\r
    // \r
    // 色は匂へど散りぬるを我が世誰ぞ常ならむ有為の奥山今日越えて浅き夢見じ酔ひもせず\r
    // \r
    // Feel free to type out some text, and delete it with Backspace. You can also try resizing this window."
    //         .into();

    'main: loop {

        let text = glium_text::TextDisplay::new(&system, &font, &buffer);

        let (w, h) = display.get_framebuffer_dimensions();

        let matrix: [[f32; 4]; 4] = cgmath::Matrix4::new(0.1,
                                                         0.0,
                                                         0.0,
                                                         0.0,
                                                         0.0,
                                                         0.1 * (w as f32) / (h as f32),
                                                         0.0,
                                                         0.0,
                                                         0.0,
                                                         0.0,
                                                         1.0,
                                                         0.0,
                                                         -0.9,
                                                         0.0,
                                                         0.0,
                                                         1.0f32)
            .into();

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        glium_text::draw(&text, &system, &mut target, matrix, (1.0, 1.0, 0.0, 1.0));
        target.finish().unwrap();

        thread::sleep(sleep_duration);


        for event in display.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                glutin::Event::ReceivedCharacter(c) => {
                    if c != '\u{7f}' && c != '\u{8}' {
                        buffer.push(c);
                    }
                }
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Back)) => {
                    buffer.pop();
                }
                _ => {}
            }
        }
    }
}
