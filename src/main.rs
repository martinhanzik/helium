extern crate sdl2;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap();
    let window = sdl_context.video()
        .unwrap()
        .window("Helium", 800, 600)
        .resizable()
        .build()
        .unwrap();
    let mut renderer = window.renderer()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();
    let font = ttf_context.load_font(std::path::Path::new("DejaVuSansMono.ttf"), 16).unwrap();
    let hello =
        font.render("Hello world!").blended(sdl2::pixels::Color::RGB(150, 150, 150)).unwrap();
    let mut hello_texture = renderer.create_texture_from_surface(&hello).unwrap();
    renderer.set_draw_color(sdl2::pixels::Color::RGB(30, 35, 40));

    for event in sdl_context.event_pump().unwrap().wait_iter() {
        match event {
            sdl2::event::Event::Quit { .. } => return,
            sdl2::event::Event::Window { .. } => {
                renderer.clear();
                renderer.copy(&mut hello_texture, None, None).unwrap();
                renderer.present();
            }
            _ => {}
        }
    }
}
