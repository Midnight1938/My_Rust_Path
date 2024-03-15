use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    sprite::MaterialMesh2dBundle,
    window::{PresentMode, WindowTheme},
};
use bevy::core::FrameCount;

const VIEWPORT_WIDTH: usize = 1280;
const VIEWPORT_HEIGHT: usize = 960;
const VIEWPORT_MAX_X: f32 = VIEWPORT_WIDTH as f32 / 2.0;
const VIEWPORT_MIN_X: f32 = -VIEWPORT_MAX_X;
const VIEWPORT_MAX_Y: f32 = VIEWPORT_HEIGHT as f32 / 2.0;
const VIEWPORT_MIN_Y: f32 = -VIEWPORT_MAX_Y;
const ASTEROID_VELOCITY: f32 = 2.0;
const BULLET_VELOCITY: f32 = 6.0;
const BULLET_DISTANCE: f32 = VIEWPORT_HEIGHT as f32 * 0.9;
const STARSHIP_ROTATION_SPEED: f32 = 5.0 * 2.0 * PI / 360.0;
const STARSHIP_ACCELERATION: f32 = 0.2;
const STARSHIP_DECELERATION: f32 = 0.01;
const STARSHIP_MAX_VELOCITY: f32 = 10.0;

fn main() {
    App::new().add_plugins((DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Asteroids!".into(),
            name: Some("asteroid.app".into()),
            resolution: (VIEWPORT_WIDTH as f32, VIEWPORT_HEIGHT as f32).into(),
            present_mode: PresentMode::AutoVsync,
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            enabled_buttons: bevy::window::EnabledButtons {
                maximize: false,
                ..Default::default()
            },
            visible: false,
            ..default()
        }),
        ..default()
    }),
    )).add_systems(Startup, setup).add_systems(
        Update,
        (
            sync_positions.after(update_pos),
            sync_ast_scale,
            sync_ship_rot,
            update_pos,
            bullet_timeout,
            slow_ships,
            asteroid_collide_ship,
            asteroid_collide_bullet,
            animate_rotation,
            keyboard_events,
            make_visible,
        ),
    ).run();
}
fn make_visible(mut window: Query<&mut Window>, frames: Res<FrameCount>) {
    if frames.0 == 3 {
        window.single_mut().visible = true;
    }
}
#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct AnimateRotation;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct Starship {
    rotation: f32,
}

impl Starship {
    fn direction(&self) -> Vec2 {
        let (y, x) = (self.rotation + PI / 2.).sin_cos();
        Vec2::new(x, y)
    }
}

#[derive(Component)]
struct Bullet {
    start: Vec2,
}

#[derive(Component)]
struct Asteroids {
    size: AsteroidSize,
}

impl AsteroidSize {
    fn scale(&self) -> f32 {
        match self {
            AsteroidSize::Big => 100.,
            AsteroidSize::Medium => 60.,
            AsteroidSize::Small => 30.,
        }
    }
}

#[derive(Clone, Copy)]
enum AsteroidSize {
    Big,
    Medium,
    Small,
}

fn create_ship_mesh() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    ).with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, 0.5, 0.0], [-0.25, -0.5, 0.0], [0.25, -0.5, 0.0]],
    ).with_inserted_indices(Indices::U32(vec![0, 1, 2])).with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 3]).with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.5, 0.0], [0.0, 1.0], [1.0, 1.0]],
    )
}

fn sync_positions(mut query: Query<(&Position, &mut Transform)>) {
    for (position, mut transform) in &mut query {
        transform.translation = Vec3::new(position.0.x, position.0.y, transform.translation.z);
    }
}

fn sync_ast_scale(mut query: Query<(&Asteroids, &mut Transform)>) {
    for (asteroid, mut transform) in &mut query {
        transform.scale = Vec3::splat(asteroid.size.scale())
    }
}

fn sync_ship_rot(mut query: Query<(&Starship, &mut Transform)>) {
    for (ship, mut transform) in &mut query {
        transform.rotation = Quat::from_rotation_z(ship.rotation)
    }
}

fn update_pos(mut query: Query<(&Velocity, &Transform, &mut Position)>) {
    for (velocity, transform, mut position) in &mut query {
        let mut new_pos = position.0 + velocity.0;
        let half_scale = transform.scale.max_element();
        if new_pos.x > VIEWPORT_MAX_X + half_scale {
            new_pos.x = VIEWPORT_MIN_X - half_scale;
        } else if new_pos.x < VIEWPORT_MIN_X - half_scale {
            new_pos.x = VIEWPORT_MAX_X + half_scale;
        }

        if new_pos.y > VIEWPORT_MAX_Y + half_scale {
            new_pos.y = VIEWPORT_MIN_Y - half_scale;
        } else if new_pos.y < VIEWPORT_MIN_Y - half_scale {
            new_pos.y = VIEWPORT_MAX_Y + half_scale;
        }

        position.0 = new_pos
    }
}

fn get_rand_pt() -> Vec2 {
    Vec2::new(
        (rand::random::<f32>() * 2. - 1.) * (VIEWPORT_WIDTH as f32) / 2.,
        (rand::random::<f32>() * 2. - 1.) * (VIEWPORT_HEIGHT as f32) / 2.,
    )
}

