use bevy::{prelude::*, window::CursorGrabMode};
use trackball::nalgebra::{Point3, Unit, UnitQuaternion};

use crate::{TrackballCamera, TrackballController, TrackballEvent};

#[allow(clippy::too_many_arguments)]
#[allow(clippy::too_many_lines)]
pub fn key(
	group: Entity,
	trackball_events: &mut EventWriter<TrackballEvent>,
	trackball: &TrackballCamera,
	controller: &mut TrackballController,
	window: &mut Window,
	key_input: &Res<ButtonInput<KeyCode>>,
	mouse_input: &Res<ButtonInput<MouseButton>>,
	zat: f32,
	w: f32,
	v: f32,
	t: f32,
) {
	let pressed = |key: Option<KeyCode>| key.is_some_and(|key| key_input.pressed(key));
	let just_pressed = |key: Option<KeyCode>| key.is_some_and(|key| key_input.just_pressed(key));
	let just_released = |key: Option<KeyCode>| key.is_some_and(|key| key_input.just_released(key));
	let pressed_button =
		|button: Option<MouseButton>| button.is_some_and(|button| mouse_input.pressed(button));
	if just_pressed(controller.input.reset_key) {
		trackball_events.send(TrackballEvent::reset(group));
	}
	if just_pressed(controller.input.ortho_key) {
		trackball_events.send(TrackballEvent::ortho(group, None));
	}
	if just_pressed(controller.input.gamer_key) {
		if controller.input.slide_far_key == Some(KeyCode::KeyW) {
			controller.input.map_esdf();
		} else {
			controller.input.map_wasd();
		}
	}
	for (key, vec) in [
		(controller.input.slide_far_key, Vec3::NEG_Z),
		(controller.input.slide_near_key, Vec3::Z),
		(controller.input.slide_left_key, Vec3::NEG_X),
		(controller.input.slide_right_key, Vec3::X),
		(controller.input.slide_up_key, Vec3::Y),
		(controller.input.slide_down_key, Vec3::NEG_Y),
	] {
		if pressed(key) {
			trackball_events.send(TrackballEvent::slide(group, (vec * v * t).into()));
		}
	}
	for (key, vec) in [
		(controller.input.screw_left_key, Vec3::Z),
		(controller.input.screw_right_key, Vec3::NEG_Z),
		(controller.input.orbit_left_key, Vec3::NEG_Y),
		(controller.input.orbit_right_key, Vec3::Y),
		(controller.input.orbit_up_key, Vec3::NEG_X),
		(controller.input.orbit_down_key, Vec3::X),
	] {
		if pressed(key) {
			trackball_events.send(TrackballEvent::orbit(
				group,
				UnitQuaternion::from_axis_angle(&Unit::new_unchecked(vec.into()), w * t),
				Point3::origin(),
			));
		}
	}
	for key in [
		controller.input.first_left_key,
		controller.input.first_right_key,
		controller.input.first_up_key,
		controller.input.first_down_key,
	] {
		if just_pressed(key) {
			controller.first_count += 1;
		}
		if just_released(key) && controller.first_count != 0 {
			controller.first_count -= 1;
		}
	}
	if !pressed(controller.input.first_key) && !pressed_button(controller.input.first_button) {
		if controller.first_count == 0 {
			controller.first.discard();
		} else if !controller.first.enabled() {
			controller.first.capture(trackball.frame.yaw_axis());
		}
	}
	for (key, vec) in [
		(controller.input.first_left_key, Vec2::Y),
		(controller.input.first_right_key, Vec2::NEG_Y),
		(controller.input.first_up_key, Vec2::X),
		(controller.input.first_down_key, Vec2::NEG_X),
	] {
		if pressed(key) {
			let ang = vec * w * t;
			let yaw_axis = *controller.first.yaw_axis().unwrap();
			trackball_events.send(TrackballEvent::first(group, ang.x, ang.y, yaw_axis));
		}
	}
	if just_pressed(controller.input.first_key) {
		controller.first.capture(trackball.frame.yaw_axis());
		window.cursor_options.grab_mode = CursorGrabMode::Locked;
		window.cursor_options.visible = false;
	}
	if just_released(controller.input.first_key) {
		controller.first.discard();
		window.cursor_options.grab_mode = CursorGrabMode::None;
		window.cursor_options.visible = true;
	}
	controller.scale.set_denominator(zat);
	if pressed(controller.input.scale_in_key) {
		trackball_events.send(TrackballEvent::scale(
			group,
			controller.scale.compute(v * t),
			Point3::origin(),
		));
	}
	if pressed(controller.input.scale_out_key) {
		trackball_events.send(TrackballEvent::scale(
			group,
			controller.scale.compute(-v * t),
			Point3::origin(),
		));
	}
}
