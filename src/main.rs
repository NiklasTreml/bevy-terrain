use std::f64::consts::PI;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    math::{vec2, DVec2, DVec3},
    pbr::{wireframe::Wireframe, CascadeShadowConfigBuilder},
    prelude::{shape::Cube, *},
    render::mesh,
    render::render_resource::PrimitiveTopology,
    transform::commands,
    window::{PresentMode, WindowPlugin},
};
use bevy_infinite_grid::{InfiniteGrid, InfiniteGridBundle, InfiniteGridPlugin};

use bevy_editor_pls::prelude::*;
use noise::{NoiseFn, Perlin, Simplex};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin::default())
        // .add_plugin(InfiniteGridPlugin)
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

    let resolution = 5.;
    let mesh = make_plane(250, 250, resolution);

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            // material: materials.add(Color::hsl(0.0, 0.0, 0.2).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GRAY,
                double_sided: true,
                perceptual_roughness: 1.0,
                metallic: 0.3,
                ..default()
            }),
            transform: Transform::from_xyz(0., 0.0, 0.0),
            ..default()
        },
        // Wireframe,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight { ..default() },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight { ..default() },
        transform: Transform::from_xyz(15.0, 8.0, 5.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight { ..default() },
        transform: Transform::from_xyz(-20.0, 8.0, 3.0),
        ..default()
    });

    let mut camera_transform = Transform::from_xyz(0.0, 8.0, 40.0);
    camera_transform.rotate_x(-0.2);
    commands.spawn(Camera3dBundle {
        transform: camera_transform,
        ..default()
    });
}

// Takes in a 2d array of Vec3 as points and generates a planes from the points
fn make_plane(depth: u32, width: u32, resolution: f32) -> Mesh {
    let vertices_count: usize = ((width + 1) * (depth + 1)) as usize;
    let triangle_count: usize = (width * depth * 2 * 3) as usize;

    // Defining vertices.
    let mut positions: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
    let mut normals: Vec<[f32; 3]> = Vec::with_capacity(vertices_count);
    let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(vertices_count);
    let mut triangles: Vec<u32> = Vec::with_capacity(triangle_count);
    let (width_f32, depth_f32) = (width as f32, depth as f32);

    let simplex = Simplex::new(48192874);
    let perlin = Perlin::new(0891273);

    for d in 0..=width {
        for w in 0..=depth {
            let (w_f32, d_f32) = (w as f32, d as f32);

            let x = (w_f32 - width_f32 / 2.) / resolution;
            let z = (d_f32 - depth_f32 / 2.) / resolution;

            let p_coords = [x * 2.5, z * 7.5]; // this adds small deformities to make the terrain
                                               // look more realistic, essentially in an effort to make it look less perfect
            let y1 = simplex.get([p_coords[0] as f64, p_coords[1] as f64]) as f32;

            // this is added in for medium terrain changes, small hills and valleys etc
            let p_coords = [x * 0.22, z * 0.22];
            let y2 = perlin.get([p_coords[0] as f64, p_coords[1] as f64]) as f32;

            // this generates big hills and valleys, but at a low frequency
            let p_coords = [x * 0.05, z * 0.05];
            let y3 = perlin.get([p_coords[0] as f64, p_coords[1] as f64]) as f32;

            let y = y1 * 0.3 + y2 * 0.5 + y3 * 2.;

            let pos = [x, y, z];
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

//
