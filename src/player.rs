//! Player-specific behavior.

use std::time::Duration;

use crate::{
    UpdateSystems,
    animation::{Animation, AnimationState},
    controller::{CharacterControllerBundle, MovementAction, MovementEvent},
};
use avian2d::{math::*, prelude::*};
use bevy::{prelude::*, sprite::Anchor};
use bevy_ecs_ldtk::prelude::*;

const PLAYER_SPRITE_FILE: &str = "sprites/character/spritesheet-characters-double.png";

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.register_type::<PlayerSpawnPoint>();

    // Record directional input as movement controls.
    app.add_systems(
        Update,
        record_player_directional_input.in_set(UpdateSystems::RecordInput),
    );
    app.register_ldtk_entity::<PlayerSpawnPointBundle>("PlayerSpawnPoint");
    app.add_observer(spawn_player_at_spawn_point);
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[require(Transform, Visibility)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[require(Transform)]
#[reflect(Component)]
pub struct PlayerSpawnPoint;

#[derive(Bundle, Default, LdtkEntity)]
struct PlayerSpawnPointBundle {
    spawn_point: PlayerSpawnPoint,
}

fn spawn_player_at_spawn_point(
    add_player_spawn: On<Add, PlayerSpawnPoint>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    player_spawn_query: Query<&Transform, With<PlayerSpawnPoint>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if !player_query.is_empty() {
        return;
    }

    let mut spawn_transform = *player_spawn_query
        .get(add_player_spawn.event().entity)
        .unwrap();

    // --- SCALE ADJUSTMENT ---
    let scale_factor = 0.2; // Adjust this value as needed
    spawn_transform.scale = Vec3::splat(scale_factor);
    // ------------------------

    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(256), 7, 7, Some(UVec2::splat(1)), None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let player_animation = Animation::default()
        .add_state(AnimationState::Idling, Duration::from_millis(500), vec![3])
        .add_state(
            AnimationState::Walking,
            Duration::from_millis(100),
            vec![7, 8],
        );

    commands.spawn((
        Name::new("Player"),
        Player,
        spawn_transform,
        Sprite {
            image: asset_server.load(PLAYER_SPRITE_FILE),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout,
                index: 6,
            }),
            ..Default::default()
        },
        Anchor::from(Vec2::new(0., -0.2)),
        player_animation,
        // Adjust Collider dimensions to match new scale (e.g. 25. instead of 50.)
        CharacterControllerBundle::new(Collider::capsule(18., 18.)).with_movement(
            5000.,
            0.9,
            800.,
            PI * 0.45,
        ),
    ));
}

/// Sends [`MovementAction`] events based on keyboard input.
fn record_player_directional_input(
    mut movement_message_writer: MessageWriter<MovementEvent>,
    player_query: Query<Entity, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(player_entity) = player_query.single() else {
        return;
    };

    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let direction = horizontal as Scalar;

    if direction != 0.0 {
        movement_message_writer.write(MovementEvent {
            entity: player_entity,
            action: MovementAction::Move(direction),
        });
    }

    if keyboard_input.any_pressed([KeyCode::Space, KeyCode::KeyW, KeyCode::ArrowUp]) {
        movement_message_writer.write(MovementEvent {
            entity: player_entity,
            action: MovementAction::Jump,
        });
    }
}
