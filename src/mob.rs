use crate::METER_IN_PIXELS;

pub enum Animations {
	Stand,
	Walk1,
	Walk2,
	Walk3,
	Jump,
	None,
}

impl Animations {
	// TODO: Change to something like 'is(&self, Animations)'
	pub fn is_jump(&self) -> bool {
		match self {
			Animations::Jump => true,
			_ => false,
		}
	}
}

pub struct MobState {
	pub look_dir: f64,
	pub walljump_time: f64,
	pub walljump: bool,
	pub air_resistance: f64,
	pub friction: f64,
}

impl MobState {
	pub fn new() -> MobState {
		MobState {
			look_dir: 1.0,
			walljump_time: 0.0,
			walljump: false,
			air_resistance: 1./650.,
			friction: 60.,
		}
	}
}

use crate::vector::Vec2;
pub struct Player {
	pub size: Vec2,
	pub pos: Vec2,
	pub velocity: Vec2,
	pub on_ground: bool,
	pub rotation: f64,
	pub color: [f64; 4],
	pub texture: crate::mob::Animations,
	pub animation: crate::render::PlayerAnimation,
	pub ani_length: f64,
	pub state: crate::mob::MobState,
}

impl Player {
	pub fn new(sx: f64, sy: f64, px: f64, py: f64, animation: &Vec<&str>, mut window: &mut piston_window::PistonWindow) -> Player {
		let size = Vec2::new(sx, sy);
		let pos = Vec2::new(px, py);
		let velocity = Vec2::new(0., 0.);
		let animation = crate::render::PlayerAnimation::new(&mut window, animation);
		Player {
			size,
			pos,
			velocity,
			on_ground: false,
			rotation: 0.0,
			color: [1.0, 1.0, 1.0, 1.0],
			texture: crate::mob::Animations::Stand,
			animation,
			ani_length: 0.0,
			state: crate::mob::MobState::new(),
		}
	}
	pub fn play_animation(&mut self, ani: crate::mob::Animations, length: f64) {
		match ani {
			crate::mob::Animations::Jump => {
				self.texture = crate::mob::Animations::Jump;
			},
			crate::mob::Animations::Walk1 => {
				self.texture = crate::mob::Animations::Walk1;
			},
			_ => {
				self.texture = crate::mob::Animations::Stand;
			},
		}
		self.ani_length = length;
	}
	pub fn update(&mut self, keystate: &mut crate::Keys, world: &crate::World, camera: &mut crate::Camera, collidables: &Vec<crate::collision::Cube>, args: &piston::UpdateArgs) {
        // Controls
        if keystate.space && self.on_ground{
            self.velocity.y = -5.;
            self.on_ground = false;
            //self.play_animation(crate::mob::Animations::Jump, 0.6);
        }
        if keystate.d {
            if self.on_ground {
                self.velocity.x += world.grav_const*2.*args.dt;
                self.velocity.x = self.velocity.x.clamp(-5., 5.);
            }else {
                self.velocity.x += world.grav_const*0.5*args.dt;
                self.velocity.x = self.velocity.x.clamp(-5.,5.);
            }
            if self.velocity.x < 0. {
                self.velocity.x -= self.velocity.x*0.1;
            }
            self.state.look_dir = 1.0;
        }else
        if keystate.a {
            if self.on_ground {
                self.velocity.x -= world.grav_const*2.*args.dt;
                self.velocity.x = self.velocity.x.clamp(-5., 5.);
            }else {
                self.velocity.x -= world.grav_const*0.5*args.dt;
                self.velocity.x = self.velocity.x.clamp(-5., 5.);
            }
            if self.velocity.x > 0. {
                self.velocity.x -= self.velocity.x*0.1;
            }
            self.state.look_dir = -1.0;
        }else {
            self.velocity.x -= self.velocity.x*0.1;
        }

        // Add gravity and subtract air resistance
        self.velocity.y += world.grav_const*args.dt;
        self.velocity.y -= self.velocity.y*(self.state.air_resistance.clamp(0.,1.));

        // Move mob
        self.pos.y += self.velocity.y*METER_IN_PIXELS*args.dt;
        self.pos.x += self.velocity.x*METER_IN_PIXELS*args.dt;

        // Walljump cooldown
        if self.state.walljump {
            self.state.walljump_time -= args.dt;
            if self.state.walljump_time <= 0. {
                self.state.walljump = false;
            }
        }

        // Collision detection
        use crate::collision::{cube_collider, Side};
        for obj in collidables {
            if cube_collider(self, obj, Side::North) {
                self.velocity.y = if self.velocity.y > 0. { 0. }
                                 else { self.velocity.y };
                self.pos.y = obj.pos.y-(obj.size.y+self.size.y)/2.;
                self.on_ground = true;
            }else
            if cube_collider(self, obj, Side::East) {
                self.velocity.x = if self.velocity.x < 0. { 0. }
                                 else { self.velocity.x };
                self.pos.x = obj.pos.x+(obj.size.x+self.size.x)/2.;

                // Add friction on wall
                self.velocity.y -= self.velocity.y.clamp(-0.06,100.)*obj.friction*args.dt;
                // Slide down wall
                if keystate.s {
                    self.velocity.y *= 2.;
                    self.velocity.y = self.velocity.y.clamp(0., 3.6);
                }
                
                // Walljump!
                if keystate.space && !self.state.walljump {
                    self.velocity.x = world.grav_const*10.*args.dt;
                    self.velocity.y = -3.;
                    keystate.space = false;
                    self.state.walljump = true;
                    self.state.walljump_time = 1.0;
                }else if keystate.space {
                    self.state.walljump_time -= 2.5*args.dt;
                }else if !keystate.space {
                    self.state.walljump = false;
                }
            }else
            if cube_collider(self, obj, Side::West) {
                self.velocity.x = if self.velocity.x > 0. { 0. }
                                 else { self.velocity.x };
                self.pos.x = obj.pos.x-(obj.size.x+self.size.x)/2.;

                // Add friction on wall
                self.velocity.y -= self.velocity.y.clamp(-0.06,100.)*obj.friction*args.dt;
                // Slide down wall
                if keystate.s {
                    self.velocity.y *= 2.;
                    self.velocity.y = self.velocity.y.clamp(0., 3.6);
                }

                // Walljump!
                if keystate.space && !self.state.walljump {
                    self.velocity.x = -world.grav_const*10.*args.dt;
                    self.velocity.y = -3.;
                    keystate.space = false;
                    self.state.walljump = true;
                    self.state.walljump_time = 1.0;
                }else if keystate.space {
                    self.state.walljump_time -= 2.5*args.dt;
                }else if !keystate.space {
                    self.state.walljump = false;
                }
            }else
            if cube_collider(self, obj, Side::South) {
                self.velocity.y = if self.velocity.y < 0. { 0. }
                                 else { self.velocity.y };
                self.pos.y = obj.pos.y+(obj.size.y+self.size.y)/2.;
            }
        }

        camera.position.x = (self.pos.x-camera.w)*camera.zoom+(camera.w*camera.zoom)/2.;
        camera.position.y = (self.pos.y-camera.h)*camera.zoom+(camera.h*camera.zoom)/2.;

        self.ani_length -= args.dt;
        if self.ani_length < 0.0 && self.velocity.y == 0.0 && (self.velocity.x > 0.1 || self.velocity.x < -0.1) {
        	match self.texture {
        		crate::mob::Animations::Stand => self.play_animation(crate::mob::Animations::Walk1, 0.2),
        		crate::mob::Animations::Walk1 => self.play_animation(crate::mob::Animations::Stand, 0.2),
        		_ => (),
        	}
        }
	}
	pub fn render(&mut self, camera: &crate::Camera, window: &mut piston_window::PistonWindow, e: &piston::Event) {
		let (x, y) = (self.pos.x, self.pos.y);
		let (w, h) = (self.size.x, self.size.y);
		let look_dir = &self.state.look_dir;
		let texture = match self.texture {
			crate::mob::Animations::Jump => &self.animation.jump,
			crate::mob::Animations::Walk1 => &self.animation.walk1,
			crate::mob::Animations::Stand => &self.animation.stand,
			crate::mob::Animations::None => &self.animation.stand,
			_ => panic!("Invalid animation!"),
		};

		let scale = &camera.zoom;
		let offset = &camera.position;
		let (app_w, app_h) = (&camera.w, &camera.h);

		use crate::piston_window::{image,ImageSize};
		use crate::graphics::Transformed;
		window.draw_2d(e, |c, g, _| {
			image(texture,
				  c.transform
				   .trans((x-(w*look_dir)/2.-app_w/2.)*scale+app_w/2.-offset.x,
				   		  (y-h/2.-app_h/2.)*scale+app_h/2.-offset.y)
				   .scale((look_dir*w/texture.get_width() as f64)*scale,
				   		  (h/texture.get_height() as f64)*scale)
			, g);
		});
	}
}