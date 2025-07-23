pub mod convex;

#[cfg(test)]
mod tests {
    use glam::Vec2;

    use crate::convex::Convex2;

    use super::*;
    #[test]
    fn test_is_convex2() {
        // 右回り
        assert_eq!(
            Convex2::is_convex2(vec![Vec2::new(1.0, 1.0), Vec2::new(1.0, -1.0), Vec2::new(-1.0, -1.0), Vec2::new(-1.0, 1.0)]),
            true
        );
        // 左回り
        assert_eq!(
            Convex2::is_convex2(vec![Vec2::new(1.0, 1.0), Vec2::new(-1.0, 1.0), Vec2::new(-1.0, -1.0), Vec2::new(1.0, -1.0)]),
            true
        );
    }
}
