use super::*;

pub struct DoglegCorridors {}

impl MapArchitect for DoglegCorridors {
    fn build_map(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        self.corridors(builder, rng);
    }
}

impl DoglegCorridors {
    pub fn new() -> Box<DoglegCorridors> {
        Box::new(DoglegCorridors {})
    }

    fn corridors(&mut self, builder: &mut MapBuilder, rng: &RandomNumbers) {
        let rooms: Vec<Rect>;
        if let Some(rooms_builder) = &builder.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Dogleg Corridors require a builder with room structures");
        }

        let mut corridors: Vec<Vec<usize>> = Vec::new();
        for (i, room) in rooms.iter().enumerate() {
            if i > 0 {
                let Point { x: new_x, y: new_y } = room.center();
                let Point { x: prev_x, y: prev_y } = rooms[i as usize - 1].center();

                if rng.range(0, 2) == 1 {
                    let mut c1 = builder.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    let mut c2 = builder.apply_vertical_tunnel(prev_y, new_y, new_x);
                    c1.append(&mut c2);
                    corridors.push(c1);
                } else {
                    let mut c1 = builder.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    let mut c2 = builder.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    c1.append(&mut c2);
                    corridors.push(c1);
                }
            }
        }
        builder.corridors = Some(corridors);
    }
}
