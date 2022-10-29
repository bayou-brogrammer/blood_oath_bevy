use crate::prelude::*;
use noise::{utils::NoiseMap, *};

pub type Height = f64;
pub type HeightMap = ndarray::Array2<f64>;

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

pub trait GradientFn: Send + Sync {
    fn distance(&self, nx: f64, ny: f64) -> f64;
}

impl GradientFn for GradientType {
    fn distance(&self, nx: f64, ny: f64) -> f64 {
        match self {
            GradientType::SquareBump => 1.0 - (1.0 - nx.powi(2)) * (1.0 - ny.powi(2)),
            GradientType::DistanceSquared => 1.0 - (nx.powi(2) + ny.powi(2)),
            GradientType::Squircle => 1.0 - (nx.powi(4) + ny.powi(4)).sqrt(),
            GradientType::Hyperboloid => {
                1.0 - (nx.powi(2) + ny.powi(2) + 0.2_f64.powi(2)).sqrt()
            }
            GradientType::Euclidean2 => {
                f64::min(1.0, (nx.powi(2) + ny.powi(2)) / std::f64::consts::SQRT_2)
            }
        }
    }
}

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub enum GradientType {
    #[default]
    Euclidean2,
    Squircle,
    SquareBump,
    Hyperboloid,
    DistanceSquared,
}

pub struct NoiseSettings {
    pub octaves: i32,
    pub persistence: f64,
    pub lacunarity: f64,

    pub biome_map_sub: f32,
    pub biome_map_frequency: f64,
    pub biome_map_gradient_mult: f32,

    pub height_map_mult: f32,
    pub height_map_frequency: f64,
    pub height_map_gradient_mult: f32,

    pub low: f64,
    pub high: f64,

    pub gradient_fn: Box<dyn GradientFn>,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            low: 0.4,
            high: 1.0,
            octaves: 8,
            lacunarity: 1.8,
            persistence: 0.55,
            gradient_fn: Box::new(GradientType::Euclidean2),

            biome_map_sub: 1.6,
            biome_map_frequency: 0.02,
            biome_map_gradient_mult: 0.4,

            height_map_mult: 0.9,
            height_map_frequency: 0.03,
            height_map_gradient_mult: 0.4,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

pub struct InternalNoiseMap {
    pub seed: u64,
    pub height_map: HeightMap,
    pub biome_map: HeightMap,
    pub settings: NoiseSettings,
}

impl Default for InternalNoiseMap {
    fn default() -> Self {
        Self {
            seed: 0,
            settings: NoiseSettings::default(),
            biome_map: HeightMap::zeros((SCREEN_WIDTH, SCREEN_HEIGHT)),
            height_map: HeightMap::zeros((SCREEN_WIDTH, SCREEN_HEIGHT)),
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

    pub fn generate_gradient(&self) -> HeightMap {
        ndarray::Array2::from_shape_fn((SCREEN_WIDTH, SCREEN_HEIGHT), |(x, y)| {
            self.settings.gradient_fn.distance(x as f64, y as f64)
        })
    }

    pub fn generate_height_map(&mut self, scale: f64) -> HeightMap {
        let perlin = noise::Perlin::new(self.seed as u32);
        ndarray::Array2::from_shape_fn((SCREEN_WIDTH, SCREEN_HEIGHT), |(x, y)| {
            self.sum_octaves(self.settings.octaves, (x, y), scale, |[x, y]| perlin.get([x, y]))
        })
    }

    pub fn generate_maps(&mut self) {
        println!("Generating height map from seed {0}", self.seed);
        let gradient = self.generate_gradient();
        let mut nm = NoiseMap::new(SCREEN_WIDTH, SCREEN_HEIGHT);
        nm.iter_mut().enumerate().for_each(|(idx, v)| {
            let x = idx % SCREEN_WIDTH;
            let y = idx / SCREEN_WIDTH;
            *v = gradient[(x, y)]
        });
        nm.write_to_file("gradient.png");

        self.biome_map = self.generate_height_map(self.settings.biome_map_frequency);
        self.height_map = self.generate_height_map(self.settings.height_map_frequency);

        for ((x, y), v) in self.height_map.indexed_iter_mut() {
            *v = *v * self.settings.height_map_mult as f64
                - gradient.get((x, y)).unwrap() * self.settings.height_map_gradient_mult as f64
        }

        for ((x, y), v) in self.biome_map.indexed_iter_mut() {
            *v -= (self.settings.biome_map_sub as f64 - gradient.get((x, y)).unwrap())
                * self.settings.biome_map_gradient_mult as f64
        }

        // if self.height_map.get_value(x, y) < 0.0 {
        //     self.height_map.set_value(x, y, 0.0);
        // }
        // if self.biome_map.get_value(x, y) < 0.0 {
        //     self.biome_map.set_value(x, y, 0.0);
        // }
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
