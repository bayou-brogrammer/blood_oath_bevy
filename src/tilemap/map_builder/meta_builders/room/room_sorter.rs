use super::*;

pub enum RoomSort {
    Leftmost,
    Rightmost,
    Topmost,
    Bottommost,
    Central,
}

pub struct RoomSorter {
    sort_by: RoomSort,
}

impl MapArchitect for RoomSorter {
    fn build_map(&mut self, builder: &mut MapBuilder, _rng: &RandomNumbers) {
        self.sorter(builder);
    }
}

impl RoomSorter {
    pub fn new(sort_by: RoomSort) -> Box<RoomSorter> {
        Box::new(RoomSorter { sort_by })
    }

    fn sorter(&mut self, builder: &mut MapBuilder) {
        match self.sort_by {
            RoomSort::Leftmost => {
                builder.rooms.as_mut().unwrap().sort_by(|a, b| a.x1.cmp(&b.x1))
            }
            RoomSort::Rightmost => {
                builder.rooms.as_mut().unwrap().sort_by(|a, b| b.x2.cmp(&a.x2))
            }
            RoomSort::Topmost => {
                builder.rooms.as_mut().unwrap().sort_by(|a, b| a.y1.cmp(&b.y1))
            }
            RoomSort::Bottommost => {
                builder.rooms.as_mut().unwrap().sort_by(|a, b| b.y2.cmp(&a.y2))
            }
            RoomSort::Central => {
                let map_center = Point::new(builder.map.width() / 2, builder.map.height() / 2);
                let center_sort = |a: &Rect, b: &Rect| {
                    let a_center = a.center();
                    let a_center_pt = Point::new(a_center.x, a_center.y);

                    let b_center = b.center();
                    let b_center_pt = Point::new(b_center.x, b_center.y);

                    let distance_a =
                        DistanceAlg::Pythagoras.distance2d(a_center_pt, map_center);
                    let distance_b =
                        DistanceAlg::Pythagoras.distance2d(b_center_pt, map_center);
                    distance_a.partial_cmp(&distance_b).unwrap()
                };

                builder.rooms.as_mut().unwrap().sort_by(center_sort);
            }
        }
    }
}
