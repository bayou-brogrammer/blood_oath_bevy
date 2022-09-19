use super::*;
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameSymbol {
    Floor,
    WallPillar,
    WallN,
    WallE,
    WallS,
    WallW,
    WallNe,
    WallNs,
    WallNw,
    WallEs,
    WallEw,
    WallSw,
    WallNes,
    WallNew,
    WallNsw,
    WallEsw,
    WallNesw,
    WallOther,
    DownStairs,
    UpStairs,
    Player,
}

impl Symbol for GameSymbol {
    fn text_fallback(self) -> char {
        use GameSymbol::*;

        match self {
            Floor => '·',
            WallPillar => '■',
            WallN => '║',
            WallE => '═',
            WallS => '║',
            WallW => '═',
            WallNe => '╚',
            WallNs => '║',
            WallNw => '╝',
            WallEs => '╔',
            WallEw => '═',
            WallSw => '╗',
            WallNes => '╠',
            WallNew => '╩',
            WallNsw => '╣',
            WallEsw => '╦',
            WallNesw => '╬',
            WallOther => '#',
            DownStairs => '>',
            UpStairs => '<',
            Player => '@',
        }
    }
}

pub fn one_bit_kenny_tileset_info() -> TilesetInfo<GameSymbol> {
    let mut symbol_map: HashMap<GameSymbol, (i32, i32)> = HashMap::new();
    {
        use GameSymbol::*;

        symbol_map.insert(Floor, (0, 0));

        symbol_map.insert(WallE, (18, 0));
        symbol_map.insert(WallEs, (18, 0));
        symbol_map.insert(WallEsw, (18, 0));
        symbol_map.insert(WallEw, (18, 0));
        symbol_map.insert(WallN, (18, 0));
        symbol_map.insert(WallNe, (18, 0));
        symbol_map.insert(WallNes, (18, 0));
        symbol_map.insert(WallNesw, (18, 0));
        symbol_map.insert(WallNew, (18, 0));
        symbol_map.insert(WallNes, (18, 0));
        symbol_map.insert(WallNs, (18, 0));
        symbol_map.insert(WallNsw, (18, 0));
        symbol_map.insert(WallNw, (18, 0));

        symbol_map.insert(Player, (24, 0));
    }

    TilesetInfo::<GameSymbol> {
        image_path: PathBuf::from("resources/1bit.png"),
        tile_size: (16, 16).into(),
        tile_start: (0, 0).into(),
        tile_gap: (0, 0).into(),
        // font_map,
        symbol_map,
    }
}
