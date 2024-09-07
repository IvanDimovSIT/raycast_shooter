use macroquad::math::{vec2, Vec2};

pub fn rotate_point(point: Vec2, origin: Vec2, theta: f32) -> Vec2 {
    let p = point - origin;
    let x_rot = p.x * theta.cos() - p.y * theta.sin();
    let y_rot = p.x * theta.sin() + p.y * theta.cos();

    vec2(x_rot, y_rot) + origin
}

pub fn find_intersection(a_start: Vec2, a_end: Vec2, b_start: Vec2, b_end: Vec2) -> Option<Vec2> {
    let line_1 = geo::Line::new(
        geo::coord! {x: a_start.x, y: a_start.y},
        geo::coord! { x: a_end.x, y: a_end.y },
    );
    let line_2 = geo::Line::new(
        geo::coord! {x: b_start.x, y: b_start.y},
        geo::coord! { x: b_end.x, y: b_end.y },
    );
    let inter = geo::line_intersection::line_intersection(line_1, line_2)?;
    match inter {
        geo::LineIntersection::SinglePoint {
            intersection,
            is_proper: _,
        } => Some(vec2(intersection.x, intersection.y)),
        _ => None,
    }
}

pub fn line_intersects_circle(start: Vec2, end: Vec2, center: Vec2, radius: f32) -> bool {
    let d = end - start;
    let f = start - center;

    let a = d.x * d.x + d.y * d.y;
    let b = 2.0 * (f.x * d.x + f.y * d.y);
    let c = (f.x * f.x + f.y * f.y) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < f32::EPSILON {
        false
    } else {
        let discriminant_sqrt = discriminant.sqrt();
        let t1 = (-b - discriminant_sqrt) / (2.0 * a);
        let t2 = (-b + discriminant_sqrt) / (2.0 * a);

        (0.0..=1.0).contains(&t1) || (0.0..=1.0).contains(&t2)
    }
}

pub fn check_circles_collide(center1: Vec2, radius1: f32, center2: Vec2, radius2: f32) -> bool {
    center1.distance(center2) <= radius1 + radius2
}

pub fn find_perpendicular_vector(v: Vec2) -> Vec2 {
    vec2(-v.y, v.x)
}

#[cfg(test)]
mod tests {
    use std::f32::{consts::PI, EPSILON};

    use super::*;

    #[test]
    fn test_find_intersection() {
        let a_start = vec2(0.0, 0.0);
        let a_end = vec2(10.0, 0.0);

        let b_start = vec2(2.0, -10.0);
        let b_end = vec2(2.0, 10.0);

        let result = find_intersection(a_start, a_end, b_start, b_end);
        assert!(result.is_some());

        assert_eq!(result.unwrap(), vec2(2.0, 0.0));
    }

    #[test]
    fn test_rotate_point() {
        let point = vec2(0.0, 1.0);
        let origin = vec2(0.0, 0.0);
        let thetha = PI;

        let result = rotate_point(point, origin, thetha);

        assert!((result.x).abs() < EPSILON);
        assert!((result.y - (-1.0)).abs() < EPSILON);
    }

    #[test]
    fn test_line_intersects_circle() {
        let start = vec2(-1.0, 1.0);
        let end = vec2(1.0, 1.0);

        let center1 = vec2(0.0, 0.0);
        let r = 0.4;

        assert!(!line_intersects_circle(start, end, center1, r));

        let center2 = vec2(0.0, 0.8);
        assert!(line_intersects_circle(start, end, center2, r));
    }

    #[test]
    fn test_check_circles_collide() {
        let c1 = vec2(0.0, 0.0);
        let r1 = 1.0;

        let c2 = vec2(1.5, 0.7);
        let r2 = 1.0;

        assert!(check_circles_collide(c1, r1, c2, r2));

        let c3 = vec2(1.9, 0.9);
        let r3 = 1.0;

        assert!(!check_circles_collide(c1, r1, c3, r3));
    }
}
