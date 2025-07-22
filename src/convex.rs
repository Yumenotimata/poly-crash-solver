use glam::{Vec2};

pub struct Convex2 {
    ps: Vec<Vec2>
}

impl Convex2 {
    pub fn new(ps: Vec<Vec2>) -> Convex2 {
        Convex2 { ps }
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
        // self.ps.iter()
        //     .fold((0.0, v), |acc: (f32, Vec2), x| {
        //         let l = x.dot(v);
        //         if l > acc.0 {
        //             (l, *x)
        //         } else {
        //             acc
        //         }
        //     }).1
        *self.ps.iter()
            .max_by(|p, q| p.dot(v).partial_cmp(&q.dot(v)).unwrap())
            .unwrap()
    }
}