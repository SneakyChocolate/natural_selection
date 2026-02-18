/*

// fn == function
// cp == bevy component
// rs == bevy resource

*/
use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};

// rs ant mesh
#[derive(Resource)]
pub struct AntMesh(pub Handle<Mesh>);

// cp ant
#[derive(Component)]
pub struct Ant;

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
        .add_systems(Startup, setup)
        .add_systems(Update, (velocity_system, ant_movement, zoom_camera))
        .run()
    ;
}

// fn setup
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let antmesh = meshes.add(Circle::new(10.0));
    commands.insert_resource(AntMesh(antmesh.clone()));

    commands.spawn((
        Ant,
        Velocity::default(),
        Mesh2d(antmesh.clone()),
        MeshMaterial2d(materials.add(Color::srgb(0., 1., 0.))),

        children![(
            Mesh2d(meshes.add(Circle::new(100.0).to_ring(2.))),
            MeshMaterial2d(materials.add(Color::srgb(1., 1., 1.))),
            Transform::default(),
        )],
        
        Transform::default(),
    ));
}

// fn zoom camera
fn zoom_camera(mouse_wheel: Res<AccumulatedMouseScroll>, camera_query: Single<&mut Projection, With<Camera>>){
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
    for mut velocity in ant_query {
        velocity.x += 1.;
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

