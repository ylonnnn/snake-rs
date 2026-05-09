#[derive(Debug)]
pub struct Config {
    pub wall_collisions: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            wall_collisions: true,
        }
    }
}
