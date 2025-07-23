use crate::convex::*;
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

    todo!()
}

#[cfg(test)]
mod tests {
    use glam::Vec2;
    use crate::convex::{Convex2, Support, Minkowski};
}