extern crate conrod_core;
extern crate piston_window;
extern crate conrod_piston;
extern crate find_folder;
extern crate rand;
mod app;
mod logic;
mod event;

use self::piston_window::{
    texture::UpdateTexture, G2d, G2dTexture, OpenGL, PistonWindow, TextureSettings,
    UpdateEvent, Window, WindowSettings,
};

fn main() {
    let mut data = app::Data::new();
    let mut game = app::Game::new();
    game.populate_hash_map();
    // for (key, val) in  game.strings.iter() {
    //     println!("key: {} val: {} {}", key, val.0, val.1);
    // }
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 400;
    let title = "TypeSpeed";

    let font_path = app::load_font("UbuntuMono-R.ttf");
    let mut ui = conrod_core::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();
    ui.fonts.insert_from_file(font_path).unwrap();
    let ids = app::Ids::new(ui.widget_id_generator());


    let mut window: PistonWindow = WindowSettings::new(title, [WIDTH, HEIGHT])
        .opengl(OpenGL::V3_2)
        .samples(4)
        .exit_on_esc(true)
        .vsync(true)
        .build()
        .unwrap();

    let (mut glyph_cache, mut text_texture_cache) = {
        const SCALE_TOLERANCE: f32 = 0.1;
        const POSITION_TOLERANCE: f32 = 0.1;
        let cache = conrod_core::text::GlyphCache::builder()
            .dimensions(WIDTH, HEIGHT)
            .scale_tolerance(SCALE_TOLERANCE)
            .position_tolerance(POSITION_TOLERANCE)
            .build();
        
        let buffer_len = WIDTH as usize * HEIGHT as usize;
        let init = vec![128; buffer_len];
        let settings = TextureSettings::new();
        let factory = &mut window.factory;
        let texture =
            G2dTexture::from_memory_alpha(
                factory,
                &init,
                WIDTH,
                HEIGHT,
                &settings
            )
            .unwrap();

        (cache, texture)
    };
    
    // Create the image map for conrod
    let image_map = conrod_core::image::Map::new();
    
    // Create a texture to use for efficiently caching text on the GPU.
    let mut text_vertex_data = Vec::new();


    while let Some(event) = window.next() {
        // Handle window resizing
        let size = window.size();

        let (win_w, win_h) = (
            size.width as conrod_core::Scalar,
            size.height as conrod_core::Scalar,
        );

        // Let our UI handle events
        if let Some(e) = event::convert(event.clone(), win_w, win_h) {
            ui.handle_event(e);
        }

        // update our UI state
        event.update(|_| logic::update(ui.set_widgets(), &ids, &mut game, &mut data));

        // draw our UI
        window.draw_2d(&event, |context, graphics| {
            if let Some(primitives) = ui.draw_if_changed() {
                // A function used for caching glyphs to the texture cache.
                let cache_queued_glyphs = |graphics: &mut G2d,
                                           cache: &mut G2dTexture,
                                           rect: conrod_core::text::rt::Rect<u32>,
                                           data: &[u8]| {
                    let offset = [rect.min.x, rect.min.y];
                    let size = [rect.width(), rect.height()];
                    let format = piston_window::texture::Format::Rgba8;
                    let encoder = &mut graphics.encoder;
                    text_vertex_data.clear();
                    text_vertex_data.extend(data.iter().flat_map(|&b| vec![255, 255, 255, b]));
                    UpdateTexture::update(
                        cache,
                        encoder,
                        format,
                        &text_vertex_data[..],
                        offset,
                        size,
                    )
                    .expect("failed to update texture")
                };

                // Specify how to get the drawable texture from the image. In this case, the image
                // *is* the texture.
                fn texture_from_image<T>(img: &T) -> &T {
                    img
                }

                // Draw the conrod `render::Primitives`.
                conrod_piston::draw::primitives(
                    primitives,
                    context,
                    graphics,
                    &mut text_texture_cache,
                    &mut glyph_cache,
                    &image_map,
                    cache_queued_glyphs,
                    texture_from_image,
                );
            }
        });
    }
}