use crate::models::Input;

const DEFAULT_POSITION: (f32, f32) = (0.0, 0.0);

pub struct Player {
    pub position: (f32, f32)
}

impl Default for Player {
    fn default() -> Self {
        Player {
            position: DEFAULT_POSITION
        }
    }
}

impl Player {
    pub fn new(position: (f32, f32)) -> Self {
        Self {
            position
        }
    }

    pub fn update_deplacement(&mut self, keys: Vec<Input>, dt: f32) {
        keys.iter()
            .for_each(|key| {
                match key {
                    Input::UP => self.position.1 -= 100.0 * dt,
                    Input::RIGHT => self.position.0 += 100.0 * dt,
                    Input::DOWN => self.position.1 += 100.0 * dt,
                    Input::LEFT => self.position.0 -= 100.0 * dt,
                }
            });
    }
}