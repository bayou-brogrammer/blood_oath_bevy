use super::*;

mod common;
mod constraints;
mod solver;

use common::*;
use constraints::*;
use solver::*;

/// Provides a map builder using the Wave Function Collapse algorithm.
pub struct WaveformCollapseBuilder {}

impl MapArchitect for WaveformCollapseBuilder {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        self.build(builder, rng);
    }
}

impl WaveformCollapseBuilder {
    /// Constructor for waveform collapse.
    pub fn new() -> Box<WaveformCollapseBuilder> {
        Box::new(WaveformCollapseBuilder {})
    }

    fn build(&mut self, builder: &mut MapBuilder, rng: &mut RandomNumbers) {
        const CHUNK_SIZE: i32 = 8;

        let patterns = build_patterns(&builder.map, CHUNK_SIZE, true, true);
        let constraints = patterns_to_constraints(patterns, CHUNK_SIZE);
        self.render_tile_gallery(&constraints, CHUNK_SIZE, builder);

        let old_map = builder.map.clone();

        builder.map = Map::new(builder.map.level, builder.width, builder.height, &builder.map.name);
        builder.spawn_list.clear();
        builder.rooms = None;
        builder.corridors = None;
        let mut tries = 0;
        loop {
            let mut solver = Solver::new(constraints.clone(), CHUNK_SIZE, &builder.map);
            while !solver.iteration(&mut builder.map, rng) {}

            if solver.possible {
                break;
            } // If it has hit an impossible condition, try again
            tries += 1;
            if tries > 10 {
                break;
            }
        }

        if tries > 10 {
            // Restore the old one
            builder.map = old_map;
        }
    }

    fn render_tile_gallery(&mut self, constraints: &[MapChunk], chunk_size: i32, builder: &mut MapBuilder) {
        builder.map = Map::new(builder.map.level, builder.width, builder.height, &builder.map.name);
        let mut counter = 0;
        let mut x = 1;
        let mut y = 1;
        while counter < constraints.len() {
            render_pattern_to_map(&mut builder.map, &constraints[counter], chunk_size, x, y);

            x += chunk_size + 1;
            if x + chunk_size > builder.map.width {
                // Move to the next row
                x = 1;
                y += chunk_size + 1;

                if y + chunk_size > builder.map.height {
                    // Move to the next page

                    builder.map =
                        Map::new(builder.map.level, builder.width, builder.height, &builder.map.name);

                    x = 1;
                    y = 1;
                }
            }

            counter += 1;
        }
    }
}
