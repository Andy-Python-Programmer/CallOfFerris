use crate::{
    components::{
        barrel::Barrel,
        enemy::Enemy,
        player::Player,
        tile::{Tile, TileType},
    },
    physics::Physics,
    utils::AssetManager,
};

pub struct Map {
    pub ground: Vec<Tile>,
    pub enemies: Vec<Enemy>,
    pub barrels: Vec<Barrel>,
    pub player: Player,

    pub total_enemies: i32,

    pub end: Option<String>,
    pub using: Option<(String, f32)>,
}

impl Map {
    pub fn parse(map: String, physics: &mut Physics, asset_manager: &AssetManager) -> Self {
        let mut draw_pos = 0.;

        #[allow(unused_assignments)]
        let mut draw_inc = 64.;

        let mut ground = vec![];
        let mut enemies = vec![];
        let mut total_enemies = 0;
        let mut barrels = vec![];

        let mut player = None;

        let mut end = None;
        let mut using = None;

        for line in map.split("\n").collect::<Vec<_>>() {
            let exp = line.split(" ").collect::<Vec<_>>();

            if exp[0].starts_with(".end") {
                end = Some(exp[1..].join(" "));
            } else if exp[0].starts_with(".using") {
                using = Some((exp[1..].join(" ").trim().to_string(), 1.0));
            } else if exp[0].starts_with(".comment") {
                // Do nothing. ¯\_(ツ)_/¯
            } else {
                for id in line.chars() {
                    match id {
                        '[' => {
                            let tile = Tile::new(draw_pos, physics, asset_manager, TileType::LEFT);

                            draw_inc = tile.dimensions().x;
                            ground.push(tile);

                            draw_pos += draw_inc;
                        }

                        '-' => {
                            let tile =
                                Tile::new(draw_pos, physics, asset_manager, TileType::CENTER);

                            draw_inc = tile.dimensions().x;
                            ground.push(tile);

                            draw_pos += draw_inc;
                        }

                        ']' => {
                            let tile = Tile::new(draw_pos, physics, asset_manager, TileType::RIGHT);

                            draw_inc = tile.dimensions().x;
                            ground.push(tile);

                            draw_pos += draw_inc;
                        }

                        '_' => {
                            draw_inc = 100.0;
                            draw_pos += draw_inc;
                        }

                        '8' => {
                            let tile =
                                Tile::new(draw_pos, physics, asset_manager, TileType::CENTER);

                            draw_inc = tile.dimensions().x;

                            ground.push(tile);
                            enemies.push(Enemy::new(draw_pos, physics, asset_manager));

                            draw_pos += draw_inc;
                            total_enemies += 1;
                        }

                        '4' => {
                            let tile =
                                Tile::new(draw_pos, physics, asset_manager, TileType::CENTER);

                            draw_inc = tile.dimensions().x;

                            ground.push(tile);
                            player = Some(Player::new(draw_pos, physics, asset_manager));

                            draw_pos += draw_inc;
                        }

                        '*' => {
                            let tile =
                                Tile::new(draw_pos, physics, asset_manager, TileType::CENTER);

                            draw_inc = tile.dimensions().x;

                            ground.push(tile);
                            barrels.push(Barrel::new(draw_pos, physics, asset_manager));

                            draw_pos += draw_inc;
                        }

                        _ => {}
                    }
                }
            }
        }

        let player = player.unwrap();

        Self {
            ground,
            enemies,
            total_enemies,
            barrels,

            player,

            end,
            using,
        }
    }
}
