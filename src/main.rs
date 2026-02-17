/*

// fn == function
// cp == bevy component
// rs == bevy resource

*/
use bevy::prelude::*;

// rs ant mesh
#[derive(Resource)]
pub struct AntMesh(pub Handle<Mesh>);

// cp ant
#[derive(Component)]
pub struct Ant;

// fn main
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, ant_movement)
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

// fn ant movement
fn ant_movement(
    ant_query: Query<&mut Transform, With<Ant>>,
) {
    for mut transform in ant_query {
        transform.translation.x += 1.;
    }
}

