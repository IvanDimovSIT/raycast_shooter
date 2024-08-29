use macroquad::math::{vec2, Vec2};

pub fn rotate_point(point: Vec2, origin: Vec2, thetha: f32) -> Vec2 {
    let p = point - origin;
    let x_rot = p.x * thetha.cos() - p.y * thetha.sin();
    let y_rot = p.x * thetha.sin() + p.y * thetha.cos();

    vec2(x_rot, y_rot) + origin
}

pub fn find_intersection(a_start: Vec2, a_end: Vec2, b_start: Vec2, b_end: Vec2) -> Option<Vec2> {
    let b_dir = b_end - b_start;
    let perp_b = Vec2::new(-b_dir.y, b_dir.x);

    let denom = a_end.dot(perp_b);

    if denom.abs() < f32::EPSILON {
        return None;
    }

    let t = (b_start - a_start).dot(perp_b) / denom;

    if t >= 0.0 {
        let intersection = a_start + t * a_end;
        let b_t = (intersection - b_start).dot(b_dir) / b_dir.dot(b_dir);

        if (0.0..=1.0).contains(&b_t) {
            return Some(intersection);
        }
    }

    None
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
}
