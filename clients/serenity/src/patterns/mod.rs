use position::Pos;
use rand;
use rand::Rng;

// Abstraction for the attacking methods to use
// They pass in the number of available bots and this method will use the
// right spread strategy for that number and return a vector
// TODO: Implement actual smart shooting
pub fn smart_attack_spread(pos: Pos, available_bots: i16) -> Vec<Pos> {
    let mut shoot_at: Vec<Pos> = Vec::new();

    match available_bots {
        4 => {
            shoot_at = triangle_smart(pos);
            shoot_at.push(pos);
        },
        3 => shoot_at = triangle_smart(pos),
        2 => {
            //TODO: Choose twin based on pos in map.
            shoot_at = rand_twin(pos);
        }
        1 => {
            shoot_at.push(pos.random_spread());
        },
        _ => ()
    }
    shoot_at
}


pub fn triangle_smart(pos: Pos) -> Vec<Pos> {
    let mut triangle = triangle_rand_tight(pos);

    // Shuffle so that the same will not be middled every time
    let mut rng = rand::thread_rng();
    rng.shuffle(&mut triangle[..]);
    {
        let p: &mut Pos = triangle.first_mut().expect("There should be three points here!");
        p.x = pos.x;
        p.y = pos.y;
    }
    // Shuffle so that the same bot will not get the middled pos every time
    rng.shuffle(&mut triangle[..]);
    return triangle;
}

pub fn triangle_rand_tight(pos: Pos) -> Vec<Pos> {
    match rand::thread_rng().gen_range(0, 2) {
        0 => { triangle_left(pos) }
        _ => { triangle_right(pos) }
    }
}

// TODO: Generalize for shot radius
pub fn triangle_left(pos: Pos) -> Vec<Pos> {
    let x = pos.x;
    let y = pos.y;
    return vec![ Pos::new(x-1, y  ),
                 Pos::new(x+1, y-1),
                 Pos::new(x,   y+1) ];
}

// TODO: Generalize for shot radius
pub fn triangle_right(pos: Pos) -> Vec<Pos> {
    let x = pos.x;
    let y = pos.y;
    return vec![ Pos::new(x+1, y  ),
                 Pos::new(x-1, y+1),
                 Pos::new(x,   y-1) ];
}

pub fn rand_twin(pos: Pos) -> Vec<Pos> {
    let ori = match rand::thread_rng().gen_range(0, 3) {
        0 => Orientation::Horizontal,
        1 => Orientation::Slash,
        _ => Orientation::Backslash,
    };
    return twin(pos, ori);
}

pub fn twin(pos: Pos, orientation: Orientation) -> Vec<Pos> {
    let x = pos.x;
    let y = pos.y;
    return match orientation {
        Orientation::Horizontal => vec![ Pos::new(x+1, y  ),
                                         Pos::new(x-1, y  ) ],
        Orientation::Slash      => vec![ Pos::new(x+1, y-1),
                                         Pos::new(x-1, y+1) ],
        Orientation::Backslash  => vec![ Pos::new(x,   y-1),
                                         Pos::new(x,   y+1) ],
    };
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Slash,
    Backslash,
}

