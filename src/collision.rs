use crate::vector::Vec2;
use piston_window::G2dTexture as Texture;

pub struct Cube {
    pub size: Vec2,
    pub pos: Vec2,
    pub friction: f64,
    pub rotation: f64,
    pub color: [f64; 4],
    pub texture: Texture,
    pub state: crate::mob::MobState,
}

use piston_window::RenderArgs;
use graphics::{rectangle,Transformed};

impl Cube {
    pub fn new(sx: f64, sy: f64, px: f64, py: f64, texture_path: &str,
               mut window: &mut piston_window::PistonWindow) -> Cube {
        let size = Vec2::new(sx, sy);
        let pos = Vec2::new(px, py);
        let velocity = Vec2::new(0., 0.);
        let texture = crate::render::create_texture(&mut window, texture_path);
        Cube {
            size,
            pos,
            friction: 60.,
            rotation: 0.0,
            color: [1.0, 1.0, 1.0, 1.0],
            texture,
            state: crate::mob::MobState::new(),
        }
    }
    pub fn render(&mut self, camera: &crate::Camera, window: &mut piston_window::PistonWindow, e: &piston::Event) {
        let (x, y) = (self.pos.x, self.pos.y);
        let (w, h) = (self.size.x, self.size.y);
        // Do the vertical check first because most of the levels are vertical
        if y-h/2. > camera.position.y+camera.h || y+h/2. < camera.position.y ||
           x-w/2. > camera.position.x+camera.w || x+w/2. < camera.position.x {
            return
        }
        let texture = &self.texture;

        let scale = &camera.zoom;
        let offset = &camera.position;
        let (app_w, app_h) = (&camera.w, &camera.h);

        use crate::piston_window::{image,ImageSize};
        use crate::graphics::Transformed;
        window.draw_2d(e, |c, g, _| {
            image(texture,
                  c.transform
                   .trans((x-w/2.-app_w/2.)*scale+app_w/2.-offset.x,
                          (y-h/2.-app_h/2.)*scale+app_h/2.-offset.y)
                   .scale((w/texture.get_width() as f64)*scale,
                          (h/texture.get_height() as f64)*scale)
            , g);
        });
    }
    pub fn _render(&mut self, gl: &mut opengl_graphics::GlGraphics,
                  camera: &crate::Camera, col: [f32; 4], args: &RenderArgs) {
        let (x, y) = (self.pos.x, self.pos.y);

        let scale = &camera.zoom;
        let offset = &camera.position;
        let (app_w, app_h) = (&camera.w, &camera.h);

        let square = [0.0, 0.0, self.size.x, self.size.y];

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform
                             .trans((x-app_w/2.)*scale+app_w/2.-offset.x,
                                    (y-app_h/2.)*scale+app_h/2.-offset.y)
                             .rot_rad(self.rotation)
                             .trans((-self.size.x/2.)*scale, (-self.size.y/2.)*scale)
                             .scale(*scale, *scale);
            rectangle(col, square, transform, gl);
        });
    }
}

#[allow(dead_code)]
pub enum Side {
    North,
    South,
    East,
    West,
}


const BORDER_MARGIN: f64 = 7.0;
// Detects collisions on the side of object2 coming from object1
// ex. if object1 falls on top of object2 then it would be a north collision
// and we would have to run cubecollider with north as the side to detect it.
pub fn cube_collider(object1: &crate::mob::Player, object2: &Cube, side: Side) -> (bool) {
    let mut collision = false;
    
    let (px1, sx1) = (object1.pos.x,object1.size.x);
    let (py1, sy1) = (object1.pos.y,object1.size.y);
    let (px2, sx2) = (object2.pos.x,object2.size.x);
    let (py2, sy2) = (object2.pos.y,object2.size.y);

    let (n1, s1, w1, e1) = (py1-sy1/2., py1+sy1/2.,
                            px1-sx1/2., px1+sx1/2.);
    let (n2, s2, w2, e2) = (py2-sy2/2., py2+sy2/2.,
                            px2-sx2/2., px2+sx2/2.);

    match side {
        Side::North => {
            if e1 > w2 && w1 < e2 {
                if n1 < n2 && s1 > n2 && s1 < BORDER_MARGIN + n2 {
                    collision = true;
                }
            }
            
        },
        Side::South => {
            if e1 > w2 && w1 < e2 {
                if s1 > s2 && n1 < s2 && n1 > -BORDER_MARGIN + s2 {
                    collision = true;
                }
            }
        },
        Side::East => {
            if s1 > n2 && n1 < s2 {
                if e1 > e2 && w1 < e2 && w1 > -BORDER_MARGIN + e2 {
                    collision = true;
                }
            }
        },
        Side::West => {
            if s1 > n2 && n1 < s2 {
                if w1 < w2 && e1 > w2 && e1 < BORDER_MARGIN + w2 {
                    collision = true;
                }
            }
        },
    }
    collision
}