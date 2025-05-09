// tilemap.rs
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub fn create_tilemap(
    commands: &mut Commands,
    asset_server: Res<AssetServer>
) {
    let tilemap_size = TilemapSize { x: 256, y: 256 };
    let tilemap_entity = commands.spawn_empty().id();

    let texture_handle: Handle<Image> = asset_server.load("textures/terrain.png");

    let tile_size = TilemapTileSize { x: 32.0, y: 16.0 }; // Your tile size
    let grid_size = tile_size.into();
    let map_type = TilemapType::Isometric(IsoCoordSystem::Diamond);

    commands.entity(tilemap_entity).insert(MaterialTilemapBundle::<StandardTilemapMaterial> {
        grid_size,
        size: tilemap_size,
        storage: TileStorage::empty(tilemap_size),
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    fill_tilemap(
        commands,
        TilemapId(tilemap_entity),
        tilemap_size,
    );
}

fn fill_tilemap(
    commands: &mut Commands,
    tilemap_id: TilemapId,
    tilemap_size: TilemapSize,
) {
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let texture_index = if (x + y) % 3 == 0 {
                1
            } else if (x as i32 - y as i32).abs() < 3 && x > 5 && x < tilemap_size.x - 5 {
                2
            } else {
                0
            };
            commands.spawn(TileBundle {
                position: tile_pos,
                tilemap_id,
                texture_index: TileTextureIndex(texture_index as u32),
                ..Default::default()
            });
        }
    }
}