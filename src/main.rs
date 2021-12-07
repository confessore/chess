use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa {
            samples: 4
        })
        .insert_resource(WindowDescriptor {
            title: String::from("chess"),
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(generate_board.system())
        .add_system(generate_pieces.system())
        .run();
}

fn setup(
    mut commands: Commands) {
        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_matrix(
                Mat4::from_rotation_translation(
                    Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                    Vec3::new(-7.0, 20.0, 4.0))),
            ..Default::default()
        });
        commands.spawn_bundle(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });
}

fn generate_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>) {
        let mesh = meshes.add(Mesh::from(shape::Plane {
            size: 1.0 
        }));
        let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
        let black_material = materials.add(Color::rgb(0.0, 0.1, 0.1).into());
        for i in 0..8 {
            for j in 0..8 {
                commands.spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    material: {
                        if (i + j + 1) % 2 == 0 {
                            white_material.clone()
                        } else {
                            black_material.clone()
                        }
                    },
                    transform: Transform::from_translation(
                        Vec3::new(i as f32, 0.0, j as f32)),
                    ..Default::default()
                });
            }
        }
}

fn generate_pieces(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>) {
        //let alien = asset_server.load("models/alien.glb#Mesh0/Primitive0");
        let monkey = asset_server.load("models/monkey.gltf#Mesh0/Primitive0");
        let white_material = materials.add(Color::rgb(1.0, 0.9, 0.9).into());
        let black_material = materials.add(Color::rgb(0.0, 0.1, 0.1).into());
        commands.spawn_bundle(PbrBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 4.0)),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(PbrBundle {
                mesh: monkey,
                material: white_material.clone(),
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(-0.2, 0.0, -1.9));
                    transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                    transform
                },
                ..Default::default()
            });
        });
}
