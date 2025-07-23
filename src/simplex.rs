use glam::Vec2;

pub struct Simplex2 {
    pub points: Vec<Vec2>,
}

impl Simplex2 {
    pub fn new(points: Vec<Vec2>) -> Self {
        Simplex2 { points }
    }

    pub fn add(&mut self, point: Vec2) {
        self.points.push(point);
    }

    pub fn contains(&self, v: &Vec2) -> bool {
        (0..self.points.len())
            .map(|i| {
                let p1 = self.points[i];
                let p2 = self.points[(i + 1) % self.points.len()];
                (p1 - *v).perp_dot(p2 - p1) >= 0.0
            })
            .collect::<Vec<_>>()
            .windows(2)
            .all(|w| w[0] == w[1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        let simplex = Simplex2::new(vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 1.0),
        ]);
        assert!(simplex.contains(&Vec2::new(0.5, 0.5)));
        assert!(!simplex.contains(&Vec2::new(1.5, 1.5)));
    }
}