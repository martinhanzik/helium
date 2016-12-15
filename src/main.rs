extern crate sdl2;
#[macro_use]
extern crate clap;

use std::io::Read;
use std::fs::File;
use clap::{Arg, App};

struct Line {
    rect: sdl2::rect::Rect,
    texture: sdl2::render::Texture,
}

fn main() {
    let matches = App::new("Helium")
        .version(crate_version!())
        .author(crate_authors!())
        .about("The awesomest of editors.")
        .arg(Arg::with_name("file")
            .help("Input file")
            .required(true)
            .index(1))
        .get_matches();

    let mut string = String::new();
    File::open(matches.value_of("file").unwrap())
        .expect("Failed to open file")
        .read_to_string(&mut string)
        .expect("Failed to read file.");

    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let window = sdl_context.video()
        .unwrap()
        .window("Helium", 800, 600)
        .resizable()
        .allow_highdpi()
        .build()
        .unwrap();
    let mut renderer = window.renderer()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    let font = ttf_context.load_font(std::path::Path::new("DejaVuSansMono.ttf"), 24).unwrap();

    let mut textures = vec![];

    for (index, line) in string.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let line_render = font.render(&line[..]).blended(sdl2::pixels::Color::RGB(150, 150, 150)).unwrap();
        let rect = sdl2::rect::Rect::new(0,
                                         index as i32 * line_render.height() as i32,
                                         line_render.width(),
                                         line_render.height());
        let texture = renderer.create_texture_from_surface(&line_render).unwrap();
        textures.push(Line {
            rect: rect,
            texture: texture,
        });
    }


    renderer.set_draw_color(sdl2::pixels::Color::RGB(30, 35, 40));

    for event in sdl_context.event_pump().unwrap().wait_iter() {
        match event {
            sdl2::event::Event::Quit { .. } => return,
            sdl2::event::Event::Window { .. } => {
                renderer.clear();
                for line in textures.iter_mut() {
                    renderer.copy(&mut line.texture, None, Some(line.rect)).unwrap();
                }
                renderer.present();
            }
            _ => {}
        }
    }
}
