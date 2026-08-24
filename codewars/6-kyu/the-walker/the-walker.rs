use std::f64::consts::PI;
​
fn solve(a: i32, b: i32, c: i32, alpha: i32, beta: i32, gamma: i32) -> Vec<i32> {
    let (a, b, c) = (a as f64, b as f64, c as f64);
    let (alpha, beta, gamma) = (
        (alpha as f64).to_radians(),
        (beta as f64).to_radians(),
        (gamma as f64).to_radians()
    );
    
    let x_a = alpha.cos() * a;
    let y_a = alpha.sin() * a;
    let x_b = x_a - beta.sin() * b;
    let y_b = y_a + beta.cos() * b;
    let x_c = x_b - gamma.cos() * c;
    let y_c = y_b - gamma.sin() * c;
    
    let dist_co = (x_c * x_c + y_c * y_c).sqrt().round() as i32;
    
    let angle_c_deg = y_c.atan2(x_c).to_degrees();
    let (deg, min, sec) = degree_to_dms_truncated(angle_c_deg);
    
    vec![dist_co, deg, min, sec]
}
​
fn degree_to_dms_truncated(angle: f64) -> (i32, i32, i32) {
    let deg = angle.floor();
    let min_full = (angle - deg) * 60.0;
    let min = min_full.floor();
    let sec = ((min_full - min) * 60.0);
    (deg as i32, min as i32, sec as i32)
}