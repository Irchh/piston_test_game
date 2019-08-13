use crate::App;
use piston::RenderArgs;
use graphics::{rectangle,Transformed};
use piston_window::PistonWindow;
use piston_window::G2dTexture as Texture;

pub struct PlayerAnimation {
    pub stand: Texture,
    pub walk1: Texture,
    pub walk2: Texture,
    pub walk3: Texture,
    pub jump: Texture,
    pub land: Texture,
}

impl PlayerAnimation {
    pub fn new(mut window: &mut PistonWindow, texture_vector: &Vec<&str>) -> PlayerAnimation {
        let stand = create_texture(&mut window, texture_vector[0]);
        let walk1 = create_texture(&mut window, texture_vector[1]);
        let walk2 = create_texture(&mut window, texture_vector[2]);
        let walk3 = create_texture(&mut window, texture_vector[3]);
        let jump = create_texture(&mut window, texture_vector[4]);
        let land = create_texture(&mut window, texture_vector[5]);
        PlayerAnimation {
            stand,
            walk1,
            walk2,
            walk3,
            jump,
            land,
        }
    }
}

pub fn create_texture(window: &mut PistonWindow, texture_path: &str) -> Texture {
    Texture::from_path(
        &mut window.create_texture_context(),
        &texture_path,
        piston_window::Flip::None,
        &piston_window::TextureSettings::new()
    ).unwrap()
}