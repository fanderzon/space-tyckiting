use position::Pos;
use defs::{Config};

pub struct Radar {
    pub radar_positions: Vec<Pos>,
    pub meta: RadarMeta
}

#[derive(Clone, Copy, Debug)]
pub struct RadarMeta {
    field_radius: i16,
    radar_radius: i16,
    row_start: i16,
    row_end: i16,
    row_start_change: i16,
    row_end_change: i16,
}

pub struct RadarPass {
    last_pos: Option<Pos>,
    meta: RadarMeta
}

impl RadarMeta {
    pub fn new(field_radius: i16, radar_radius: i16) -> RadarMeta {
        RadarMeta {
            field_radius: field_radius,
            radar_radius: radar_radius,
            row_start: 0,
            row_end: field_radius - radar_radius,
            row_start_change: -(radar_radius + 1),
            row_end_change: 0
        }
    }

    pub fn with_defaults() -> RadarMeta {
        RadarMeta::new(14,2)
    }
}

impl Radar {
    pub fn new() -> Radar {
        Radar {
            radar_positions: Vec::new(),
            meta: RadarMeta::with_defaults(),
        }
    }

    pub fn get_radar_positions(&mut self, config: &Config) -> Vec<Pos> {
        // If we already have calculated the radar positions for this config return it
        if self.meta.field_radius == config.field_radius &&
            self.meta.radar_radius == config.radar &&
            self.radar_positions.len() > 0
        {
            return self.radar_positions.clone();
        }

        // Reset meta for new calculation
        self.meta = RadarMeta::new(config.field_radius, config.radar);

        // Start in the top left corner
        let start_pos = Pos {
            x: self.meta.row_start,
            y: -(config.field_radius - config.radar)
        };
        let last_pos = start_pos;
        self.radar_positions.push(start_pos);

        let meta = self.meta.clone();
        let mut pass: RadarPass = RadarPass{
            last_pos: Some(last_pos),
            meta: meta
        };
        loop {
            pass = get_next_position(pass);
            if pass.last_pos.is_some() {
                self.radar_positions.push(pass.last_pos.unwrap());
            } else {
                break;
            }
        }
        return self.radar_positions.clone();
    }
}

fn get_next_position(mut pass: RadarPass) -> RadarPass {
    let last_pos = pass.last_pos.unwrap();

    if last_pos.x >= pass.meta.row_end {
        pass.meta.row_start = pass.meta.row_start + pass.meta.row_start_change;
        pass.meta.row_end = pass.meta.row_end + pass.meta.row_end_change;

        if pass.meta.row_start <= -(pass.meta.field_radius) {
            let subtract_y = if (last_pos.y + 4) > 0 { last_pos.y + 4 } else { 0 };
            pass.meta.row_start = -(pass.meta.field_radius - pass.meta.radar_radius);
            pass.meta.row_start_change = 0;
            pass.meta.row_end_change = -(pass.meta.radar_radius + 1);
            pass.meta.row_end = pass.meta.field_radius - subtract_y - pass.meta.radar_radius;
        }

        // When the y coord is larger than the radius let's exit
        if last_pos.y + pass.meta.radar_radius + 1  > pass.meta.field_radius {
            pass.last_pos = None;
            return pass;
        }
        pass.last_pos = Some(Pos {
            x: pass.meta.row_start,
            y: last_pos.y + (pass.meta.radar_radius + 1)
        });
        return pass;
    }
    pass.last_pos = Some(Pos {
        x: last_pos.x + 4,
        y: last_pos.y,
    });
    return pass;
}
