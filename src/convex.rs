use std::cmp;

use glam::{Vec2};

pub struct Convex2 {
    ps: Vec<Vec2>
}

impl Convex2 {
    pub fn new(ps: Vec<Vec2>) -> Result<Convex2, String> {
        if Convex2::is_convex2(&ps) {
            Ok(Convex2 { ps })
        } else {
            Err("The polygon is not convex".into())
        }
    }

    pub fn is_convex2(ps: &Vec<Vec2>) -> bool {
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

#[derive(Clone)]
pub struct Support {
    ps: Vec<Vec2>
}

impl Support {
    pub fn new(ps: Vec<Vec2>) -> Support {
        Support { ps }
    }

    pub fn map(&self, v: &Vec2) -> Vec2 {
        *self.ps.iter()
            .max_by(|p, q| p.dot(*v).partial_cmp(&q.dot(*v)).unwrap())
            .unwrap()
    }
}

pub struct Minkowski {
    pub p: Support,
    pub q: Support,
}

impl Minkowski {
    pub fn new<S: Into<Support>>(p: S, q: S) -> Minkowski {
        Minkowski { p: p.into(), q: q.into() }
    }

    pub fn map(&self, v: &Vec2) -> Vec2 {
        self.p.map(v) - self.q.map(&-v)
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::convex::Convex2;

    #[test]
    fn test_is_convex2() {
        // 凸
        // 右回り
        assert_eq!(
            Convex2::is_convex2(&vec![Vec2::new(1.0, 1.0), Vec2::new(1.0, -1.0), Vec2::new(-1.0, -1.0), Vec2::new(-1.0, 1.0)]),
            true
        );
        // 左回り
        assert_eq!(
            Convex2::is_convex2(&vec![Vec2::new(1.0, 1.0), Vec2::new(-1.0, 1.0), Vec2::new(-1.0, -1.0), Vec2::new(1.0, -1.0)]),
            true
        );

        // 凸でない
        assert_eq!(
            Convex2::is_convex2(&vec![Vec2::new(1.0, 1.0), Vec2::new(1.0, -1.0), Vec2::new(-1.0, 1.0), Vec2::new(-1.0, -1.0)]),
            false
        );
        assert_eq!(
            Convex2::is_convex2(&vec![Vec2::new(1.0, 1.0), Vec2::new(1.0, -1.0), Vec2::new(-1.0, -1.0), Vec2::new(-1.0, 1.0), Vec2::new(0.0, 0.0)]),
            false
        );
    }
}