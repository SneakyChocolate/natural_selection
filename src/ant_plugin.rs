use bevy::{prelude::*};
use rand::RngExt;

use crate::Evolution;
use crate::{Hunger, Speed, Velocity, Vision, spectating_plugin::DeltaTime};
use crate::food_plugin::Food;

pub struct AntPlugin;
impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
    	app
    		.add_systems(Update, ant_movement)
    		.add_systems(Update, ant_eating)
    		.add_systems(Update, ant_color_system)
    	;
    }
}

/// rs ant mesh
#[derive(Resource)]
pub struct AntMesh(pub Handle<Mesh>);

/// cp ant
#[derive(Component)]
pub struct Ant;

/// cp ant queen
#[derive(Component)]
pub struct AntQueen;

/// fn spawn ant
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
        Evolution(0),
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

/// sy ant movement
fn ant_movement(
    ant_query: Query<(&Transform, &mut Velocity, &Speed, &Hunger, &Vision), With<Ant>>,
    food_query: Query<&Transform, With<Food>>,
    dt: Res<DeltaTime>,
) {
    let mut rng = rand::rng();
    for (ant_transform, mut velocity, ant_speed, ant_hunger, ant_vision) in ant_query {
        velocity.0.x += rng.random_range(-1000.0..1000.0) * dt.0;
        velocity.0.y += rng.random_range(-1000.0..1000.0) * dt.0;
        if velocity.0.length_squared() > ant_speed.0.powi(2) {
            velocity.0 = velocity.0.normalize_or_zero() * ant_speed.0;
        }

        if ant_hunger.percentage() < 0.8 {
            for food_transform in food_query {
                let delta_translation = food_transform.translation - ant_transform.translation;
                if delta_translation.length_squared() < ant_vision.0.powi(2) {
                    let new_velocity = delta_translation.normalize_or_zero() * ant_speed.0;
                    velocity.0.x = new_velocity.x;
                    velocity.0.y = new_velocity.y;
                }
            }
        }
    }
}

/// sy ant eating
fn ant_eating(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    ant_mesh: Res<AntMesh>,
    mut ant_query: Query<(&Transform, &mut Hunger, &Speed, &Vision, &mut Evolution), With<Ant>>,
    mut food_query: Query<(&Transform, &mut Food)>,
) {
    let mut rng = rand::rng();

    for (ant_transform, mut ant_hunger, speed, vision, mut evolultion) in &mut ant_query {
        if ant_hunger.percentage() >= 0.8 {continue;}

        for (food_transform, mut food_value) in &mut food_query {
            let delta_translation = food_transform.translation - ant_transform.translation;
            if delta_translation.length_squared() < 20_f32.powi(2) {
                let needed = ant_hunger.needed();
                let taking = food_value.current.min(needed);
                food_value.current -= taking;
                ant_hunger.current += taking;
                evolultion.0 += 1;

                if evolultion.0 == 5 {
                    // build nest
                    
                }

                for _ in 0..1 {
                    spawn_ant(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        &ant_mesh,
                        ant_transform.clone(),
                        (speed.0 + rng.random_range(-100.0..100.0)).max(10.),
                        (vision.0 + rng.random_range(-100.0..100.0)).max(10.),
                    );
                }
            }
        }
    }
}

/// sy ant color system
fn ant_color_system(
    query: Query<(&MeshMaterial2d<ColorMaterial>, &Hunger), With<Ant>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (material_handle, hunger) in query {
        if let Some (material) = materials.get_mut(material_handle) {
            material.color = Color::srgb(0., hunger.percentage(), 0.);
        }
    }
}

