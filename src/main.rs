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
pub struct Speed(pub f32);

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
) {
    commands.spawn((
        Ant,
        Hunger::new(20.),
        Velocity::default(),
        Mesh2d(ant_mesh.0.clone()),
        MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))),
        Speed(speed),

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
            zoom_system,
            hunger_system,
            kill_system,
            ant_eating,
        ))
        .add_systems(Update, (
            food_color_system,
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
            Transform::default(),
            200.,
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
                0.,
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
    ant_query: Query<(&Transform, &mut Velocity, &Speed, &Hunger), With<Ant>>,
    food_query: Query<&Transform, With<Food>>,
) {
    let mut rng = rand::rng();
    for (ant_transform, mut velocity, ant_speed, ant_hunger) in ant_query {
        velocity.0.x += rng.random_range(-10.0..10.0);
        velocity.0.y += rng.random_range(-10.0..10.0);
        if velocity.0.length() > ant_speed.0 {
            velocity.0 = velocity.0.normalize_or_zero() * ant_speed.0;
        }

        if ant_hunger.percentage() < 0.8 {
            for food_transform in food_query {
                let delta_translation = food_transform.translation - ant_transform.translation;
                if delta_translation.length() < 100. {
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
    mut ant_query: Query<(&Transform, &mut Hunger, &Speed), With<Ant>>,
    mut food_query: Query<(&Transform, &mut Food)>,
) {
    let mut rng = rand::rng();

    for (ant_transform, mut ant_hunger, speed) in &mut ant_query {
        if ant_hunger.percentage() >= 0.8 {continue;}

        for (food_transform, mut food_value) in &mut food_query {
            let delta_translation = food_transform.translation - ant_transform.translation;
            if delta_translation.length() < 20. {
                let needed = ant_hunger.needed();
                let taking = food_value.current.min(needed);
                food_value.current -= taking;
                ant_hunger.current += taking;

                for _ in 0..2 {
                    spawn_ant(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &ant_mesh,
                        ant_transform.clone(),
                        speed.0 + rng.random_range(-10.0..10.0),
                    );
                }
            }
        }
    }
}

// fn velocity system
fn velocity_system(
    query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut transform, velocity) in query {
        transform.translation.x += velocity.0.x * dt;
        transform.translation.y += velocity.0.y * dt;
    }
}

// fn hunger system
fn hunger_system(
    query: Query<(&mut Hunger)>,
    time: Res<Time>,
) {
    for mut hunger in query {
        hunger.current -= time.delta_secs();
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

