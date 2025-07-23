use std::cmp;

use glam::{Vec2};

pub struct Convex2 {
    ps: Vec<Vec2>
}

impl Convex2 {
    pub fn new(ps: Vec<Vec2>) -> Convex2 {
        Convex2 { ps }
    }

    pub fn is_convex2(ps: &Vec<Vec2>) -> bool {
        let len = ps.len();

        match ps.len() {
            0..=1   => return false,
            // 辺、三角形は常に凸
            2..=3   => return true,
            len => 
                (0..len).fold((None, true), |(dir, res), i| {
                    let p = ps[(i + 1) % len] - ps[i % len];
                    let q = ps[(i + 2) % len] - ps[(i + 1) % len];

                    let ord = p.perp_dot(q) >= 0.0;
                    let dir = dir.or(Some(ord));

                    (dir, dir.unwrap() == ord && res)
                }).1
            
        }
    }
}

impl Into<Support> for Convex2 {
    fn into(self) -> Support {
        Support::new(self.ps)
    }
}

pub struct Support {
    ps: Vec<Vec2>
}

impl Support {
    pub fn new(ps: Vec<Vec2>) -> Support {
        Support { ps }
    }

    pub fn map(&self, v: Vec2) -> Vec2 {
        *self.ps.iter()
            .max_by(|p, q| p.dot(v).partial_cmp(&q.dot(v)).unwrap())
            .unwrap()
    }
}