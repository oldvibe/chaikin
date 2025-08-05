pub fn chaikin_algo(in_points: &[(f32, f32)]) -> Vec<(f32, f32)> {
    let mut tmp:Vec<(f32, f32)> = Vec::new();

    tmp.push(in_points[0]);
    for i in 0..in_points.len().saturating_sub(1) {
        let p0: (f32, f32) = in_points[i];
        let p1:(f32, f32) = in_points[i + 1];

        let q: (f32, f32) = (
            0.75 * p0.0 + 0.25 * p1.0,
            0.75 * p0.1 + 0.25 * p1.1,
        );
        let r: (f32, f32) = (
            0.25 * p0.0 + 0.75 * p1.0,
            0.25 * p0.1 + 0.75 * p1.1,
        );

        tmp.push(q);
        tmp.push(r);
    }
    tmp.push(*in_points.last().unwrap());

    tmp
}
