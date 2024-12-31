//! Renders two cameras to the same window to accomplish *split screen*. The left camera
//! uses perspective projection whereas the right camera uses orthographic projection. Spawns three
//! windows each with different scaling modes regarding window resize events:
//!
//!  1. window with fixed vertical field of view.
//!  2. window with fixed horizontal field of view.
//!  3. window with fixed unit per pixels.

#![allow(clippy::similar_names)]

use bevy::{
	prelude::*,
	render::camera::{RenderTarget, Viewport},
	window::{PrimaryWindow, WindowRef, WindowResized},
};
use bevy_trackball::prelude::{Fixed, *};

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(TrackballPlugin)
		.add_systems(Startup, setup)
		.add_systems(Update, set_camera_viewports)
		.run();
}

/// set up a simple 3D scene
fn setup(
	mut windows: Query<&mut Window>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	// circular base
	commands.spawn((
		Mesh3d(meshes.add(Circle::new(4.0))),
		MeshMaterial3d(materials.add(Color::WHITE)),
		Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
	));
	// cube
	commands.spawn((
		Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
		MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
		Transform::from_xyz(0.0, 0.5, 0.0),
	));
	// light
	commands.spawn((
		PointLight {
			shadows_enabled: true,
			..default()
		},
		Transform::from_xyz(4.0, 8.0, 4.0),
	));

	// Windows
	let mut window1 = windows.single_mut();
	"Fixed Vertical Field of View (Perspective vs Orthographic)".clone_into(&mut window1.title);
	let res = &window1.resolution;
	let max = Vec2::new(res.width() * 0.5, res.height()).into();
	// Left and right camera orientation.
	let [target, eye, up] = [Vec3::Y * 0.5, Vec3::new(-2.5, 4.5, 9.0) * 1.2, Vec3::Y];
	// Spawn a 2nd window.
	let window2 = commands
		.spawn(Window {
			title: "Fixed Horizontal Field of View (Perspective vs Orthographic)".to_owned(),
			..default()
		})
		.id();
	// Spawn a 3rd window.
	let window3 = commands
		.spawn(Window {
			title: "Fixed Unit Per Pixels (Perspective vs Orthographic)".to_owned(),
			..default()
		})
		.id();

	// Cameras
	let mut order = 0;
	let fov = Fixed::default();
	for (fov, window) in [
		(fov, WindowRef::Primary),
		(fov.to_hor(&max), WindowRef::Entity(window2)),
		(fov.to_upp(&max), WindowRef::Entity(window3)),
	] {
		let mut scope = Scope::default();
		scope.set_fov(fov);
		// Left trackball controller and camera 3D bundle.
		let left = commands
			.spawn((
				TrackballController::default(),
				Camera {
					target: RenderTarget::Window(window),
					// Renders the right camera after the left camera,
					// which has a default priority of 0.
					order,
					..default()
				},
				Camera3d::default(),
				LeftCamera,
			))
			.id();
		order += 1;
		// Right trackball controller and camera 3D bundle.
		let right = commands
			.spawn((
				TrackballController::default(),
				Camera {
					target: RenderTarget::Window(window),
					// Renders the right camera after the left camera,
					// which has a default priority of 0.
					order,
					// Don't clear on the second camera
					// because the first camera already cleared the window.
					clear_color: ClearColorConfig::None,
					..default()
				},
				Camera3d::default(),
				RightCamera,
			))
			.id();
		order += 1;
		// Insert left trackball camera and make it sensitive to right trackball controller as well.
		commands.entity(left).insert(
			TrackballCamera::look_at(target, eye, up)
				.with_scope(scope)
				.add_controller(right, true),
		);
		// Set orthographic projection mode for right camera.
		scope.set_ortho(true);
		// Insert right trackball camera and make it sensitive to left trackball controller as well.
		commands.entity(right).insert(
			TrackballCamera::look_at(target, eye, up)
				.with_scope(scope)
				.add_controller(left, true),
		);
	}
}

#[derive(Component)]
struct LeftCamera;

#[derive(Component)]
struct RightCamera;

#[allow(clippy::needless_pass_by_value)]
fn set_camera_viewports(
	primary_windows: Query<(Entity, &Window), With<PrimaryWindow>>,
	windows: Query<(Entity, &Window)>,
	mut resize_events: EventReader<WindowResized>,
	mut left_cameras: Query<&mut Camera, (With<LeftCamera>, Without<RightCamera>)>,
	mut right_cameras: Query<&mut Camera, With<RightCamera>>,
) {
	// We need to dynamically resize the camera's viewports whenever the window size changes,
	// so then each camera always takes up half the screen. A `resize_event` is sent when the window
	// is first created, allowing us to reuse this system for initial setup.
	for resize_event in resize_events.read() {
		let (resize_id, resize_window) = windows.get(resize_event.window).unwrap();
		let resolution = &resize_window.resolution;
		for mut left_camera in &mut left_cameras {
			if let RenderTarget::Window(window_ref) = left_camera.target {
				let Some((target_id, _target_window)) = (match window_ref {
					WindowRef::Primary => primary_windows.get_single().ok(),
					WindowRef::Entity(id) => windows.get(id).ok(),
				}) else {
					continue;
				};
				if target_id == resize_id {
					left_camera.viewport = Some(Viewport {
						physical_position: UVec2::new(0, 0),
						physical_size: UVec2::new(
							resolution.physical_width() / 2,
							resolution.physical_height(),
						),
						..default()
					});
				}
			}
		}
		for mut right_camera in &mut right_cameras {
			if let RenderTarget::Window(window_ref) = right_camera.target {
				let Some((target_id, _target_window)) = (match window_ref {
					WindowRef::Primary => primary_windows.get_single().ok(),
					WindowRef::Entity(id) => windows.get(id).ok(),
				}) else {
					continue;
				};
				if target_id == resize_id {
					right_camera.viewport = Some(Viewport {
						physical_position: UVec2::new(resolution.physical_width() / 2, 0),
						physical_size: UVec2::new(
							resolution.physical_width() / 2,
							resolution.physical_height(),
						),
						..default()
					});
				}
			}
		}
	}
}
