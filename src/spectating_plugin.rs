use bevy::{input::mouse::AccumulatedMouseScroll, prelude::*};

/// rs zoom
#[derive(Resource)]
pub struct Zoom(pub f32);

/// rs delta time
#[derive(Resource)]
pub struct DeltaTime(pub f32);

/// rs time multiplier
#[derive(Resource)]
pub struct TimeMultiplier(pub f32);

pub struct SpectatingPlugin;
impl Plugin for SpectatingPlugin {
    fn build(&self, app: &mut App) {
    	app
    		.add_systems(Update, zoom_input_system)
    		.add_systems(Update, zoom_apply_system)
    		.add_systems(Update, delta_time_system)
    		.add_systems(Update, time_multiplier_system)
    		.add_systems(Update, camera_movement_system)
    	;
    }
}

/// sy zoom input system
fn zoom_input_system(
    mouse_wheel: Res<AccumulatedMouseScroll>,
    mut zoom: ResMut<Zoom>,
    key_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
){
    let dt = time.delta_secs();
    if mouse_wheel.delta.y < 0. || key_input.pressed(KeyCode::KeyJ) {
        zoom.0 *= 1. + (2. * dt);
    }
    else if mouse_wheel.delta.y > 0. || key_input.pressed(KeyCode::KeyK) {
        zoom.0 /= 1. + (2. * dt);
    }
}

/// sy zoom apply system
fn zoom_apply_system(
    camera: Single<&mut Projection, With<Camera>>,
    zoom: Res<Zoom>,
){
    if let Projection::Orthographic(orthographic) = &mut *camera.into_inner() {
        orthographic.scale = zoom.0;
    }
}

/// sy delta time system
fn delta_time_system(
    time: Res<Time>,
    time_multiplier: Res<TimeMultiplier>,
    mut dt: ResMut<DeltaTime>,
) {
    dt.0 = time.delta_secs() * time_multiplier.0;
}

/// sy time multiplier system
fn time_multiplier_system(
    key_input: Res<ButtonInput<KeyCode>>,
    mut time_multiplier: ResMut<TimeMultiplier>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    if key_input.pressed(KeyCode::KeyL) {
        time_multiplier.0 *= 1. + (2. * dt);
    }
    if key_input.pressed(KeyCode::KeyH) {
        time_multiplier.0 /= 1. + (2. * dt);
    }
    // info!("multiplier is {}", time_multiplier.0);
}

/// sy camera movement system
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

