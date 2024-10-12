use bevy::prelude::*;
use bevy::prelude::Query;
use rand::seq::SliceRandom;
use crate::game::*;

const CHUNK_WIDTH: usize = 16*2;
const CHUNK_HEIGHT: usize = 16*2;
const TILE_SIZE: f32 = 16.0;
const CHUNK_SCALE: f32 = 1.0;
const CHUNK_SLANT_DEGREE: f32 = 10.0;


#[derive(Clone, Copy, PartialEq)]
enum TileType {
    Soil,
    DeadSoil,
    SalineSoil,
}

// Component to hold the tile type for an entity
#[derive(Component)]
struct Tile {
    tile_type: TileType,
}

//Assign texture to each tile type
fn get_tile_texture_path(tile_type: TileType) -> &'static str {
    match tile_type {
        TileType::Soil => "textures/tiles/tiletypes/Soil.png",
        TileType::DeadSoil => "textures/tiles/tiletypes/DeadSoil.png",
        TileType::SalineSoil => "textures/tiles/tiletypes/SalineSoil.png",
    }
}

// Neighbour constraints (for simplicity, only simple adjacency rules)
fn valid_neighbors(tile_type: TileType) -> Vec<TileType> {
    match tile_type {
        TileType::Soil => vec![TileType::Soil, TileType::DeadSoil],
        TileType::DeadSoil => vec![TileType::Soil, TileType::DeadSoil, TileType::SalineSoil],
        TileType::SalineSoil => vec![TileType::DeadSoil, TileType::SalineSoil],
    }
}

//using wavefunction collapse
pub fn generatechunk(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    playertransform: Query<(&PlayerMarker, &mut Transform)>
    ) {
    //2D Array of tiles
    let mut grid: [[Option<TileType>; CHUNK_WIDTH]; CHUNK_HEIGHT] = [[None; CHUNK_WIDTH]; CHUNK_HEIGHT];
    //Offset of array
    // Compute offsets to center the chunk around player
    let x_offset = (CHUNK_WIDTH as f32 * TILE_SIZE) / 2.0;
    let y_offset = (CHUNK_HEIGHT as f32 * TILE_SIZE) / 2.0;
  

        // Initialize grid with wave function collapse logic
        for y in 0..CHUNK_HEIGHT {
            for x in 0..CHUNK_WIDTH {
                // Get possible tile types based on neighbors
                let mut possible_tiles = vec![TileType::Soil, TileType::DeadSoil, TileType::SalineSoil];

                // Check neighboring tiles and restrict choices
                if x > 0 {
                    if let Some(left_tile) = grid[y][x - 1] {
                        possible_tiles.retain(|&t| valid_neighbors(left_tile).contains(&t));
                    }
                }

                if y > 0 {
                    if let Some(top_tile) = grid[y - 1][x] {
                        possible_tiles.retain(|&t| valid_neighbors(top_tile).contains(&t));
                    }
                }

                // Randomly select one of the possible tiles
                let tile_type = *possible_tiles.choose(&mut rand::thread_rng()).unwrap();
                grid[y][x] = Some(tile_type);

                // Get texture for the tile type
                let texture_handle = asset_server.load(get_tile_texture_path(tile_type));

                // Spawn the tile entity with its texture
                commands.spawn(SpriteBundle {
                    texture: texture_handle, 
                    transform: Transform {
                        translation: Vec3::new((x as f32 * TILE_SIZE) - x_offset, (y as f32 * TILE_SIZE) - y_offset,0.0),
                        scale: Vec3::splat(CHUNK_SCALE),
                        ..default()
                    },
                    ..Default::default()
                })
                .insert(Tile { tile_type });
            }
        }
    }
