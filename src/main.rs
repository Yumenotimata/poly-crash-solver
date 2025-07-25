use poly_crash_solver::convex::Convex2;

use glam::Vec2;

fn main() {
    let convex1 = Convex2::new(vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ]).unwrap();

    let convex2 = Convex2::new(vec![
        Vec2::new(0.5, 0.5),
        Vec2::new(1.5, 0.5),
        Vec2::new(1.5, 1.5),
        Vec2::new(0.5, 1.5),
    ]).unwrap();

    let result = poly_crash_solver::gjk::crashed(convex1, convex2);
    if result {
        println!("The convex shapes are colliding.");
    } else {
        println!("The convex shapes are not colliding."); 
    }
}