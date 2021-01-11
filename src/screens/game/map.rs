//! Helper struct that helps to parse .map files made only for Call of Ferris
//!
//! # Map
//! `[` => Create left tile \
//! `-` => Create center tile \
//! `]` => Create right tile \
//! `_` => Increase draw x by 100.0 \
//! `8` => Push a tile with a enemy \
//! `4` => Create a tile with the player \
//! `*` => Create a tile with a barrel \
//!
//! # Setter Syntax
//! `.comment` => A comment \
//! `.using_weapon` => Set the current weapon \
//! `.end` => The end quote displayed on the win screen

use crate::{
    game::components::{
        barrel::Barrel,
        bullet::WeaponType,
        enemy::Enemy,
        player::Player,
        tile::{Tile, TileType},
    },
    game::physics::Physics,
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

    pub weapon: WeaponType,
}

impl Map {
    pub fn parse(map_id: &str, physics: &mut Physics, asset_manager: &AssetManager) -> Self {
        let map = asset_manager.get_file(format!("/maps/{}.map", map_id).as_str());

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

        let mut weapon = WeaponType::Turbofish;

        for line in map.split("\n").collect::<Vec<_>>() {
            let exp = line.split(" ").collect::<Vec<_>>();

            if exp[0].starts_with(".end") {
                end = Some(exp[1..].join(" "));
            } else if exp[0].starts_with(".using_weapon") {
                let using_weapon = (exp[1..].join(" ").trim().to_string(), 1.0);

                weapon = match using_weapon.0.as_str() {
                    "Turbofish Gun" => WeaponType::Turbofish,
                    "Grappling Gun" => WeaponType::Grappling,
                    _ => panic!(""),
                };

                using = Some(using_weapon);
            } else if exp[0].starts_with(".comment") {
                // Do nothing. ¯\_(ツ)_/¯
            } else {
                for id in line.chars() {
                    match id {
                        '[' => {
                            let tile = Tile::new(draw_pos, physics, asset_manager, TileType::LEFT);

                            draw_inc = (tile.dimensions().x / 2.0) + 32.0;
                            draw_pos += draw_inc;

                            ground.push(tile);
                        }

                        '-' => {
                            let tile =
                                Tile::new(draw_pos, physics, asset_manager, TileType::CENTER);

                            draw_inc = (tile.dimensions().x / 2.0) + 32.0;
                            draw_pos += draw_inc;

                            ground.push(tile);
                        }

                        ']' => {
                            let tile = Tile::new(
                                (draw_pos - 32.0) + 20.0,
                                physics,
                                asset_manager,
                                TileType::RIGHT,
                            );

                            draw_inc = (tile.dimensions().x / 2.0) + 32.0;
                            draw_pos += draw_inc;

                            ground.push(tile);
                        }

                        '_' => {
                            draw_inc = 100.0;
                            draw_pos += draw_inc;
                        }

                        '8' => {
                            let tile =
                                Tile::new(draw_pos, physics, asset_manager, TileType::CENTER);

                            draw_inc = (tile.dimensions().x / 2.0) + 32.0;

                            ground.push(tile);
                            enemies.push(Enemy::new(draw_pos, physics, asset_manager));

                            draw_pos += draw_inc;
                            total_enemies += 1;
                        }

                        '4' => {
                            let tile =
                                Tile::new(draw_pos, physics, asset_manager, TileType::CENTER);

                            player = Some(Player::new(draw_pos, physics, asset_manager));

                            draw_inc = tile.dimensions().x;
                            draw_pos += draw_inc;

                            ground.push(tile);
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

            weapon,
        }
    }
}
