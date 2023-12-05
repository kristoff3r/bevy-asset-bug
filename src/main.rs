use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

pub struct SomePlugin;

#[derive(Resource)]
pub struct SomeResource {
    material: Handle<CustomMaterial>,
}

impl FromWorld for SomeResource {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.resource::<AssetServer>();
        let mut materials = world.resource_mut::<Assets<CustomMaterial>>();
        let texture = asset_server.load("branding/icon.png");

        SomeResource {
            material: materials.add(CustomMaterial {
                color: Color::RED,
                color_texture: Some(texture),
            }),
        }
    }
}

impl Plugin for SomePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CustomMaterial>::default())
            .init_resource::<SomeResource>()
            // ^ This doesn't work
            .add_systems(Startup, setup);
    }

    fn finish(&self, app: &mut App) {
        // app.init_resource::<SomeResource>();
        // ^ This works
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SomePlugin))
        // .init_resource::<SomeResource>()
        // ^ This also doesn't work
        .run();
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    res: Res<SomeResource>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        // Neither of these two work when init_resource is called during plugin construction
        // material: res.material.clone_weak(),
        material: materials.add(CustomMaterial {
            color: Color::RED,
            color_texture: Some(asset_server.load("branding/icon.png")),
        }),
        ..default()
    });
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material_2d.wgsl".into()
    }
}
