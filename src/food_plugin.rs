
use bevy::{prelude::*};

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
    	app
    		.add_systems(Update, food_color_system)
    	;
    }
}

/// rs food mesh
#[derive(Resource)]
pub struct FoodMesh(pub Handle<Mesh>);

/// cp food
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

/// fn spawn food
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

/// sy food color system
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