fn keyboard_events(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Starship, &Position, &mut Velocity)>,
) {
    for (mut starship, ship_position, mut velocity) in &mut query {
        if input.just_pressed(KeyCode::Space) {
            let (y, x) = (starship.rotation + PI / 2.0).sin_cos();
            commands.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(Circle::default()).into(),
                transform: Transform::default().with_scale(Vec3::splat(5.)).with_translation(Vec3::new(0., 0., 0.5)),
                material: materials.add(Color::rgba(0.8, 0.4, 0.8, 1.0)),
                ..default()
            }).insert(Bullet {
                start: ship_position.0.clone(),
            }).insert(Position(ship_position.0.clone())).insert(Velocity(Vec2::new(x, y).normalize() * BULLET_VELOCITY));
        }
        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            velocity.0 += starship.direction() * STARSHIP_ACCELERATION;
        }
        if velocity.0.length() > STARSHIP_MAX_VELOCITY {
            velocity.0 = velocity.0.normalize_or_zero() * STARSHIP_MAX_VELOCITY;
        }
        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            starship.rotation -= STARSHIP_ROTATION_SPEED;
        } else if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            starship.rotation += STARSHIP_ROTATION_SPEED;
        }
    }
}

fn bullet_timeout(mut commands: Commands, mut query: Query<(Entity, &mut Bullet, &mut Position)>) {
    for (entity, bullet, position) in &mut query {
        if (bullet.start - position.0).length() > BULLET_DISTANCE {
            commands.entity(entity).despawn();
        }
    }
}

fn slow_ships(input: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Starship>>) {
    if !(input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp)) {
        for mut velocity in &mut query {
            velocity.0 *= 1. - STARSHIP_DECELERATION;
        }
    }
}

fn asteroid_collide_bullet(mut commands: Commands,
                           mut meshes: ResMut<Assets<Mesh>>,
                           mut materials: ResMut<Assets<ColorMaterial>>,
                           bullets_query: Query<(Entity, &Transform, &Position), With<Bullet>>,
                           asteroid_query: Query<(Entity, &Asteroids, &Transform, &Position), With<Asteroids>>) {
    for (bullets_entity, bullet_transform, bullet_pos) in &bullets_query {
        for (asteroid_entity, asteroid, asteroid_transform, asteroid_pos) in &asteroid_query {
            let bullet_size = bullet_transform.scale.max_element();
            let asteroid_size = asteroid_transform.scale.max_element();
            let distance = (bullet_pos.0 - asteroid_pos.0).length();

            if distance < bullet_size / 2.0 + asteroid_size / 2.0 {
                commands.entity(bullets_entity).despawn();
                commands.entity(asteroid_entity).despawn();

                let asteroid_new = match asteroid.size {
                    AsteroidSize::Big => Some(AsteroidSize::Medium),
                    AsteroidSize::Medium => Some(AsteroidSize::Small),
                    AsteroidSize::Small => {
                        spawn_asteroid(AsteroidSize::Big, get_rand_pt(), &mut commands, &mut meshes, &mut materials);
                        None
                    }
                };
                if let Some(asteroid_new) = asteroid_new {
                    for _ in 0..2 {
                        spawn_asteroid(asteroid_new, asteroid_pos.0.clone(), &mut commands, &mut meshes, &mut materials);
                    }
                }
            }
        }
    }
}

fn spawn_asteroid(spawn_size: AsteroidSize, spawn_pos: Vec2, commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Circle::default()).into(),
        transform: Transform::default().with_translation(Vec3::new(0., 0., 1.)),
        material: materials.add(Color::rgba(0.8, 0.8, 0.8, 1.0)),
        ..default()
    })
        .insert(Asteroids { size: spawn_size })
        .insert(Position(spawn_pos))
        .insert(Velocity(get_rand_pt().normalize() * ASTEROID_VELOCITY));
}

fn asteroid_collide_ship(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    starship_query: Query<(Entity, &Transform, &Position), With<Starship>>,
    asteroids_query: Query<(&Transform, &Position), With<Asteroids>>,
) {
    for (starship_entity, starship_transform, starship_position) in &starship_query {
        for (asteroid_transform, asteroid_position) in &asteroids_query {
            let starship_size = starship_transform.scale.max_element();
            let asteroid_size = asteroid_transform.scale.max_element();
            let distance = (starship_position.0 - asteroid_position.0).length();

            if distance < starship_size / 5.0 + asteroid_size / 2.0 {
                commands.entity(starship_entity).despawn();
                death_messages(&mut commands, &asset_server);
            }
        }
    }
}

fn death_messages(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("fonts/MesloLGS-NF-Regular.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };
    commands.spawn((
        Text2dBundle {
            text: Text::from_section("GameOver!", text_style.clone()).with_justify(JustifyText::Center),
            ..default()
        },
        AnimateRotation,
    ));
}

fn animate_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateRotation>)>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_rotation_z((time.elapsed_seconds() * 5.).cos());
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(create_ship_mesh()).into(),
        transform: Transform::default().with_scale(Vec3::splat(50.)),
        material: materials.add(Color::rgba(0.0, 1.0, 0.0, 1.0)),
        ..default()
    }).insert(Starship { rotation: 0. }).insert(Position(Vec2::splat(0.))).insert(Velocity(Vec2::splat(0.)));

    for _ in 0..5 {
        spawn_asteroid(AsteroidSize::Big, get_rand_pt(), &mut commands, &mut meshes, &mut materials);
    }
}
