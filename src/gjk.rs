use crate::{convex::*, simplex::Simplex2};
use glam::Vec2;

pub fn crashed(p: Convex2, q: Convex2) -> bool {
    let p: Support = p.into();
    let q: Support = q.into();
    let minkowski = Minkowski::new(p.clone(), q.clone());

    // pの適当な点から原点へのベクトルの方向に最も遠い点p1
    let p1 = minkowski.map(&(Vec2::new(0.0, 0.0) - p.map(&Vec2::new(1.0, 0.0))));
    // p1から原点へのベクトルの方向に最も遠い点p2
    let p2 = minkowski.map(&-p1);
    // 線分p12の原点への最近接点t
    let t = (-p2).project_onto(p1 - p2) + p2;
    let p3 = minkowski.map(&-t);
    let mut simplex = Simplex2::new(vec![p1, p2, p3]);
    let mut prev_simplex = Simplex2::new(vec![Vec2::ZERO, Vec2::ZERO, Vec2::ZERO]);

    loop {
        if simplex.contains(&Vec2::new(0.0, 0.0)) {
            return true;
        } else if simplex == prev_simplex {
            return false;
        } else {
            // p1, p2, p3のうち原点から最も遠い点を除外
            let mut points = simplex.points.clone();
            points.sort_by(|a, b| a.length_squared().partial_cmp(&b.length_squared()).unwrap());
            points.pop();
            let p1 = points[0];
            let p2 = points[1];
            let t = (-p2).project_onto(p1 - p2) + p2;
            let p3 = minkowski.map(&-t);
            prev_simplex = simplex.clone();
            simplex = Simplex2::new(vec![p1, p2, p3]);
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec2;
    use crate::convex::{Convex2, Support, Minkowski};
}