use glam::{Vec2};

pub struct Convex2 {
    ps: Vec<Vec2>
}

impl Convex2 {
    pub fn new(ps: Vec<Vec2>) -> Convex2 {
        Convex2 { ps }
    }

    pub fn is_convex2(ps: Vec<Vec2>) -> bool {
        let len = ps.len();

        if len <= 1 {
            return false;
        }

        let a = ps[0];
        let b = ps[1];
        let c = ps[2];

        let p = b - a;
        let q = c - b;

        let init = p.dot(q) >= 0.0;

        for i in 0..len {
            let a = ps[i % len];
            let b = ps[(i + 1) % len];
            let c = ps[(i + 2) % len];

            let p = b - a;
            let q = c - b;

            println!("iterate {}, {}, {}", a, b, c);

            if (p.dot(q) >= 0.0) != init {
                println!("{}, {}, {}", a, b, c);
                return false;
            }
        }

        return true;
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