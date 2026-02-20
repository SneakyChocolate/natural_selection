pub mod spectating_plugin;
pub mod ant_plugin;
pub mod food_plugin;

use crate::ant_plugin::*;
use crate::food_plugin::*;
use crate::spectating_plugin::*;
use rand::RngExt;
use bevy::{prelude::*};


/*

/// fn == general function
/// sy == bevy system
/// cp == bevy component
/// rs == bevy resource
/// pl == bevy plugin

*/

/// rs position boundaries
#[derive(Resource, Default)]
pub struct PositionBoundaries{
    pub min: Vec2,
    pub max: Vec2,
}

/// cp evolution
#[derive(Component)]
pub struct Evolution(pub usize);

/// cp lifetime
#[derive(Component)]
pub struct Lifetime(pub f32);

/// cp speed
#[derive(Component)]
pub struct Speed(pub f32);

/// cp vision
#[derive(Component)]
pub struct Vision(pub f32);

/// cp hunger
#[derive(Component)]
pub struct Hunger {
    pub current: f32,
    pub max: f32,
}

impl Hunger {
    pub fn new(max: f32) -> Self {
        Self {
            current: max,
            max,
        }
    }
    pub fn percentage(&self) -> f32 {
        self.current / self.max
    }
    pub fn needed(&self) -> f32 {
        self.max - self.current
    }
}

/// cp velocity
#[derive(Component, Default, Copy, Clone)]
pub struct Velocity(pub Vec2);

/// fn spawn camera
pub fn spawn_camera(
    commands: &mut Commands,
) {
    commands.spawn((
        Camera2d,
        Transform::default(),
    ));
}

/// fn main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            setup,
            spawn_entities,
        ).chain())
        .add_systems(Update, (
            velocity_system,
            hunger_system,
            kill_system,
            lifetime_system,
        ))
        .add_systems(Update, (
            despawn_food_system,
            update_position_boundaries,
        ))
        .add_plugins((
            SpectatingPlugin,
            AntPlugin,
        ))
        .run()
    ;
}

/// sy setup
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(AntMesh(meshes.add(Circle::new(10.0))));
    commands.insert_resource(AntPheromoneMesh(meshes.add(Circle::new(5.))));
    commands.insert_resource(FoodMesh(meshes.add(Circle::new(20.0))));
    commands.insert_resource(DeltaTime(0.));
    commands.insert_resource(TimeMultiplier(1.));
    commands.insert_resource(Zoom(1.));
    commands.insert_resource(PositionBoundaries::default());
}

/// sy spawn entities
fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ant_mesh: Res<AntMesh>,
    food_mesh: Res<FoodMesh>,
) {
    let mut rng = rand::rng();

    spawn_camera(&mut commands);

    // spawn ants
    let nest_entity = spawn_ant_nest(
        &mut commands,
        &mut meshes,
        &mut materials,
        Transform::from_xyz(0., 0., -1.),
    );
    for _ in 0..200 {
        spawn_ant(
            &mut commands,
            &mut meshes,
            &mut materials,
            &ant_mesh,
            Transform::default(),
            200.,
            100.,
            nest_entity,
        );
        spawn_ant(
            &mut commands,
            &mut meshes,
            &mut materials,
            &ant_mesh,
            Transform::default(),
            (200_f32 + rng.random_range(-100.0..100.0)).max(10.),
            (100_f32 + rng.random_range(-50.0..50.0)).max(10.),
            nest_entity,
        );
    }

    // spawn food
    for _ in 0..800 {
        spawn_food(
            &mut commands,
            &mut meshes,
            &mut materials,
            &food_mesh,
            Transform::from_xyz(
                rng.random_range(-20000.0..20000.0),
                rng.random_range(-20000.0..20000.0),
                0.,
            )
        );
    }
}



/// sy velocity system
fn velocity_system(
    query: Query<(&mut Transform, &Velocity)>,
    dt: Res<DeltaTime>,
) {
    for (mut transform, velocity) in query {
        transform.translation.x += velocity.0.x * dt.0;
        transform.translation.y += velocity.0.y * dt.0;
    }
}

/// sy hunger system
fn hunger_system(
    query: Query<(&mut Hunger)>,
    dt: Res<DeltaTime>,
) {
    for mut hunger in query {
        hunger.current -= dt.0;
    }
}

/// sy lifetime system
fn lifetime_system(
    query: Query<(&mut Lifetime)>,
    dt: Res<DeltaTime>,
) {
    for mut value in query {
        value.0 -= dt.0;
    }
}

/// sy kill system
fn kill_system(
    mut commands: Commands,
    hunger_query: Query<(Entity, &Hunger)>,
    lifetime_query: Query<(Entity, &Lifetime)>,
) {
    for (entity, hunger) in hunger_query {
        if hunger.current <= 0. {
            commands.entity(entity).despawn();
        }
    }
    for (entity, lifetime) in lifetime_query {
        if lifetime.0 <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

/// sy kill system
fn despawn_food_system(
    mut commands: Commands,
    query: Query<(Entity, &Food)>,
) {
    for (entity, food) in query {
        if food.current <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

/// sy update position boundaries
fn update_position_boundaries(
    mut boundaries: ResMut<PositionBoundaries>,
    positions: Query<&Transform>,
) {
    let mut min = Vec2::default();
    let mut max = Vec2::default();
    for transform in positions {
        if transform.translation.x < min.x {
            min.x = transform.translation.x;
        }
        if transform.translation.y < min.y {
            min.y = transform.translation.y;
        }
        if transform.translation.x > max.x {
            max.x = transform.translation.x;
        }
        if transform.translation.y > max.y {
            max.y = transform.translation.y;
        }
    }
    boundaries.min = min;
    boundaries.max = max;
}
