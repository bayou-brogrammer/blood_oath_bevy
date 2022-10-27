use crate::prelude::*;
use opensimplex_noise_rs::OpenSimplexNoise;

fn get_id_from_pos(x: i32, y: i32) -> usize {
    (x + SCREEN_WIDTH * y) as usize
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
        Biomes::Dirt => (to_cp437('░'), SANDY_BROWN),

        Biomes::Water => (to_cp437('~'), CYAN),
        Biomes::DeepWater => (to_cp437('~'), BLUE),

        Biomes::Sand => (to_cp437('░'), YELLOW),
        Biomes::WetSand => (to_cp437('░'), LIGHT_YELLOW),

        Biomes::DarkForest => (to_cp437('¶'), DARKGREEN),
        Biomes::LightForest => (to_cp437('¶'), LIGHTGREEN),

        Biomes::Mountain => (to_cp437('^'), SLATEGRAY),

        Biomes::Snow => (to_cp437('░'), WHITE),
    }
}

pub fn get_biome_at(height: f32, moisture: f32) -> Biomes {
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

    pub biome_map_mult: f32,
    pub biome_map_sub: f32,
    pub biome_map_frequency: f64,

    pub height_map_mult: f32,
    pub height_map_frequency: f64,

    pub gradient_mult: f32,
}

impl Default for NoiseSettings {
    fn default() -> Self {
        Self {
            octaves: 3,
            persistence: 0.5,
            gradient_mult: 0.8,
            biome_map_sub: 0.1,
            biome_map_mult: 0.4,
            height_map_mult: 1.1,
            biome_map_frequency: 0.007,
            height_map_frequency: 0.004,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

pub struct NoiseMap {
    pub seed: i64,
    pub height_map: Vec<f32>,
    pub biome_map: Vec<f32>,
    pub settings: NoiseSettings,
}

impl Default for NoiseMap {
    fn default() -> Self {
        Self {
            seed: 0,
            settings: NoiseSettings::default(),
            biome_map: vec![0.0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
            height_map: vec![0.0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize],
        }
    }
}
impl NoiseMap {
    #[allow(clippy::too_many_arguments)]
    fn sum_octaves(
        &self,
        num_iterations: i32,
        point: (i32, i32),
        scale: f64,
        low: f64,
        high: f64,
        noise_fn: impl Fn(f64, f64) -> f64,
    ) -> f64 {
        let mut max_amp = 0.0;
        let mut amp = 1.0;
        let mut freq = scale;
        let mut noise = 0.0;

        for _ in 0..num_iterations {
            noise += noise_fn(point.0 as f64 * freq, point.1 as f64 * freq) * amp;
            max_amp += amp;
            amp *= self.settings.persistence;
            freq *= 2.0;
        }

        (noise / max_amp) * (high - low) / 2.0 + (high + low) / 2.0
    }
    pub fn generate_gradient(&self) -> Vec<f32> {
        let mut gradient: Vec<f32> = vec![1.0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let mut color_value: f32;

                let a = if x > (SCREEN_WIDTH / 2) { SCREEN_WIDTH - x } else { x };
                let b = if y > SCREEN_HEIGHT / 2 { SCREEN_HEIGHT - y } else { y };

                let smaller = std::cmp::min(a, b) as f32;
                color_value = smaller / (SCREEN_WIDTH as f32 / 2.0);

                color_value = 1.0 - color_value;
                color_value = color_value * color_value;

                gradient[get_id_from_pos(x, y)] = match color_value - 0.1 {
                    x if x > 1.0 => 1.0,
                    x if x < 0.0 => 0.0,
                    x => x,
                };
            }
        }

        gradient
    }

    pub fn generate_noise_map(&mut self, scale: f64) -> Vec<f32> {
        let noise_generator = OpenSimplexNoise::new(Some(self.seed));

        let mut noise_map = vec![0.0; (SCREEN_WIDTH * SCREEN_HEIGHT) as usize];
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let val = self.sum_octaves(
                    self.settings.octaves,
                    (x, y),
                    scale,
                    0.0,
                    1.0,
                    |x, y| noise_generator.eval_2d(x, y),
                );

                noise_map[get_id_from_pos(x, y)] = val as f32;
            }
        }

        noise_map
    }

    pub fn generate_maps(&mut self) {
        let gradient = self.generate_gradient();
        self.biome_map = self.generate_noise_map(self.settings.biome_map_frequency);
        self.height_map = self.generate_noise_map(self.settings.height_map_frequency);

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                self.height_map[get_id_from_pos(x, y)] =
                    self.height_map[get_id_from_pos(x, y)] * self.settings.height_map_mult
                        - gradient[get_id_from_pos(x, y)] * self.settings.gradient_mult;

                self.biome_map[get_id_from_pos(x, y)] = self.biome_map[get_id_from_pos(x, y)]
                    - (self.settings.biome_map_sub - gradient[get_id_from_pos(x, y)])
                        * self.settings.biome_map_mult;

                if self.height_map[get_id_from_pos(x, y)] < 0.0 {
                    self.height_map[get_id_from_pos(x, y)] = 0.0;
                }
                if self.biome_map[get_id_from_pos(x, y)] < 0.0 {
                    self.biome_map[get_id_from_pos(x, y)] = 0.0;
                }
            }
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

pub fn render_noise(ctx: Res<BracketContext>, nm: Res<NoiseMap>) {
    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let height = nm.height_map[get_id_from_pos(x, y)];
            let moisture = nm.biome_map[get_id_from_pos(x, y)];

            let biome = get_biome_at(height, moisture);
            let (glyph, color) = get_biome_color(biome);
            ctx.set(x, y, color, BLACK, glyph);
        }
    }
}
