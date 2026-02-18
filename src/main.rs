use std::sync::Arc;

/*

// fn == function
// cp == bevy component
// rs == bevy resource

*/
use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};
use rand::RngExt;

// rs ant mesh
#[derive(Resource)]
pub struct AntMesh(pub Handle<Mesh>);

// rs ant mesh
#[derive(Resource)]
pub struct FoodChunks(pub [Vec<Entity>; 100 * 100]);

// rs zoom
#[derive(Resource)]
pub struct Zoom(pub f32);

// rs delta time
#[derive(Resource)]
pub struct DeltaTime(pub f32);

// rs time multiplier
#[derive(Resource)]
pub struct TimeMultiplier(pub f32);

// rs food mesh
#[derive(Resource)]
pub struct FoodMesh(pub Handle<Mesh>);

// cp speed
#[derive(Component)]
pub struct Speed(pub f32);

// cp vision
#[derive(Component)]
pub struct Vision(pub f32);

// cp ant
#[derive(Component)]
pub struct Ant;

// cp hunger
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

// cp food
#[derive(Component)]
pub struct Food {
    pub current: f32,
    pub max: f32,
}

impl Food {
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

// cp velocity
#[derive(Component, Default, Copy, Clone)]
pub struct Velocity(pub Vec2);

// fn spawn ant
pub fn spawn_ant(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    ant_mesh: &Res<AntMesh>,
    transform: Transform,
    speed: f32,
    vision: f32,
) {
    commands.spawn((
        Ant,
        Hunger::new(20.),
        Velocity::default(),
        Mesh2d(ant_mesh.0.clone()),
        MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))),
        Speed(speed),
        Vision(vision),

        children![
            (
                Mesh2d(meshes.add(Circle::new(vision).to_ring(2.))),
                MeshMaterial2d(materials.add(Color::srgba(1., 1., 1., 0.1))),
            ),
            (
                Mesh2d(meshes.add(Circle::new(speed).to_ring(5.))),
                MeshMaterial2d(materials.add(Color::srgba(0., 0., 1., 0.1))),
            ),
        ],
    
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
        Food::new(100.),
        Mesh2d(food_mesh.0.clone()),
        MeshMaterial2d(materials.add(Color::srgba(1., 1., 0., 0.5))),
        transform,
    ));
}

// fn main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (
            setup,
            spawn_entities
        ).chain())
        .add_systems(Update, (
            velocity_system,
            ant_movement,
            zoom_input_system,
            zoom_apply_system,
            hunger_system,
            kill_system,
            ant_eating,
        ))
        .add_systems(Update, (
            food_color_system,
            despawn_food_system,
            delta_time_system,
            time_multiplier_system,
            camera_movement_system,
        ))
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
    commands.insert_resource(DeltaTime(0.));
    commands.insert_resource(TimeMultiplier(1.));
    commands.insert_resource(Zoom(1.));
    commands.insert_resource(FoodChunks(std::array::from_fn(|_| Vec::with_capacity(300))));
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

    commands.spawn((
        Camera2d,
        Transform::default(),
    ));

    // spawn ants
    for _ in 0..10 {
        spawn_ant(
            &mut commands,
            &mut meshes,
            &mut materials,
            &ant_mesh,
            Transform::default(),
            200.,
            100.,
        );
    }

    // spawn food
    for _ in 0..2000 {
        spawn_food(
            &mut commands,
            &mut meshes,
            &mut materials,
            &food_mesh,
            Transform::from_xyz(
                rng.random_range(-10000.0..10000.0),
                rng.random_range(-10000.0..10000.0),
                0.,
            )
        );
    }
}

// fn zoom input system
fn zoom_input_system(
    mouse_wheel: Res<AccumulatedMouseScroll>,
    mut zoom: ResMut<Zoom>,
    key_input: Res<ButtonInput<KeyCode>>,
){
    if mouse_wheel.delta.y < 0. || key_input.pressed(KeyCode::KeyJ) {
        zoom.0 *= 1.01;
    }
    else if mouse_wheel.delta.y > 0. || key_input.pressed(KeyCode::KeyK) {
        zoom.0 *= 0.99;
    }
}

// fn zoom apply system
fn zoom_apply_system(
    camera: Single<&mut Projection, With<Camera>>,
    zoom: Res<Zoom>,
){
    if let Projection::Orthographic(orthographic) = &mut *camera.into_inner() {
        orthographic.scale = zoom.0;
    }
}

