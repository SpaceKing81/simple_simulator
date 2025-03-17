use macroquad::{math::{vec2, Vec2}, prelude::rand, window::{screen_height, screen_width}};

const MASS_MODIFIER:f32 = 2.0;
const GRAVITY:f32 = 1.0;
// const CHARGE:f32 = -1.0;

pub struct Partical {
  pub pos:Vec2, // current pos
  pub vel:Vec2, // direction of motion
  pub mass:f32, // mass of partical
  
}
// idk, randos
impl Partical {
  pub fn new() -> Self {
    let pos= vec2(
      rand::gen_range(0.0, screen_width()),
      rand::gen_range(0.0, screen_height())
    );
    let vel= vec2(
      rand::gen_range(0.0, 10.),
      rand::gen_range(0.0, 10.)
    );

    let mass = 10.0;
    Self {
      pos,
      vel,
      mass,

    }
  }
  pub fn place_new(pos:Vec2) -> Self {
    Self {
      pos,
      vel:Vec2::ZERO,
      mass:10.0,
    }
  }
}
// Mechanics
impl Partical {
  fn forward(&mut self) {
    self.pos += self.vel;
  }
  fn edge_case(&mut self) {
    let upper_bound = vec2(screen_width(), screen_height());
    let lower_bound = Vec2::ZERO;
    match self.pos.x {
      s if s > upper_bound.x => {self.pos.x = upper_bound.x; self.vel.x *= -1.0},
      s if s < lower_bound.x => {self.pos.x = lower_bound.x; self.vel.x *= -1.0},
      _ => (),
    };
    match self.pos.y {
      s if s > upper_bound.y => {self.pos.y = upper_bound.y; self.vel.y *= -1.0},
      s if s < lower_bound.y => {self.pos.y = lower_bound.y; self.vel.y *= -1.0},
      _ => (),
    };

  }
  fn gravity_acceleration(&mut self, pos:Vec2) {
    let (d,distance) = self.displacment(pos);
    let a = GRAVITY * d / (distance * distance);
    self.vel += a;
  }
  fn acceleration_down(&mut self) {
    self.vel.y -= 1.0;
  }

}


// Helping fn
impl Partical {
  
  pub fn get_size(&self) -> f32 {
    let size= self.mass / MASS_MODIFIER;
    size
  }
  fn displacment(&self, pos:Vec2) -> (Vec2, f32){
    let dx = self.pos.x - pos.x;
    let dy = self.pos.y - pos.y;
    let d = vec2(dx, dy);
    let magniude = d.length();
    (d,magniude)
  }
  fn kinetic_energy(&self) -> f32 {
    0.5 * self.mass * self.vel.length() * self.vel.length()
  }

}