//! This example demonstrates how to use the `bevy_gizmos` crate to draw lines and points in 2D.

use bevy::{
    color::palettes::css::*,
    input::{mouse::MouseButtonInput, ButtonState},
    math::{Isometry2d, Vec2},
    prelude::*,
};
// Systems

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn plot_line(mut gizmos: Gizmos, query: Query<&MovablePoint>) {
    let points: Vec<&MovablePoint> = query.iter().collect();
    if points.len() < 2 {
        return;
    }
    for i in 0..points.len() - 1 {
        println!("i: {}", i);
        let point1 = points[i];
        let point2 = points[i + 1];
        gizmos.line_2d(
            point1.position.translation,
            point2.position.translation,
            YELLOW,
        );
    }
}

fn plot_point(mut gizmos: Gizmos, query: Query<&MovablePoint>) {
    for point in &query {
        let color = if point.is_selected {
            point.selected_color
        } else {
            point.default_color
        };
        gizmos.circle_2d(point.position, point.size, color);
    }
}

fn move_point_with_mouse(
    mut query: Query<&mut MovablePoint>,
    input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mouse_position: Res<MousePosition>,
) {
    let mut clear_selected = || {
        for mut point in &mut query {
            point.is_selected = false;
        }
    };
    let Some(mouse_position) = mouse_position.0 else {
        clear_selected();
        return;
    };
    if !input.pressed(MouseButton::Left) {
        clear_selected();
        return;
    }

    let Ok((camera, camera_transform)) = camera.get_single() else {
        return;
    };
    // Convert the starting point and end point (current mouse pos) into world coords:
    let Ok(mouse_point) = camera.viewport_to_world_2d(camera_transform, mouse_position) else {
        return;
    };
    println!("mouse_point: {:?}", mouse_point);
    for mut point in query.iter_mut() {
        if point.is_selected {
            point.position.translation = mouse_point;
            return;
        }
    }

    for mut point in &mut query {
        if point.position.translation.distance(mouse_point) < point.size {
            point.is_selected = true;
            break;
        }
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

fn add_point_with_right_mouse(
    mut commands: Commands,
    camera: Query<(&Camera, &GlobalTransform)>,
    input: Res<ButtonInput<MouseButton>>,
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
        commands.spawn(MovablePoint {
            position: Isometry2d::from_xy(world_position.x, world_position.y),
            size: circle.radius,
            default_color: GREEN,
            selected_color: RED,
            is_selected: false,
        });
    }
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct MovablePoint {
    position: Isometry2d,
    size: f32,
    default_color: Srgba,
    selected_color: Srgba,
    is_selected: bool,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(MousePosition::default())
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (
                handle_mouse_move,
                move_point_with_mouse,
                add_point_with_right_mouse,
                plot_point,
                plot_line,
            )
                .chain(),
        )
        .run();
}
