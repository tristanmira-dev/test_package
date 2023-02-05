use bevy::prelude::*;

#[derive(Component, DerefMut, Deref)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_system(animate_sprite)
    .run();
}

fn animate_sprite(time: Res<Time>, texture_atlases: Res<Assets<TextureAtlas>>, mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>) {
    for (mut timer, mut sprite, handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len(); 
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle = asset_server.load("samurai_animations.png");

    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(60.0, 70.0), 5, 1, None, Option::Some(Vec2::new(40.0, 70.0)));

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        (
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                transform: Transform::from_scale(Vec3::splat(3.0)),
                ..SpriteSheetBundle::default()
            },
            AnimationTimer(
                Timer::from_seconds(0.1, TimerMode::Repeating)
            )
        ));
}