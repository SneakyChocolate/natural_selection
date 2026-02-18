/*

// fn == function
// cp == bevy component
// rs == bevy resource

*/
use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};
use rand::{Rng, RngExt};

// rs ant mesh
#[derive(Resource)]
pub struct AntMesh(pub Handle<Mesh>);

// rs food mesh
#[derive(Resource)]
pub struct FoodMesh(pub Handle<Mesh>);

// cp ant
#[derive(Component)]
pub struct Ant {
    pub hunger: f32,
}

impl Default for Ant {
    fn default() -> Self {
        Self {
            hunger: 10.
        }
    }
}

// fn spawn ant
pub fn spawn_ant(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    ant_mesh: &Res<AntMesh>,
    transform: Transform,
) {
    commands.spawn((
        Ant::default(),
        Velocity::default(),
        Mesh2d(ant_mesh.0.clone()),
        MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))),

        children![(
            Mesh2d(meshes.add(Circle::new(100.0).to_ring(2.))),
            MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
            Transform::default(),
        )],
    
        transform,
    ));
}

// fn spawn food
pub fn spawn_food(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    food_mesh: &Res<FoodMesh>,
    transform: Transform,
) {
    commands.spawn((
        Food(100.),
        Mesh2d(food_mesh.0.clone()),
        MeshMaterial2d(materials.add(Color::srgb(1., 1., 0.))),
        transform,
    ));
}

// cp food
#[derive(Component)]
pub struct Food(pub f32);

// cp velocity
#[derive(Component, Default, Copy, Clone)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// fn main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_entities).chain())
        .add_systems(Update, (velocity_system, ant_movement, zoom_system))
        .run()
    ;
}

// fn setup
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(AntMesh(meshes.add(Circle::new(10.0))));
    commands.insert_resource(FoodMesh(meshes.add(Circle::new(20.0))));
}

// fn spawn entities
fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ant_mesh: Res<AntMesh>,
    food_mesh: Res<FoodMesh>,
) {
    let mut rng = rand::rng();

    commands.spawn(Camera2d);

    // spawn ants
    for _ in 0..10 {
        spawn_ant(
            &mut commands,
            &mut meshes,
            &mut materials,
            &ant_mesh,
            Transform::default()
        );
    }

    // spawn food
    for _ in 0..200 {
        spawn_food(
            &mut commands,
            &mut meshes,
            &mut materials,
            &food_mesh,
            Transform::from_xyz(
                rng.random_range(-10000.0..10000.0),
                rng.random_range(-10000.0..10000.0),
                0.
            )
        );
    }
}

// fn zoom system
fn zoom_system(mouse_wheel: Res<AccumulatedMouseScroll>, camera_query: Single<&mut Projection, With<Camera>>){
    match &mut *camera_query.into_inner() {
        Projection::Orthographic(orthographic) => {
            if mouse_wheel.delta.y < 0. {
                orthographic.scale *= 1.1;
            }
            else if mouse_wheel.delta.y > 0. {
                orthographic.scale *= 0.9;
            }
        }
        _ => (),
    }
}

// fn ant movement
fn ant_movement(
    ant_query: Query<&mut Velocity, With<Ant>>,
) {
    let mut rng = rand::rng();
    for mut velocity in ant_query {
        velocity.x += rng.random_range(-1.0..1.0);
        velocity.y += rng.random_range(-1.0..1.0);
    }
}

// fn velocity system
fn velocity_system(
    query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut transform, velocity) in query {
        transform.translation.x += velocity.x * dt;
        transform.translation.y += velocity.y * dt;
    }
}

