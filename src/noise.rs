use crate::prelude::*;
// use indicatif::ProgressIterator;
use noise::*;
// use rayon::prelude::{ParallelBridge, ParallelIterator};

// pub type Height = f64;
pub type HeightMap = ndarray::Array2<f64>;

//////////////////////////////////////////////////////////////////////////////////////////////
// Height Maps
//////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NoiseMap {
    pub low: f64,
    pub high: f64,

    pub octaves: usize,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
}

impl NoiseMap {
    pub fn new(
        octaves: usize,
        persistence: f64,
        frequency: f64,
        lacunarity: f64,
        lh_shape: (f64, f64),
    ) -> Self {
        Self { low: lh_shape.0, high: lh_shape.1, octaves, frequency, lacunarity, persistence }
    }

    pub fn generate(
        &self,
        shape: (usize, usize),
        noise_fn: Box<dyn NoiseFn<f64, 2>>,
    ) -> HeightMap {
        ndarray::Array2::from_shape_fn(shape, |(x, y)| {
            self.sum_octaves((x, y), |[x, y]| noise_fn.get([x, y]))
        })
    }

    fn sum_octaves(&self, point: (usize, usize), noise_fn: impl Fn([f64; 2]) -> f64) -> f64 {
        let mut amp = 1.0;
        let mut max_amp = 0.0;
        let mut freq = self.frequency;

        let mut noise = 0.0;
        for _ in 0..self.octaves {
            noise += noise_fn([point.0 as f64 * freq, point.1 as f64 * freq]) * amp;
            max_amp += amp;
            amp *= self.persistence;
            freq *= self.lacunarity;
        }

        (noise / max_amp) * (self.high - self.low) / 2.0 + (self.high + self.low) / 2.0
    }
}

// //////////////////////////////////////////////////////////////////////////////////////////////
// // Biome
// //////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum Biome {
    /// From deepest part of the map -> highest part of the map
    #[default]
    DeepWater,
    Water,
    Sand,
    Dirt,
    Grass,

    Mountain,
    Snow,
}

// fn get_biome_color(tt: &Biome) -> (FontCharType, (u8, u8, u8)) {
//     match tt {
//         Biome::Grass => (to_cp437('"'), GREEN),
//         Biome::Dirt => (to_cp437('d'), SADDLEBROWN),

//         Biome::Water => (to_cp437('~'), CYAN),
//         Biome::DeepWater => (to_cp437('~'), BLUE),

//         Biome::Sand => (to_cp437('░'), YELLOW),

//         Biome::LightForest => (to_cp437('¶'), LIGHTGREEN),

//         Biome::Mountain => (to_cp437('^'), SLATEGRAY),

//         Biome::Snow => (to_cp437('s'), WHITE),
//     }
// }

fn get_tt_color(tt: &TileType) -> (FontCharType, (u8, u8, u8)) {
    match tt {
        TileType::Grass => (to_cp437('"'), GREEN),
        TileType::Dirt => (to_cp437('d'), SADDLEBROWN),

        TileType::Water => (to_cp437('~'), CYAN),
        TileType::DeepWater => (to_cp437('~'), BLUE),

        TileType::Sand => (to_cp437('░'), YELLOW),

        TileType::Mountain => (to_cp437('^'), SLATEGRAY),

        TileType::Snow => (to_cp437('s'), WHITE),
        TileType::DownStairs => todo!(),
        TileType::UpStairs => todo!(),
        TileType::Floor => todo!(),
        TileType::Wall => todo!(),
        TileType::Door => todo!(),
    }
}

// pub fn get_biome_at(height: f64, moisture: f64) -> Biome {
//     match (height, moisture) {
//         (h, _) if h < 0.38 => Biome::DeepWater,
//         (h, _) if h < 0.42 => Biome::Water,
//         (h, m) if h < 0.46 && b < 0.57 => Biome::Sand,
//         (h, m) if h < 0.47 && b >= 0.6 => Biome::Dirt,
//         (h, m) if h > 0.54 && b < 0.43 => Biome::Grass,
//         (h, m) if h < 0.62 && b >= 0.49 => Biome::LightForest,
//         (h, m) if h >= 0.68 && b >= 0.10 => Biome::Mountain,
//         (h, _) if h >= 0.79 => Biome::Snow,
//         _ => Biome::LightForest,
//     }
// }

pub fn get_tt_at(h: f64, m: f64) -> TileType {
    match (h, m) {
        (h, _) if h < 0.3 => TileType::DeepWater,
        (h, _) if h < 0.42 => TileType::Water,
        (h, m) if h < 0.46 && m < 0.57 => TileType::Sand,
        (h, m) if h < 0.46 && m >= 0.6 => TileType::Dirt,
        (h, m) if h > 0.54 && m < 0.45 && h < 0.62 => TileType::Grass,
        (h, m) if h >= 0.68 && m > 0.10 => TileType::Mountain,
        (h, _) if h >= 0.79 => TileType::Snow,
        _ => TileType::Sand,
    }

    // match (h, m) {
    //     (h, _) if h < 0.38 => TileType::DeepWater,
    //     (h, _) if h < 0.42 => TileType::Water,
    //     (h, m) if h < 0.46 && m < 0.57 => TileType::Sand,
    //     (h, m) if h < 0.46 && m >= 0.6 => TileType::Dirt,
    //     (h, m) if h > 0.54 && m < 0.43 && h < 0.62 => TileType::Grass,
    //     (h, m) if h >= 0.68 && m >= 0.10 => TileType::Mountain,
    //     (h, _) if h >= 0.79 => TileType::Snow,
    //     _ => TileType::Grass,
    // }
}

