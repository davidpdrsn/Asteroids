use size::*;
use point::*;

pub struct Config {
    pub enemy_size: Size,
    pub hero_size: Size,
    pub hero_speed: u32,
    pub hero_starting_position: Point,
    pub number_of_enemies: u32,
    pub starting_safe_zone_around_hero_size: u32,
    pub world_size: Size,
    pub vertical_movement_allowed: bool,
    pub horizontal_movement_allowed: bool,
}

impl Config {
    pub fn default() -> Config {
        let mut c = Config {
            enemy_size: Size { height: 10, width: 10 },
            hero_size: Size { height: 20, width: 20 },
            hero_starting_position: Point { x: 0, y: 0 },
            hero_speed: 10,
            number_of_enemies: 0,
            starting_safe_zone_around_hero_size: 20,
            world_size: Size { height: 300, width: 300 },
            vertical_movement_allowed: false,
            horizontal_movement_allowed: true,
        };

        c.hero_starting_position.x = 0;
        c.hero_starting_position.y = 75.percent_of(c.world_size.height);

        c
    }
}

trait PercentageCalculation<T> {
    fn percent_of(self, T) -> T;
}

impl PercentageCalculation<u32> for u32 {
    fn percent_of(self, x: Self) -> Self {
        (x as f32 * (self as f32 / 100.0)) as u32
    }
}