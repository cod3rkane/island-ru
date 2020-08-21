extern crate num;
extern crate opensimplex;

const MAX_WIDTH: usize = 15876; // 126*126

pub fn generate_island_gradient_map(width: i32, height: i32) -> [f64; MAX_WIDTH] {
    let mut map: [f64; MAX_WIDTH] = [0.0; MAX_WIDTH];
    let mut map_original: [f64; MAX_WIDTH] = [0.0; MAX_WIDTH];
    let noise = opensimplex::OsnContext::new(1).unwrap();
    const FREQUENCY_NOISE: f64 = 5.88;

    for x in 0..width {
        for y in 0..height {
            let nx: f64 = x as f64 / width as f64 - 0.5;
            let ny: f64 = y as f64 / height as f64 - 0.5;
            let f: f64 = 1.0 * noise.noise2(1.0 * FREQUENCY_NOISE * nx, 1.0 * FREQUENCY_NOISE * ny);
            let f1: f64 = 0.5 * noise.noise2(0.5 * FREQUENCY_NOISE * nx, 0.5 * FREQUENCY_NOISE * ny);
            let f2: f64 = 0.25 * noise.noise2(0.25 * FREQUENCY_NOISE * nx, 0.25 * FREQUENCY_NOISE * ny);
            let e: f64 = f + f1 + f2;
            let n = e.powf(2.75);

            map_original[y as usize * width as usize + x as usize] = n;
        }
    }

    /*
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
    }*/

    map_original
}
