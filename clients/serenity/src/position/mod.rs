extern crate serde;
extern crate serde_json;

use std::cmp;
use std::ops::Add;
use util;
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/position.rs"));

impl Pos {
    #[allow(dead_code)]
    pub fn new(x: i16, y: i16) -> Pos {
        return Pos {x: x, y: y};
    }

    pub fn origo() -> Pos {
        return Pos {x: 0, y: 0};
    }

    pub fn distance(self, other: Pos) -> i16 {
        let dx: i16 = self.x - other.x;
        let dy: i16 = self.y - other.y;
        let dz: i16 = self.x + self.y - other.x - other.y;

        // cmp::max only takes 2 args
        cmp::max( cmp::max(dx.abs(), dy.abs()), dz.abs())
    }

    pub fn random_spread(&self) -> Pos {
        Pos {
            x: self.x + util::get_rand_range(-1, 1),
            y: self.y + util::get_rand_range(-1, 1)
        }
    }

    #[allow(dead_code)]
    pub fn neighbors(&self, radius: i16) -> Vec<Pos> {
        let mut result: Vec<Pos> = Vec::new();
        let x_min = self.x - radius;
        let x_max = self.x + radius;
        let y_min = self.y - radius;
        let y_max = self.y + radius;

        for x in x_min..x_max {
            for y in y_min..y_max {
                let new_pos = Pos { x: x, y: y };
                if self.distance(new_pos) <= radius && *self != new_pos {
                    result.push(new_pos);
                }
            }
        }

        return result;
    }

    #[allow(dead_code)]
    pub fn clamped_neighbors(&self, radius: i16, field_radius: i16) -> Vec<Pos> {
        let center = Pos { x: 0, y: 0 };
        self.neighbors(radius)
            .iter()
            .cloned()
            .filter(|pos| pos.distance(center) <= field_radius)
            .collect()
    }

    #[allow(dead_code)]
    pub fn clamp(&self, field_radius: &i16) -> Pos {
        let center = Pos { x:0,y:0 };
        let d = self.distance(center);
        let radius = *field_radius;

        // When in radius, we don't have to change anything
        if d != 0 && d > radius {
            // We clip the position to nearest in radius
            let t = 1.0 * radius as f32 / d as f32;

            let cx = self.x as f32 * t;
            let cy = (-self.x - self.y) as f32 * t;
            let cz = self.y as f32 * t;

            // We need to round the floating point location to nearest hex
            return self.round_cube_to_nearest_hex( cx, cy, cz);
        }

        return self.clone();
    }

    #[allow(dead_code)]
    fn round_cube_to_nearest_hex( &self, x: f32, y: f32, z: f32 ) -> Pos {
        // Simply rounding would work in most of the cases.
        // The extra logic is there to handle few special cases,
        // where a position would round to wrong hex and possibly out of area.
        // The logic is explained here:
        // http://www.redblobgames.com/grids/hexagons/#rounding

        let mut rx = x.round() as i16;
        let ry = y.round() as i16;
        let mut rz = z.round() as i16;

        let x_diff: f32 = {rx as f32 - x}.abs();
        let y_diff: f32 = {ry as f32 - y}.abs();
        let z_diff: f32 = {rz as f32 - z}.abs();

        if x_diff > y_diff && x_diff > z_diff {
            rx = -ry - rz;
        } else if y_diff > z_diff {
            // Do nothing apparently? ry = -rx - rz;
        } else {
            rz = -rx - ry;
        }
        Pos {
            x: rx as i16,
            y: rz as i16
        }
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos { x: self.x + other.x, y: self.y + other.y }
    }
}
