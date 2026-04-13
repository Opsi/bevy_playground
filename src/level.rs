use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

use crate::player::PlayerSpawnPoint;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, startup)
        .add_observer(on_tiled_object_added);
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            TiledMap(asset_server.load("maps/test_map.tmx")),
            TilemapAnchor::Center,
        ))
        .observe(
            |collider_created: On<TiledEvent<ColliderCreated>>, mut commands: Commands| {
                commands
                    .entity(collider_created.event().origin)
                    .insert(RigidBody::Static);
            },
        );
}

fn on_tiled_object_added(
    add_tiled_object: On<Add, TiledObject>,
    object_query: Query<(&TiledObject, &TiledName)>,
    mut commands: Commands,
) {
    info!("on_tiled_object_added");
    let (tiled_object, tiled_name) = object_query.get(add_tiled_object.entity).unwrap();
    if tiled_name.0 == "PlayerSpawnPoint" && matches!(tiled_object, TiledObject::Point) {
        commands
            .entity(add_tiled_object.entity)
            .insert(PlayerSpawnPoint);
    }
}
