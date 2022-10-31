use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{WinSize, FORMATION_MEMBERS_MAX, BASE_SPEED};

// region:        --- Common Components
#[derive(Component)]
pub struct Velocity {
  pub x: f32,
  pub y: f32,
}

#[derive(Component)]
pub struct Movable {
  pub auto_despawn: bool,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);
impl From<(f32, f32)> for SpriteSize {
  fn from(val: (f32, f32)) -> Self {
    SpriteSize(Vec2::new(val.0, val.1))
  }
}
// endregion:     --- Common Components

// region:        --- Player Components
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;

// endregion:     --- Player Components

// region:        --- Enemy Components

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;

#[derive(Clone, Component)]
pub struct Formation {
  pub start: (f32, f32),
  pub radius: (f32, f32),
  pub pivot: (f32, f32),
  pub speed: f32,
  pub angle: f32, // change per tick
}

#[derive(Default)]
pub struct FormationMaker {
  current_template: Option<Formation>,
  current_members: u32,
}


/// Formation factory implementation
impl FormationMaker {
	pub fn make(&mut self, win_size: &WinSize) -> Formation {
		match (&self.current_template, self.current_members >= FORMATION_MEMBERS_MAX) {
			// if has current template and still within max members
			(Some(tmpl), false) => {
				self.current_members += 1;
				tmpl.clone()
			}
			// if first formation or previous formation is full (need to create a new one)
			(None, _) | (_, true) => {
				let mut rng = thread_rng();

				// compute the start x/y
				let w_span = win_size.w / 2. + 100.;
				let h_span = win_size.h / 2. + 100.;
				let x = if rng.gen_bool(0.5) { w_span } else { -w_span };
				let y = rng.gen_range(-h_span..h_span) as f32;
				let start = (x, y);

				// compute the pivot x/y
				let w_span = win_size.w / 4.;
				let h_span = win_size.h / 3. - 50.;
				let pivot = (rng.gen_range(-w_span..w_span), rng.gen_range(0.0..h_span));

				// compute the radius
				let radius = (rng.gen_range(80.0..150.), 100.);

				// compute the start angle
				let angle = (y - pivot.1).atan2(x - pivot.0);

				// speed (fixed for now)
				let speed = BASE_SPEED;

				// create the formation
				let formation = Formation {
					start,
					radius,
					pivot,
					speed,
					angle,
				};

				// store as template
				self.current_template = Some(formation.clone());
				// reset members to 1
				self.current_members = 1;

				formation
			}
		}
	}
}

// endregion:     --- Enemy Components

// region:        --- Explosion Components
#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
  fn default() -> Self {
    Self(Timer::from_seconds(0.05, true))
  }
}

// endregion:     --- Explosion Components
