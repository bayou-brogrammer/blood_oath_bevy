use super::*;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn build(
        &mut self,
        width: i32,
        height: i32,
        depth: i32,
        rng: &RandomNumbers,
    ) -> MapBuilder {
        let mut mb = MapBuilder {
            rooms: Vec::new(),
            spawn_list: Vec::new(),
            player_start: Position::default(),
            map: TileMap::new(width, height, depth, "New Map"),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);

        mb.player_start = Position::from(mb.rooms[0].center());
        for room in mb.rooms.iter().skip(1) {
            mb.spawn_list.push(room.center().into());
        }

        mb
    }
}
