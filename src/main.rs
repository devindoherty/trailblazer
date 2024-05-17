use bevy::prelude::*;
use bevy::sprite::Anchor;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin{
                primary_window: Some(Window {
                    title: "Trailblazer".to_string(),
                    resolution: (640.0, 480.0).into(),
                    resizable: true,
                    ..default()
                }),
                ..default()
            })
        )
        .add_systems(Startup, setup)
        .add_systems(Update, animate_sprite)
        .run()
}

#[derive(Component)]
struct AnimateTranslation;

struct State;

struct GameState {
    menu: State,
    exploration: State,
    combat: State
}

#[derive(Component)]
struct Map {
    tile_map: Vec<Vec<i32>>, 
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("sprites.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 64, 64, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let animation_indices = AnimationIndices {first: 1, last: 500};

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
    )); 

    let font = asset_server.load("FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::RED,
    };

    commands.spawn((
        Text2dBundle {
            text: Text::from_section("Trailblazer", text_style.clone()),
                // .with_alignment(TextAlignment::Center),
            // text_anchor: Anchor::TopCenter,
            transform: Transform {
                translation: Vec3::new(0.0, 150.0, 0.0),
                ..default()
            },    
            ..default()
        },
        AnimateTranslation,
    ));

    let tiles = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];

    let map = Map {
        tile_map: tiles,
    };
}

fn animate_text(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
) {
    for mut transform in &mut query {
        transform.translation.x = 100.0 * time.elapsed_seconds().sin() - 400.0;
        transform.translation.y = 100.0 * time.elapsed_seconds().cos();
    }
}
