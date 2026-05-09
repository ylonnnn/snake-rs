use snake_rs::{config::Config, game::SnakeGame};

fn main() {
    let mut game = SnakeGame::new((20, 20));
    game.with_config(Config {
        wall_collisions: false,
    });

    game.start();
}
