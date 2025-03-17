use macroquad::prelude::*;

use crate::partical::Partical;
use crate::matrix::Matrix;

pub struct Universe {
  matter:Vec<Partical>,
  // space:Matrix<bool>,
  
}
impl Universe {
  pub fn new() -> Self {
    Universe {
      matter: Vec::new(),
      // space:Matrix::new(screen_height() as usize, screen_width() as usize, false),
    }
  }
  pub fn fill(&mut self, n:u32) {
    let mut space_filled = Vec::new();
    for i in 0..n {
      let new = Partical::new();
      let pos = (new.pos.x as usize, new.pos.y as usize);
      space_filled.append(&mut Self::get_circle(new.get_size() as usize, pos));
      self.matter.push(new);
    }
    // for (x,y) in space_filled {self.space.set(y, x, true);}
  }
  pub fn add_partical(&mut self, pos:Vec2) {
    self.matter.push(Partical::place_new(pos));
  }
  
  fn get_circle(r:usize,pos:(usize, usize)) -> Vec<(usize,usize)>{
    let mut spots = vec![pos];
    if r == 0 {return spots;}
    for x in 0..r {
      for y in 0..(r.saturating_sub(x)) {
        let (v,w) = (x,y);
        if x == 0 && y == 0 { continue; }
        if x == 0 {
          spots.push((pos.0,pos.1.saturating_sub(w)));
          spots.push((pos.0,pos.1.saturating_add(w)));
          continue;
        }
        if y == 0 {
          spots.push((pos.0.saturating_sub(v),pos.1));
          spots.push((pos.0.saturating_add(v),pos.1));
          continue;
        }
        spots.push((pos.0.saturating_add(v),pos.1.saturating_add(w)));
        spots.push((pos.0.saturating_add(v),pos.1.saturating_sub(w)));
        spots.push((pos.0.saturating_sub(v),pos.1.saturating_add(w)));
        spots.push((pos.0.saturating_sub(v),pos.1.saturating_sub(w)));
      }
    }
    spots
  }
}
// Mechanics
impl Universe {
  
}


// Graphics
impl Universe {
  pub fn display(&self) {
    for i in &self.matter {
      let speed = i.vel.length();
      let max_speed = 500.0; // Adjust this based on simulation
      let normalized_speed = (speed / max_speed).clamp(0.0, 1.0);
  
      // Convert speed to hue (0 = red, 1 = violet, >1 = white)
      let hue = (1.0 - normalized_speed) * 240.0; // 240° (blue) → 0° (red)
      let (r, g, b) = Self::hsv_to_rgb(hue, 1.0, normalized_speed); // Full saturation and brightness
  
      let color = Color::new(r, g, b, normalized_speed); // Alpha always 1 for full opacity
      draw_circle(i.pos.x, i.pos.y, i.get_size(), color);
    }
  }


  fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h as i32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        300..=360 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    (r + m, g + m, b + m)
}

}