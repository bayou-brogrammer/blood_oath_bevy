use crate::prelude::*;
use noise::*;

pub type Height = f64;
pub type HeightMap<H> = ndarray::Array2<H>;
pub type HeightMap64 = HeightMap<f64>;

pub fn clip(height_map: &mut HeightMap64, min: f64, max: f64) {
    height_map.mapv_inplace(|v| {
        if v < min {
            min
        } else if v > max {
            max
        } else {
            v
        }
    })
}

pub fn clip_min(height_map: &mut HeightMap64, value: f64) {
    clip(height_map, value, f64::INFINITY)
}

pub fn clip_max(height_map: &mut HeightMap64, value: f64) {
    clip(height_map, f64::NEG_INFINITY, value)
}

//////////////////////////////////////////////////////////////////////////////////////////////
// Biomes
//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Eq, PartialEq)]
pub enum Biomes {
    Grass,
    DeepWater,
    Water,
    Dirt,
    Sand,
    WetSand,
    DarkForest,
    LightForest,
    Mountain,
    Snow,
}

fn get_biome_color(biome: Biomes) -> (FontCharType, (u8, u8, u8)) {
    match biome {
        Biomes::Grass => (to_cp437('"'), GREEN),
        Biomes::Dirt => (to_cp437('d'), SADDLEBROWN),

        Biomes::Water => (to_cp437('~'), CYAN),
        Biomes::DeepWater => (to_cp437('~'), BLUE),

        Biomes::Sand => (to_cp437('░'), YELLOW),
        Biomes::WetSand => (to_cp437('░'), SANDY_BROWN),

        Biomes::DarkForest => (to_cp437('¶'), DARKGREEN),
        Biomes::LightForest => (to_cp437('¶'), LIGHTGREEN),

        Biomes::Mountain => (to_cp437('^'), SLATEGRAY),

        Biomes::Snow => (to_cp437('s'), WHITE),
    }
}

pub fn get_biome_at(height: f64, moisture: f64) -> Biomes {
    match (height, moisture) {
        (a, _) if a < 0.39 => Biomes::DeepWater,
        (a, _) if a < 0.42 => Biomes::Water,
        (a, b) if a < 0.46 && b < 0.57 => Biomes::Sand,
        (a, b) if a < 0.47 && b < 0.6 => Biomes::WetSand,
        (a, b) if a < 0.47 && b >= 0.6 => Biomes::Dirt,
        (a, b) if a > 0.54 && b < 0.43 && a < 0.62 => Biomes::Grass,
        (a, b) if a < 0.62 && b >= 0.49 => Biomes::LightForest,
        (a, b) if a >= 0.68 && b >= 0.10 => Biomes::Mountain,
        (a, _) if a >= 0.79 => Biomes::Snow,
        _ => Biomes::LightForest,
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////
// Noise
//////////////////////////////////////////////////////////////////////////////////////////////

#[derive(PartialEq, Clone)]
pub struct NoiseSettings {
    pub octaves: i32,
    pub persistence: f64,
    pub lacunarity: f64,

    pub biome_map_mult: f32,
    pub biome_map_sub: f32,
    pub biome_map_frequency: f64,

    pub height_map_mult: f32,
    pub height_map_frequency: f64,

    pub gradient_hm_mult: f32,

    pub low: f64,
    pub high: f64,

    pub set_zero: bool,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            set_zero: true,
            low: 0.4,
            high: 1.0,
            octaves: 8,
            lacunarity: 1.8,
            persistence: 0.55,

            biome_map_sub: 1.6,
            biome_map_mult: 0.4,
            biome_map_frequency: 0.02,

            gradient_hm_mult: 0.4,
            height_map_mult: 0.9,
            height_map_frequency: 0.03,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

pub struct InternalNoiseMap {
    pub seed: i64,

    pub height_map: HeightMap64,
    pub biome_map: HeightMap64,

    pub settings: NoiseSettings,
}

impl Default for InternalNoiseMap {
    fn default() -> Self {
        Self {
            seed: 0,
            settings: NoiseSettings::default(),
            height_map: ndarray::Array2::zeros((SCREEN_WIDTH, SCREEN_HEIGHT)),
            biome_map: ndarray::Array2::zeros((SCREEN_WIDTH, SCREEN_HEIGHT)),
        }
    }
}
impl InternalNoiseMap {
    #[allow(clippy::too_many_arguments)]
    fn sum_octaves(
        &self,
        num_iterations: i32,
        point: (usize, usize),
        scale: f64,
        noise_fn: impl Fn([f64; 2]) -> f64,
    ) -> f64 {
        let mut max_amp = 0.0;
        let mut amp = 1.0;
        let mut freq = scale;
        let mut noise = 0.0;

        for _ in 0..num_iterations {
            noise += noise_fn([point.0 as f64 * freq, point.1 as f64 * freq]) * amp;
            max_amp += amp;
            amp *= self.settings.persistence;
            freq *= self.settings.lacunarity;
        }

        (noise / max_amp) * (self.settings.high - self.settings.low) / 2.0
            + (self.settings.high + self.settings.low) / 2.0
    }

    pub fn generate_gradient(&self) -> HeightMap64 {
        ndarray::Array2::from_shape_fn((SCREEN_WIDTH, SCREEN_HEIGHT), |(x, y)| {
            let mut color_value: f64;

            let a = if x > (SCREEN_WIDTH / 2) { SCREEN_WIDTH - x } else { x };
            let b = if y > SCREEN_HEIGHT / 2 { SCREEN_HEIGHT - y } else { y };

            let smaller = std::cmp::min(a, b) as f64;
            color_value = smaller / (SCREEN_WIDTH as f64 / 2.0);

            color_value = 1.0 - color_value;
            color_value = color_value * color_value;

            color_value
        })
    }

    pub fn generate_height_map(&mut self, scale: f64) -> HeightMap64 {
        let perlin = noise::Perlin::new(self.seed as u32);
        ndarray::Array2::from_shape_fn((SCREEN_WIDTH, SCREEN_HEIGHT), |(x, y)| {
            self.sum_octaves(self.settings.octaves, (x, y), scale, |[x, y]| perlin.get([x, y]))
        })
    }

    pub fn generate_maps(&mut self) {
        let gradient = self.generate_gradient();
        self.height_map = self.generate_height_map(self.settings.height_map_frequency);
        self.biome_map = self.generate_height_map(self.settings.biome_map_frequency);

        self.height_map.indexed_iter_mut().for_each(|((x, y), v)| {
            *v = (*v * self.settings.height_map_mult as f64
                - *gradient.get([x, y]).unwrap() as f64
                    * self.settings.gradient_hm_mult as f64)
                .max(0.0)
        });

        self.biome_map.indexed_iter_mut().for_each(|((x, y), v)| {
            *v -= ((*v - self.settings.biome_map_sub as f64 - *gradient.get([x, y]).unwrap())
                * self.settings.biome_map_mult as f64)
                .max(0.0)
        });
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

pub fn render_noise(ctx: Res<BracketContext>, nm: Res<InternalNoiseMap>) {
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let height = nm.height_map.get((x, y)).unwrap();
            let moisture = nm.biome_map.get((x, y)).unwrap();

            let biome = get_biome_at(*height, *moisture);
            let (glyph, color) = get_biome_color(biome);
            ctx.set(x as i32, y as i32, color, BLACK, glyph);
        }
    }
}
