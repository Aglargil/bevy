//! This example demonstrates how to use the `bevy_gizmos` crate to draw lines and points in 2D.

use bevy::{
    color::palettes::css::*,
    math::{Isometry2d, Vec2},
    prelude::*,
    sprite::MaterialMesh2dBundle,
};
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn plot_line(mut gizmos: Gizmos, query: Query<&MovablePoint>) {
    let points: Vec<&MovablePoint> = query.iter().collect();
    if points.len() < 2 {
        return;
    }

    gizmos.linestrip_2d(points.iter().map(|p| p.position.translation), YELLOW);
}

fn plot_point(mut query: Query<(&mut Transform, &MovablePoint)>) {
    for (mut transform, movable_point) in &mut query {
        transform.translation = Vec3::new(
            movable_point.position.translation.x,
            movable_point.position.translation.y,
            0.0,
        );
    }
}

fn add_point_with_right_mouse(
    mut commands: Commands,
    camera: Query<(&Camera, &GlobalTransform)>,
    input: Res<ButtonInput<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_position: Res<MousePosition>,
) {
    if input.just_pressed(MouseButton::Right) {
        let Some(mouse_position) = mouse_position.0 else {
            return;
        };
        let Ok((camera, camera_transform)) = camera.get_single() else {
            return;
        };
        let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, mouse_position)
        else {
            return;
        };
        let circle = Circle::new(5.0);
        commands.spawn((
            MovablePoint {
                position: Isometry2d::from_xy(world_position.x, world_position.y),
                size: circle.radius,
                is_selected: false,
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(circle.mesh().build()).into(),
                material: materials.add(Color::WHITE),
                transform: Transform::from_xyz(world_position.x, world_position.y, 0.0),
                ..Default::default()
            },
        ));
    }
}

// Components
/// The current mouse position, if known.
#[derive(Clone, Default, Resource)]
struct MousePosition(Option<Vec2>);

/// Update the current cursor position and track it in the [`MousePosition`] resource.
fn handle_mouse_move(
    mut cursor_events: EventReader<CursorMoved>,
    mut mouse_position: ResMut<MousePosition>,
) {
    if let Some(cursor_event) = cursor_events.read().last() {
        mouse_position.0 = Some(cursor_event.position);
    }
}

#[derive(Component)]
struct MovablePoint {
    position: Isometry2d,
    size: f32,
    is_selected: bool,
}

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MousePosition::default())
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (
                handle_mouse_move,
                add_point_with_right_mouse,
                plot_line,
                plot_point,
            )
                .chain(),
        )
        .run();
}
