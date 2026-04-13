use avian2d::prelude::*;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ecs_ldtk::prelude::*;

mod animation;
mod camera;
mod controller;
mod debug;
mod level;
mod player;
mod walls;

fn main() {
    let mut app = App::new();

    // Add Bevy plugins.
    app.add_plugins(
        DefaultPlugins
            // Prevent blur effect by changing default sampling.
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                // Wasm builds will check for meta files (that don't exist) if this isn't set.
                // This causes errors and even panics on web build on itch.
                // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Window {
                    title: "Platformer Demo".to_string(),
                    fit_canvas_to_parent: true,
                    ..default()
                }
                .into(),
                ..default()
            }),
    );

    // Order new `UpdateSystems` variants by adding them here:
    app.configure_sets(
        Update,
        (
            UpdateSystems::TickTimers,
            UpdateSystems::RecordInput,
            UpdateSystems::ApplyMovement,
            UpdateSystems::UpdateSprite,
        )
            .chain(),
    );

    app.add_plugins((
        animation::plugin,
        camera::plugin,
        debug::plugin,
        player::plugin,
        level::plugin,
        controller::CharacterControllerPlugin,
        walls::WallPlugin,
    ));

    app.add_plugins((
        LdtkPlugin,
        // Setup physics
        PhysicsPlugins::default().with_length_unit(100.0),
    ));

    app.insert_resource(LevelSelection::index(0));
    app.insert_resource(LdtkSettings {
        level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
            load_level_neighbors: true,
        },
        set_clear_color: SetClearColor::FromLevelBackground,
        ..default()
    });

    // Run the app !
    app.run();
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
enum UpdateSystems {
    TickTimers,
    RecordInput,
    ApplyMovement,
    UpdateSprite,
}
