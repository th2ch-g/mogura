//! Demonstrates rigid as well as loose constellation clamp on the ground plane.
//!
//!  1. There is a main viewport (maximap) and a minimap at the right bottom.
//!  2. Both have their own trackball controller.
//!  3. The minimap is additionally sensitive to the controller of the maximap.
//!  5. When orbiting the maximap down by pressing `k`, it will stop as soon as the initially lower
//!     positioned minimap camera hits the ground plane.
//!  6. Toggling from rigid to loose constellation clamp by pressing `q` while keeping `k` pressed
//!     allows the maximap camera to orbit further until it hits the ground by itself.
//!  7. Press `Return` to reset camera positions and try again.

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use std::f32::consts::PI;

#[cfg(not(target_arch = "wasm32"))]
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::{
	color::palettes::basic::SILVER,
	prelude::*,
	render::{
		camera::Viewport,
		render_asset::RenderAssetUsages,
		render_resource::{Extent3d, TextureDimension, TextureFormat},
	},
	window::WindowResized,
};
use bevy_trackball::prelude::*;

fn main() {
	App::new()
		.add_plugins((
			DefaultPlugins
				.set(ImagePlugin::default_nearest())
				.set(WindowPlugin {
					primary_window: Some(Window {
						canvas: Some("#bevy".to_owned()),
						..default()
					}),
					..default()
				}),
			#[cfg(not(target_arch = "wasm32"))]
			WireframePlugin,
		))
		.add_plugins(TrackballPlugin)
		.add_systems(Startup, setup)
		.add_systems(
			Update,
			(
				rotate,
				#[cfg(not(target_arch = "wasm32"))]
				toggle_wireframe,
			),
		)
		.add_systems(Update, (resize_minimap, toggle_rigid_loose))
		.run();
}

/// A marker component for our shapes so we can query them separately from the ground plane
#[derive(Component)]
struct Shape;

const SHAPES_X_EXTENT: f32 = 14.0;
const EXTRUSION_X_EXTENT: f32 = 16.0;
const Z_EXTENT: f32 = 5.0;

fn setup(
	windows: Query<&Window>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut images: ResMut<Assets<Image>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let debug_material = materials.add(StandardMaterial {
		base_color_texture: Some(images.add(uv_debug_texture())),
		..default()
	});

	let shapes = [
		meshes.add(Cuboid::default()),
		meshes.add(Tetrahedron::default()),
		meshes.add(Capsule3d::default()),
		meshes.add(Torus::default()),
		meshes.add(Cylinder::default()),
		meshes.add(Cone::default()),
		meshes.add(ConicalFrustum::default()),
		meshes.add(Sphere::default().mesh().ico(5).unwrap()),
		meshes.add(Sphere::default().mesh().uv(32, 18)),
	];

	let extrusions = [
		meshes.add(Extrusion::new(Rectangle::default(), 1.)),
		meshes.add(Extrusion::new(Capsule2d::default(), 1.)),
		meshes.add(Extrusion::new(Annulus::default(), 1.)),
		meshes.add(Extrusion::new(Circle::default(), 1.)),
		meshes.add(Extrusion::new(Ellipse::default(), 1.)),
		meshes.add(Extrusion::new(RegularPolygon::default(), 1.)),
		meshes.add(Extrusion::new(Triangle2d::default(), 1.)),
	];

	let num_shapes = shapes.len();

	for (i, shape) in shapes.into_iter().enumerate() {
		commands.spawn((
			Mesh3d(shape),
			MeshMaterial3d(debug_material.clone()),
			Transform::from_xyz(
				-SHAPES_X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * SHAPES_X_EXTENT,
				2.0,
				Z_EXTENT / 2.0,
			)
			.with_rotation(Quat::from_rotation_x(-PI / 4.)),
			Shape,
		));
	}

	let num_extrusions = extrusions.len();

	for (i, shape) in extrusions.into_iter().enumerate() {
		commands.spawn((
			Mesh3d(shape),
			MeshMaterial3d(debug_material.clone()),
			Transform::from_xyz(
				-EXTRUSION_X_EXTENT / 2.
					+ i as f32 / (num_extrusions - 1) as f32 * EXTRUSION_X_EXTENT,
				2.0,
				-Z_EXTENT / 2.,
			)
			.with_rotation(Quat::from_rotation_x(-PI / 4.)),
			Shape,
		));
	}

	// light
	commands.spawn((
		PointLight {
			shadows_enabled: true,
			intensity: 10_000_000.,
			range: 100.0,
			shadow_depth_bias: 0.2,
			..default()
		},
		Transform::from_xyz(8.0, 16.0, 8.0),
	));

	// ground plane
	commands.spawn((
		Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
		MeshMaterial3d(materials.add(Color::from(SILVER))),
	));

	// cameras and boundary conditions
	let mut bound = Bound::default();
	bound.min_target[1] = 0.0;
	bound.min_eye[1] = 0.3;
	bound.min_distance = 6.0;
	bound.max_distance = 50.0;
	let [target, eye, up] = [Vec3::Y, Vec3::new(0.0, 7.0, 14.0), Vec3::Y];
	let maximap = commands
		.spawn((
			TrackballController::default(),
			TrackballCamera::look_at(target, eye, up).with_clamp(bound.clone()),
			Camera3d::default(),
		))
		.id();
	let window = windows.single();
	let width = window.resolution.physical_width() / 3;
	let height = window.resolution.physical_height() / 3;
	let down = Quat::from_rotation_x(15f32.to_radians());
	commands.spawn((
		TrackballController::default(),
		TrackballCamera::look_at(target, down * (eye - target) + target, up)
			.with_clamp(bound)
			.add_controller(maximap, true),
		Camera {
			order: 1,
			clear_color: ClearColorConfig::None,
			viewport: Some(Viewport {
				physical_position: UVec2::new(
					window.resolution.physical_width() - width,
					window.resolution.physical_height() - height,
				),
				physical_size: UVec2::new(width, height),
				..default()
			}),
			..default()
		},
		Camera3d::default(),
		MinimapCamera,
	));

	// UI
	#[cfg(not(target_arch = "wasm32"))]
	commands.spawn((
		TargetCamera(maximap),
		Text::new("Press space to toggle wireframes"),
		Node {
			position_type: PositionType::Absolute,
			top: Val::Px(12.0),
			left: Val::Px(12.0),
			..default()
		},
	));
	commands.spawn((
		Clamp,
		TargetCamera(maximap),
		Text::new("Rigid Constellation Clamp (Toggle: Q)"),
		Node {
			position_type: PositionType::Absolute,
			bottom: Val::Px(10.0),
			left: Val::Px(10.0),
			..default()
		},
	));
}