//////////////////////////////////////////////////////////////////////////////////////////////
// Noise
//////////////////////////////////////////////////////////////////////////////////////////////

pub trait GradientFn: Send + Sync {
    fn distance(&self, x: f64, y: f64) -> f64;
}

impl GradientFn for GradientType {
    fn distance(&self, x: f64, y: f64) -> f64 {
        let nx = 2. * x as f64 / SCREEN_WIDTH as f64 - 1.;
        let ny = 2. * y as f64 / SCREEN_HEIGHT as f64 - 1.;

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
    pub octaves: usize,
    pub persistence: f64,
    pub lacunarity: f64,

    pub moisture_map_sub: f32,
    pub moisture_map_frequency: f64,
    pub moisture_map_gradient_mult: f32,

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
            low: 0.0,
            high: 1.0,
            octaves: 7,
            lacunarity: 2.0,
            persistence: 0.5,
            gradient_fn: Box::new(GradientType::Euclidean2),

            moisture_map_sub: 1.6,
            moisture_map_frequency: 0.02,
            moisture_map_gradient_mult: 0.4,

            height_map_mult: 1.2,
            height_map_frequency: 0.04,
            height_map_gradient_mult: 0.3,
        }
    }
}

//////////////////////////////////////////////////////////////////////////////////////////////

pub struct InternalNoiseMap {
    pub seed: u64,
    pub height_map: HeightMap,
    pub moisture_map: HeightMap,
    pub settings: NoiseSettings,
}

impl Default for InternalNoiseMap {
    fn default() -> Self {
        Self {
            seed: 0,
            settings: NoiseSettings::default(),
            moisture_map: HeightMap::zeros((SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize)),
            height_map: HeightMap::zeros((SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize)),
        }
    }
}
impl InternalNoiseMap {
    pub fn generate_gradient(&self) -> HeightMap {
        ndarray::Array2::from_shape_fn(
            (SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize),
            |(x, y)| self.settings.gradient_fn.distance(x as f64, y as f64),
        )
    }

    pub fn generate_height_map(&mut self, scale: f64) -> HeightMap {
        let open = noise::OpenSimplex::new(self.seed as u32);
        let perlin = noise::Simplex::new(self.seed as u32);
        let add = Add::new(open, perlin);

        NoiseMap::new(
            self.settings.octaves,
            self.settings.persistence,
            scale,
            self.settings.lacunarity,
            (self.settings.low, self.settings.high),
        )
        .generate((SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize), Box::new(add))
    }

    pub fn generate_maps(&mut self) -> TileMap {
        let gradient = self.generate_gradient();
        self.moisture_map = self.generate_height_map(self.settings.moisture_map_frequency);
        self.height_map = self.generate_height_map(self.settings.height_map_frequency);

        for ((x, y), v) in self.height_map.indexed_iter_mut() {
            *v = *v * self.settings.height_map_mult as f64
                - gradient.get((x, y)).unwrap() * self.settings.height_map_gradient_mult as f64
        }

        for ((x, y), v) in self.moisture_map.indexed_iter_mut() {
            *v -= (self.settings.moisture_map_sub as f64 - gradient.get((x, y)).unwrap())
                * self.settings.moisture_map_gradient_mult as f64
        }

        let mut map = TileMap::new(
            grid_2d::Size::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32),
            0,
            "Nope",
        );

        for y in 0..SCREEN_HEIGHT as usize {
            for x in 0..SCREEN_WIDTH as usize {
                let height = self.height_map.get((x, y)).unwrap();
                let moisture = self.moisture_map.get((x, y)).unwrap();
                *map.tiles.get_checked_mut(Coord::new(x as i32, y as i32)) =
                    get_tt_at(*height, *moisture);
            }
        }

        map
    }
}

// //////////////////////////////////////////////////////////////////////////////////////////////

pub fn render_noise(
    ctx: Res<BracketContext>,
    map: Res<TileMap>,
    camera: Res<CameraView>,
    renderables: Query<(&Glyph, &Position)>,
) {
    let mut batch = ctx.new_draw_batch();
    batch.target(LAYER_ZERO);
    batch.cls();

    // for x in 0..SCREEN_WIDTH as i32 {
    //     for y in 0..SCREEN_HEIGHT as i32 {
    //         let tt = map.tiles.get(Coord::new(x, y)).unwrap();
    //         let (glyph, color) = get_tt_color(tt);
    //         ctx.set(x as i32, y as i32, color, BLACK, glyph);
    //         batch.set(, color, glyph)
    //     }
    // }

    camera.viewport.for_each(|pt| {
        let coord = pt.to_coord();
        let screen_pt = camera.world_to_screen(pt);
        if map.in_bounds(coord) && map.is_revealed(coord) {
            let tt = map.tiles.get(coord).unwrap();
            let (glyph, color) = get_tt_color(tt);
            batch.set(screen_pt, ColorPair::new(color, BLACK), glyph);
        } else {
            let tt = TileType::DeepWater;
            let (glyph, color) = get_tt_color(&tt);
            batch.set(screen_pt, ColorPair::new(color, BLACK), glyph);
        }
    });

    let mut entities = renderables.iter().collect::<Vec<_>>();
    entities.sort_by(|&a, &b| b.0.render_order.cmp(&a.0.render_order));
    for (glyph, pos) in entities {
        if map.is_visible(**pos) {
            let screen_pt = camera.world_to_screen(pos.to_point());
            batch.set(screen_pt, glyph.color, glyph.glyph);
        }
    }

    ctx.submit_batch(BATCH_ZERO, batch);
}
