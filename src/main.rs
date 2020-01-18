extern crate reqwest;
extern crate rss;
extern crate sdl2;

pub mod graphics;
pub mod items;

fn main() -> Result<(), String> {
    let mut g = graphics::Renderer::new()?;
    g.render()?;
    Ok(())
}