#[derive(Component)]
struct Clamp;

#[derive(Component)]
struct MinimapCamera;

#[allow(clippy::needless_pass_by_value)]
fn resize_minimap(
	windows: Query<&Window>,
	mut resize_events: EventReader<WindowResized>,
	mut minimap: Query<&mut Camera, With<MinimapCamera>>,
) {
	for resize_event in resize_events.read() {
		let window = windows.get(resize_event.window).unwrap();
		let mut minimap = minimap.single_mut();
		let width = window.resolution.physical_width() / 3;
		let height = window.resolution.physical_height() / 3;
		minimap.viewport = Some(Viewport {
			physical_position: UVec2::new(
				window.resolution.physical_width() - width,
				window.resolution.physical_height() - height,
			),
			physical_size: UVec2::new(width, height),
			..default()
		});
	}
}

#[allow(clippy::needless_pass_by_value)]
fn toggle_rigid_loose(
	mut minimap: Query<&mut TrackballCamera, With<MinimapCamera>>,
	mut text: Query<&mut Text, With<Clamp>>,
	keycode: Res<ButtonInput<KeyCode>>,
) {
	if keycode.just_pressed(KeyCode::KeyQ) {
		let text = &mut text.single_mut().0;
		let mut minimap = minimap.single_mut();
		let rigid = minimap.group.values_mut().next().unwrap();
		if *rigid {
			*text = "Loose Constellation Clamp (Toggle: Q)".to_owned();
		} else {
			*text = "Rigid Constellation Clamp (Toggle: Q)".to_owned();
		}
		*rigid = !*rigid;
	}
}

fn rotate(mut query: Query<&mut Transform, With<Shape>>, time: Res<Time>) {
	for mut transform in &mut query {
		transform.rotate_y(time.delta_secs() / 2.);
	}
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
	const TEXTURE_SIZE: usize = 8;

	let mut palette: [u8; 32] = [
		255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
		198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
	];

	let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
	for y in 0..TEXTURE_SIZE {
		let offset = TEXTURE_SIZE * y * 4;
		texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
		palette.rotate_right(4);
	}

	Image::new_fill(
		Extent3d {
			width: TEXTURE_SIZE as u32,
			height: TEXTURE_SIZE as u32,
			depth_or_array_layers: 1,
		},
		TextureDimension::D2,
		&texture_data,
		TextureFormat::Rgba8UnormSrgb,
		RenderAssetUsages::RENDER_WORLD,
	)
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
	mut wireframe_config: ResMut<WireframeConfig>,
	keyboard: Res<ButtonInput<KeyCode>>,
) {
	if keyboard.just_pressed(KeyCode::Space) {
		wireframe_config.global = !wireframe_config.global;
	}
}
