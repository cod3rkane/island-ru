
const MAX_WIDTH: usize = 15876; // 126*126

pub fn generate_island_gradient_map(width: i32, height: i32) -> [f64; MAX_WIDTH] {
    let mut map: [f64; MAX_WIDTH] = [0.0; MAX_WIDTH];

    for x in 0..width {
        for y in 0..height {
            let i: f64 = x as f64 / width as f64 * 2.0 - 1.0;
            let j: f64 = y as f64 / height as f64 * 2.0 - 1.0;
            let value = i.abs().max(j.abs());

            let a: f64 = -3.0;
            let b: f64 = 1.5;
            let c: f64 = b - b * value;
            let island_gradient_value = value.powf(a) / (value.powf(a) + c.powf(a));

            let index: usize = y as usize * width as usize + x as usize;
            map[index] = island_gradient_value;
        }
    }

    map
}