// fn ant movement
fn ant_movement(
    ant_query: Query<(&Transform, &mut Velocity, &Speed, &Hunger, &Vision), With<Ant>>,
    food_query: Query<&Transform, With<Food>>,
    dt: Res<DeltaTime>,
) {
    let mut rng = rand::rng();
    for (ant_transform, mut velocity, ant_speed, ant_hunger, ant_vision) in ant_query {
        velocity.0.x += rng.random_range(-1000.0..1000.0) * dt.0;
        velocity.0.y += rng.random_range(-1000.0..1000.0) * dt.0;
        if velocity.0.length() > ant_speed.0 {
            velocity.0 = velocity.0.normalize_or_zero() * ant_speed.0;
        }

        if ant_hunger.percentage() < 0.8 {
            for food_transform in food_query {
                let delta_translation = food_transform.translation - ant_transform.translation;
                if delta_translation.length() < ant_vision.0 {
                    let new_velocity = delta_translation.normalize_or_zero() * ant_speed.0;
                    velocity.0.x = new_velocity.x;
                    velocity.0.y = new_velocity.y;
                }
            }
        }
    }
}

// fn ant eating
fn ant_eating(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ant_mesh: Res<AntMesh>,
    mut ant_query: Query<(&Transform, &mut Hunger, &Speed, &Vision), With<Ant>>,
    mut food_query: Query<(&Transform, &mut Food)>,
) {
    let mut rng = rand::rng();

    for (ant_transform, mut ant_hunger, speed, vision) in &mut ant_query {
        if ant_hunger.percentage() >= 0.8 {continue;}

        for (food_transform, mut food_value) in &mut food_query {
            let delta_translation = food_transform.translation - ant_transform.translation;
            if delta_translation.length() < 20. {
                let needed = ant_hunger.needed();
                let taking = food_value.current.min(needed);
                food_value.current -= taking;
                ant_hunger.current += taking;

                for _ in 0..4 {
                    spawn_ant(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &ant_mesh,
                        ant_transform.clone(),
                        (speed.0 + rng.random_range(-50.0..50.0)).max(10.),
                        (vision.0 + rng.random_range(-50.0..50.0)).max(10.),
                    );
                }
            }
        }
    }
}

// fn velocity system
fn velocity_system(
    query: Query<(&mut Transform, &Velocity)>,
    dt: Res<DeltaTime>,
) {
    for (mut transform, velocity) in query {
        transform.translation.x += velocity.0.x * dt.0;
        transform.translation.y += velocity.0.y * dt.0;
    }
}

// fn hunger system
fn hunger_system(
    query: Query<(&mut Hunger)>,
    dt: Res<DeltaTime>,
) {
    for mut hunger in query {
        hunger.current -= dt.0;
    }
}

// fn kill system
fn kill_system(
    mut commands: Commands,
    query: Query<(Entity, &Hunger)>,
) {
    for (entity, hunger) in query {
        if hunger.current <= 0. {
            commands.entity(entity).despawn();
        }
    }
}

// fn kill system
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

// fn food color system
fn food_color_system(
    query: Query<(&MeshMaterial2d<ColorMaterial>, &Food)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (material_handle, food) in query {
        if let Some (material) = materials.get_mut(material_handle) {
            material.color.set_alpha(food.percentage());
        }
    }
}

// fn delta time system
fn delta_time_system(
    time: Res<Time>,
    time_multiplier: Res<TimeMultiplier>,
    mut dt: ResMut<DeltaTime>,
) {
    dt.0 = time.delta_secs() * time_multiplier.0;
}

// fn time multiplier system
fn time_multiplier_system(
    key_input: Res<ButtonInput<KeyCode>>,
    mut time_multiplier: ResMut<TimeMultiplier>,
) {
    if key_input.pressed(KeyCode::KeyL) {
        time_multiplier.0 *= 1.01;
    }
    if key_input.pressed(KeyCode::KeyH) {
        time_multiplier.0 *= 0.99;
    }
    // info!("multiplier is {}", time_multiplier.0);
}

// fn camera movement system
fn camera_movement_system(
    key_input: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<Camera2d>>,
    time: Res<Time>,
    zoom: Res<Zoom>,
) {
    let dt = time.delta_secs();
    if key_input.pressed(KeyCode::KeyW) {
        camera.translation.y += 1000. * dt * zoom.0;
    }
    if key_input.pressed(KeyCode::Space) {
        camera.translation.y -= 1000. * dt * zoom.0;
    }
    if key_input.pressed(KeyCode::KeyA) {
        camera.translation.x -= 1000. * dt * zoom.0;
    }
    if key_input.pressed(KeyCode::KeyD) {
        camera.translation.x += 1000. * dt * zoom.0;
    }
}

