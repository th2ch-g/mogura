//! Demonstrates viewport stealing to share input with egui.

use std::f32::consts::PI;

use bevy::{
	prelude::*,
	render::{
		camera::RenderTarget,
		render_resource::{
			Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
		},
		view::RenderLayers,
	},
};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiUserTextures};
use bevy_trackball::prelude::*;
use egui::Widget;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				canvas: Some("#bevy".to_owned()),
				..default()
			}),
			..default()
		}))
		.add_plugins(TrackballPlugin)
		.add_plugins(EguiPlugin)
		.add_systems(Startup, setup)
		.add_systems(Update, render_to_image_example_system)
		.add_systems(
			Update,
			camera_attached_light.after(TrackballSystemSet::Camera),
		)
		.run();
}

// Marks the preview pass cube.
#[derive(Component)]
struct PreviewPassCube;

// Marks the main pass cube, to which the material is applied.
#[derive(Component)]
struct MainPassCube;

#[derive(Deref, Resource)]
struct CubePreviewImage(Handle<Image>);

fn setup(
	mut egui_user_textures: ResMut<EguiUserTextures>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut images: ResMut<Assets<Image>>,
) {
	let size = Extent3d {
		width: 512,
		height: 512,
		..default()
	};

	// This is the texture that will be rendered to.
	let mut image = Image {
		texture_descriptor: TextureDescriptor {
			label: None,
			size,
			dimension: TextureDimension::D2,
			format: TextureFormat::Bgra8UnormSrgb,
			mip_level_count: 1,
			sample_count: 1,
			usage: TextureUsages::TEXTURE_BINDING
				| TextureUsages::COPY_DST
				| TextureUsages::RENDER_ATTACHMENT,
			view_formats: &[],
		},
		..default()
	};

	// fill image.data with zeroes
	image.resize(size);

	let image_handle = images.add(image);
	egui_user_textures.add_image(image_handle.clone());
	commands.insert_resource(CubePreviewImage(image_handle.clone()));

	// Transform of cubes.
	let cube_transform = Transform {
		rotation: Quat::from_rotation_x(-PI / 5.0),
		..default()
	};

	let cube_handle = meshes.add(Cuboid::new(4.0, 4.0, 4.0));
	let default_material = StandardMaterial {
		base_color: Color::srgb(0.8, 0.7, 0.6),
		reflectance: 0.02,
		unlit: false,
		..default()
	};
	let preview_material_handle = materials.add(default_material.clone());

	// This specifies the layer used for the preview pass, which will be attached to the preview
	// pass camera and cube.
	let preview_pass_layer = RenderLayers::layer(1);

	// The cube that will be rendered to the texture.
	commands
		.spawn((
			Mesh3d(cube_handle),
			MeshMaterial3d(preview_material_handle),
			cube_transform,
		))
		.insert(PreviewPassCube)
		.insert(preview_pass_layer.clone());

	// Cube parameters.
	let cube_size = 4.0;
	let cube_handle = meshes.add(Cuboid::new(cube_size, cube_size, cube_size));

	// Main pass cube.
	let main_material_handle = materials.add(default_material);
	commands
		.spawn((
			Mesh3d(cube_handle),
			MeshMaterial3d(main_material_handle),
			cube_transform,
		))
		.insert(MainPassCube);

	// Defines initial transform of cameras and light.
	let target = Vec3::ZERO;
	let eye = Vec3::new(0.0, 0.0, 15.0);
	let up = Vec3::Y;

	// The same light is reused for both passes, you can specify different lights for preview and
	// main pass by setting appropriate `RenderLayers`.
	commands
		.spawn((
			Transform::from_translation(eye).looking_at(target, up),
			SpotLight {
				intensity: 10_000_000.0,
				range: 100.0,
				..default()
			},
		))
		.insert(RenderLayers::default().with(1));

	// The main pass camera with controller.
	let controller = commands
		.spawn((
			TrackballController::default(),
			TrackballCamera::look_at(target, eye, up),
			Camera3d::default(),
		))
		.id();

	// UI camera sensitive to main pass camera's `controller`.
	commands
		.spawn((
			TrackballCamera::look_at(target, eye, up).add_controller(controller, true),
			Camera {
				// Render before the main pass camera.
				order: -1,
				target: RenderTarget::Image(image_handle),
				clear_color: ClearColorConfig::Custom(Color::srgba(1.0, 1.0, 1.0, 0.0)),
				..default()
			},
			Camera3d::default(),
		))
		.insert(preview_pass_layer);
}

fn render_to_image_example_system(
	cube_preview_image: Res<CubePreviewImage>,
	preview_cube_query: Query<&MeshMaterial3d<StandardMaterial>, With<PreviewPassCube>>,
	main_cube_query: Query<&MeshMaterial3d<StandardMaterial>, With<MainPassCube>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut contexts: EguiContexts,
) {
	let cube_preview_texture_id = contexts.image_id(&cube_preview_image).unwrap();
	let preview_material_handle = preview_cube_query.single();
	let preview_material = materials.get_mut(preview_material_handle).unwrap();

	let ctx = contexts.ctx_mut();
	let mut apply = false;
	egui::Window::new("Cube material preview").show(ctx, |ui| {
		ui.image(egui::load::SizedTexture::new(
			cube_preview_texture_id,
			egui::vec2(300.0, 300.0),
		));
		egui::Grid::new("preview").show(ui, |ui| {
			ui.label("Base color:");
			color_picker_widget(ui, &mut preview_material.base_color);
			ui.end_row();

			ui.label("Emissive:");
			let mut emissive_color = Color::from(preview_material.emissive);
			color_picker_widget(ui, &mut emissive_color);
			preview_material.emissive = emissive_color.into();
			ui.end_row();

			ui.label("Perceptual roughness:");
			egui::Slider::new(&mut preview_material.perceptual_roughness, 0.089..=1.0).ui(ui);
			ui.end_row();

			ui.label("Reflectance:");
			egui::Slider::new(&mut preview_material.reflectance, 0.0..=1.0).ui(ui);
			ui.end_row();

			ui.label("Unlit:");
			ui.checkbox(&mut preview_material.unlit, "");
			ui.end_row();
		});

		apply = ui.button("Apply").clicked();
	});

	if apply {
		let material_clone = preview_material.clone();

		let main_material_handle = main_cube_query.single();
		materials.insert(main_material_handle, material_clone);
	}
}

fn color_picker_widget(ui: &mut egui::Ui, color: &mut Color) -> egui::Response {
	let [r, g, b, a] = Srgba::from(*color).to_f32_array();
	let mut egui_color: egui::Rgba = egui::Rgba::from_srgba_unmultiplied(
		(r * 255.0) as u8,
		(g * 255.0) as u8,
		(b * 255.0) as u8,
		(a * 255.0) as u8,
	);
	let res = egui::widgets::color_picker::color_edit_button_rgba(
		ui,
		&mut egui_color,
		egui::color_picker::Alpha::Opaque,
	);
	let [r, g, b, a] = egui_color.to_srgba_unmultiplied();
	*color = Color::srgba(
		r as f32 / 255.0,
		g as f32 / 255.0,
		b as f32 / 255.0,
		a as f32 / 255.0,
	);
	res
}

fn camera_attached_light(
	camera_transform: Query<
		&Transform,
		(
			Changed<Transform>,
			With<TrackballController>,
			Without<SpotLight>,
		),
	>,
	mut light_transform: Query<&mut Transform, With<SpotLight>>,
) {
	if let Ok(camera_transform) = camera_transform.get_single() {
		*light_transform.single_mut() = *camera_transform;
	}
}
