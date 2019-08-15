#![feature(clamp)]

extern crate piston;
extern crate graphics;
extern crate piston_window;
extern crate opengl_graphics;
extern crate roxmltree;

mod vector;
mod collision;
mod world;
mod render;
mod mob;
mod loader;

use collision::Cube;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use piston_window::PistonWindow;
use opengl_graphics::{ GlGraphics, OpenGL };

const METER_IN_PIXELS: f64 = 50.0;

pub struct Keys {
    space: bool,
    a: bool,
    d: bool,
    s: bool,
    w: bool,
}

impl Keys {
    fn new() -> Keys {
        Keys {
            space: false,
            a: false,
            d: false,
            s: false,
            w: false,
        }
    }
}

pub struct World {
    grav_const: f64,
    w: f64,
    h: f64,
}

pub struct Camera {
    position: vector::Vec2,
    zoom: f64,
    w: f64,
    h: f64,
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    mobs: Vec<collision::Cube>,
    obj: Vec<collision::Cube>,
    players: Vec<mob::Player>,
    keystate: Keys,
    world: World,
    camera: Camera,
}

impl App {
    fn render(&mut self, args: &RenderArgs, e: &Event, window: &mut PistonWindow) {
        use graphics::*;

        #[allow(dead_code)]
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        #[allow(dead_code)]
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        #[allow(dead_code)]
        const BLUE:  [f32; 4] = [0.0, 0.0, 1.0, 1.0];
        #[allow(dead_code)]
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

		self.gl.draw(args.viewport(), |_c, gl| {
	        clear(GREEN, gl);
	    });

        for obj in &mut self.obj {
            //obj._render(&mut self.gl, &self.camera, RED, args);
            obj.render(&self.camera, window, e);
        }

        for mob in &mut self.mobs {
            //mob._render(&mut self.gl, &self.camera, BLUE, args);
            mob.render(&self.camera, window, e);
        }

        for player in &mut self.players {
            player.render(&self.camera, window, e);
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        for player in &mut self.players {
            player.update(&mut self.keystate, &self.world, &mut self.camera, &self.obj, args);
        }

        // This can lag the game quite a bit, or atleast slow down the updates.
        if self.players.len() > 0 && false { 
            let _true_velocity = (self.players[0].velocity.y*self.players[0].velocity.y+self.players[0].velocity.x*self.players[0].velocity.x).sqrt();
            print!("\rVelocity: {:.2} m/s  ", _true_velocity as f32);
            print!("Animation length: {:.2}  ", self.players[0].ani_length);
            print!("W/H: {:.2}/{:.2}   ", self.camera.w, self.camera.h);
        }
    }
    fn btn_press(&mut self, key: &piston::Button) {
        if key == &Button::Keyboard(Key::Space) { self.keystate.space = true; }
        if key == &Button::Keyboard(Key::A) { self.keystate.a = true; }
        if key == &Button::Keyboard(Key::D) { self.keystate.d = true; }
        if key == &Button::Keyboard(Key::S) { self.keystate.s = true; }
        if key == &Button::Keyboard(Key::W) { self.keystate.w = true; }
        //println!("\nPressed keyboard key '{:?}'", key);
    }
    fn btn_release(&mut self, key: &piston::Button) {
        if key == &Button::Keyboard(Key::Space) { self.keystate.space = false; }
        if key == &Button::Keyboard(Key::A) { self.keystate.a = false; }
        if key == &Button::Keyboard(Key::D) { self.keystate.d = false; }
        if key == &Button::Keyboard(Key::S) { self.keystate.s = false; }
        if key == &Button::Keyboard(Key::W) { self.keystate.w = false; }
        //println!("\nReleased keyboard key '{:?}'", key);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a window.
    let mut window: PistonWindow = WindowSettings::new(
            "spinning-square",
            [800, 600]
        )
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        mobs: Vec::new(),
        obj: Vec::new(),
        players: Vec::new(),
        keystate: Keys::new(),
        world: World {
            grav_const: 9.807,
            w: 0.,
            h: 0.,
        },
        camera: Camera {
            position: vector::Vec2::new(0.0, 0.0),
            zoom: 1.0,
            w: 0.,
            h: 0.,
        },
    };

    let cube1 = Cube::new(32., 32., 200., 200., "assets/sprites/brick.png", &mut window);
    let cube2 = Cube::new(32., 32., 232., 200., "assets/sprites/brick.png", &mut window);

    app.mobs.push(cube1);
    app.mobs.push(cube2);

    let map = loader::load_map("assets/maps/test.tmx", &mut app, &mut window);

    let mut player_animation: Vec<&str> = Vec::new();
    let ani_str = "assets/sprites/Player1.png";
    player_animation.push(ani_str);
    player_animation.push("assets/sprites/Player2.png");
    for i in 0..4 {
        player_animation.push(ani_str);
    }
    let player1 = mob::Player::new(32., 32., app.world.w/2., app.world.h/2., &player_animation, &mut window);
    app.players.push(player1);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &e, &mut window);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(key) = e.press_args() {
            app.btn_press(&key);
        }

        if let Some(key) = e.release_args() {
            app.btn_release(&key);
        }

        if let Some(r) = e.resize_args() {
            //print!("{:?}\n", r);
            app.camera.w = r.draw_size[0] as f64;
            app.camera.h = r.draw_size[1] as f64;
        }
    }
    println!("\nDone!");
}