//! Spawn the main level.

use bevy::prelude::*;

use crate::{asset_tracking::LoadResource, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
    #[dependency]
    pub scenes: Vec<Handle<Image>>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            scenes: vec![
                assets.load("images/scenes/scene1.png"),
                assets.load("images/scenes/scene2.png"),
                assets.load("images/scenes/scene3.png"),
                assets.load("images/scenes/scene4.png"),
            ],
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    info!("Spawning level");
    commands.spawn((
        Name::new("Level"),
        Transform::default(),
        Visibility::default(),
        DespawnOnExit(Screen::Gameplay),
        children![scene(&level_assets)],
    ));
}

#[derive(Component)]
pub struct SceneBackground {
    pub index: usize,
}

fn scene(level_assets: &LevelAssets) -> impl Bundle {
    info!("Spawning scene");
    (
        Name::new("Scene Background"),
        Sprite::from_image(level_assets.scenes[0].clone()),
        Transform::from_scale(Vec3::splat(2.0)),
        SceneBackground { index: 0 },
    )
}
