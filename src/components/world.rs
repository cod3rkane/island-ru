extern crate opensimplex;

const MAX_MAP_ARRAY_SIZE: usize = 1296; // 36*36 our map size

pub fn create_random_world(width: usize, height: usize, seed: i64) -> [f64; MAX_MAP_ARRAY_SIZE] {
    let mut map: [f64; MAX_MAP_ARRAY_SIZE] = [0.0; MAX_MAP_ARRAY_SIZE];
    let noise = opensimplex::OsnContext::new(seed).unwrap();
    const FREQUENCY_NOISE: f64 = 2.28;

    for i in 0..height {
        for j in (0..width).rev() {
            let nx: f64 = j as f64 / width as f64 - 0.7;
            let ny: f64 = i as f64 / height as f64 - 0.5;
            let e: f64 = noise.noise2(FREQUENCY_NOISE * nx, FREQUENCY_NOISE * ny);
            let n: f64 = e.powf(1.24);

            map[j as usize * width as usize + i as usize] = n;
        }
    }

    map
}

pub fn create_random_trees(width: usize, height: usize, seed: i64) -> [f64; MAX_MAP_ARRAY_SIZE] {
    let mut map: [f64; MAX_MAP_ARRAY_SIZE] = [0.0; MAX_MAP_ARRAY_SIZE];
    let noise = opensimplex::OsnContext::new(seed).unwrap();
    const FREQUENCY_NOISE: f64 = 8.88;

    for i in 0..height {
        for j in (0..width).rev() {
            let nx: f64 = j as f64 / width as f64 - 0.7;
            let ny: f64 = i as f64 / height as f64 - 0.5;
            let e: f64 = noise.noise2(FREQUENCY_NOISE * nx, FREQUENCY_NOISE * ny);
            let n: f64 = e.powf(1.24);

            map[j as usize * width as usize + i as usize] = n;
        }
    }

    map
}

pub fn create_random_noise(width: usize, height: usize, seed: i64, frequency: f64) -> [f64; MAX_MAP_ARRAY_SIZE] {
    let mut map: [f64; MAX_MAP_ARRAY_SIZE] = [0.0; MAX_MAP_ARRAY_SIZE];
    let noise = opensimplex::OsnContext::new(seed).unwrap();

    for i in 0..height {
        for j in (0..width).rev() {
            let nx: f64 = j as f64 / width as f64 - 0.7;
            let ny: f64 = i as f64 / height as f64 - 0.5;
            let e: f64 = noise.noise2(frequency * nx, frequency * ny);
            let n: f64 = e.powf(1.8);

            map[j as usize * width as usize + i as usize] = n;
        }
    }

    map
}

