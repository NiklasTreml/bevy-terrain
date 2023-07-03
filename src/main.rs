use std::f64::consts::PI;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::{vec2, DVec2, DVec3},
    pbr::wireframe::Wireframe,
    prelude::{shape::Cube, *},
    render::mesh,
    render::render_resource::PrimitiveTopology,
    transform::commands,
    window::{PresentMode, WindowPlugin},
};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};

use bevy_editor_pls::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin::default())
        .add_plugin(InfiniteGridPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let mesh = make_mesh();
    commands.spawn(InfiniteGridBundle {
        grid: InfiniteGrid {
            // shadow_color: None,
            ..Default::default()
        },
        ..Default::default()
    });

    let mesh = make_plane(25, 25);

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::hsl(0.0, 0.0, 0.2).into()),
            transform: Transform::from_xyz(0., 0.0, -5.0),
            ..default()
        },
        Wireframe,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    let mut camera_transform = Transform::from_xyz(0.0, 2.0, -2.0);
    camera_transform.rotate_x(-0.5);
    commands.spawn(Camera3dBundle {
        transform: camera_transform,
        ..default()
    });
}

fn make_mesh() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0., 0., 0.], [1., 5., 1.], [2., 0., 0.]],
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);
    mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1])));

    mesh
}

// Takes in a 2d array of Vec3 as points and generates a planes from the points
fn make_plane(depth: u32, width: u32) -> Mesh {
    let vertices_count: usize = ((width + 1) * (depth + 1)) as usize;
    let triangle_count: usize = (width * depth * 2 * 3) as usize;

    // Defining vertices.
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertices_count);
    let mut triangles: Vec<u32> = Vec::with_capacity(triangle_count);
    let (width_f32, depth_f32) = (width as f32, depth as f32);

    for d in 0..=width {
        for w in 0..=depth {
            let (w_f32, d_f32) = (w as f32, d as f32);

            let x = (w_f32 - width_f32 / 2.) / width_f32;
            let z = (d_f32 - depth_f32 / 2.) / depth_f32;
            let pos = [x, calculate_height(x, z), z];
            positions.push(pos);
            normals.push([0.0, 1.0, 0.0]);
            uvs.push([w_f32 / width_f32, d_f32 / depth_f32]);
        }
    }
    for d in 0..depth {
        for w in 0..width {
            // First tringle
            triangles.push((d * (width + 1)) + w);
            triangles.push(((d + 1) * (width + 1)) + w);
            triangles.push(((d + 1) * (width + 1)) + w + 1);
            // Second triangle
            triangles.push((d * (width + 1)) + w);
            triangles.push(((d + 1) * (width + 1)) + w + 1);
            triangles.push((d * (width + 1)) + w + 1);
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(mesh::Indices::U32(triangles)));

    mesh
}

fn calculate_height(x: f32, z: f32) -> f32 {
    x * z * 3.
}

//
