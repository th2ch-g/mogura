use bevy::{
	input::{mouse::MouseWheel, touch::TouchPhase},
	prelude::*,
	render::camera::RenderTarget,
	window::{PrimaryWindow, WindowRef},
};

use super::{TrackballCamera, TrackballController};

/// Trackball viewport currently focused and hence capturing input events.
///
///  * Enables multiple viewports/windows with individual controllers/cameras.
///  * Enables UI systems to steal the viewport in order to capture input events.
#[derive(Resource, Clone, Debug, PartialEq, Eq, Default)]
pub struct TrackballViewport {
	entity: Option<Entity>,
	stolen: usize,
}

impl TrackballViewport {
	/// Condition whether the viewport has been stolen, evaluated by [`IntoSystemConfigs::run_if`].
	///
	/// Interferes with automatic viewport stealing if the `bevy_egui` feature is enabled. As
	/// automatic viewport stealing gives the viewport back with `set_stolen(None)` instead of
	/// `set_stolen(Some(0))`, you can override it in the same frame for your own input capturing.
	#[allow(clippy::needless_pass_by_value)]
	#[must_use]
	pub fn stolen(viewport: Res<Self>) -> bool {
		viewport.stolen != 0
	}
	/// Whether viewport has just been given back.
	///
	/// Interferes with automatic viewport stealing if the `bevy_egui` feature is enabled. As
	/// automatic viewport stealing gives the viewport back with `set_stolen(None)` instead of
	/// `set_stolen(Some(0))`, you can override it in the same frame for your own input capturing.
	#[must_use]
	pub const fn was_stolen(&self) -> bool {
		self.entity.is_none()
	}
	/// Steals the viewport or gives it back.
	///
	/// Interferes with automatic viewport stealing if the `bevy_egui` feature is enabled. As
	/// automatic viewport stealing gives the viewport back with `set_stolen(None)` instead of
	/// `set_stolen(Some(0))`, you can override it in the same frame for your own input capturing.
	///
	/// # Examples
	///
	/// Steals the viewport for `Some(frames)` and lets it count `frames` down with `None`:
	///
	/// ```ignore
	/// fn system(/* ... */) {
	/// 	viewport.set_stolen(just_stolen.then_some(3));
	/// }
	///
	/// // frame 0: just_stolen = true  -> set_stolen(Some(3)) -> frames = 3 -> stolen = true
	/// // frame 1: just_stolen = false -> set_stolen(None)    -> frames = 2 -> stolen = true
	/// // frame 2: just_stolen = false -> set_stolen(None)    -> frames = 1 -> stolen = true
	/// // frame 3: just_stolen = false -> set_stolen(None)    -> frames = 0 -> stolen = false
	/// ```
	///
	/// Steals the viewport with `Some(1)` and gives it back with `Some(0)`:
	///
	/// ```ignore
	/// fn system(/* ... */) {
	/// 	if just_stolen {
	/// 		viewport.set_stolen(Some(1));
	/// 	}
	/// 	if just_give_back {
	/// 		viewport.set_stolen(Some(0));
	/// 	}
	/// }
	///
	/// // frame  0: just_stolen = true    -> set_stolen(Some(1)) -> frames = 1 -> stolen = true
	/// // frame  1: just_stolen = false   ->                     -> frames = 1 -> stolen = true
	/// // frame 25: just_stolen = false   ->                     -> frames = 1 -> stolen = true
	/// // frame 50: just_give_back = true -> set_stolen(Some(0)) -> frames = 0 -> stolen = false
	/// ```
	#[allow(clippy::needless_pass_by_value)]
	pub fn set_stolen(&mut self, stolen: Option<usize>) {
		if let Some(frames) = stolen {
			self.entity = None;
			self.stolen = frames;
		} else if self.stolen != 0 {
			self.stolen -= 1;
		}
	}
	#[allow(clippy::too_many_arguments)]
	#[allow(clippy::type_complexity)]
	pub(super) fn select<'a>(
		viewport: &mut ResMut<Self>,
		key_input: &Res<ButtonInput<KeyCode>>,
		mouse_input: &Res<ButtonInput<MouseButton>>,
		touch_events: &mut EventReader<TouchInput>,
		wheel_events: &EventReader<MouseWheel>,
		primary_windows: &'a mut Query<(Entity, &mut Window), With<PrimaryWindow>>,
		secondary_windows: &'a mut Query<&mut Window, Without<PrimaryWindow>>,
		cameras: &'a mut Query<(Entity, &Camera, &TrackballCamera, &mut TrackballController)>,
	) -> Option<(
		bool,
		Entity,
		Mut<'a, Window>,
		Entity,
		&'a Camera,
		&'a TrackballCamera,
		Mut<'a, TrackballController>,
	)> {
		let touch = touch_events
			.read()
			.filter_map(|touch| (touch.phase == TouchPhase::Started).then_some(touch.position))
			.last();
		let input = !wheel_events.is_empty()
			|| key_input.get_just_pressed().len() != 0
			|| mouse_input.get_just_pressed().len() != 0;
		let mut new_viewport = viewport.clone();
		let mut max_order = 0;
		for (group, camera, _trackball, _controller) in cameras.iter() {
			let RenderTarget::Window(window_ref) = camera.target else {
				continue;
			};
			let window = match window_ref {
				WindowRef::Primary => primary_windows
					.get_single()
					.ok()
					.map(|(_id, window)| window),
				WindowRef::Entity(id) => secondary_windows.get(id).ok(),
			};
			let Some(window) = window else {
				continue;
			};
			let Some(pos) = touch
				.filter(|_pos| window.focused)
				.or_else(|| window.cursor_position().filter(|_pos| input))
			else {
				continue;
			};
			let Some(Rect { min, max }) = camera.logical_viewport_rect() else {
				continue;
			};
			let contained = (min.x..max.x).contains(&pos.x) && (min.y..max.y).contains(&pos.y);
			if contained && camera.order >= max_order {
				new_viewport.entity = Some(group);
				max_order = camera.order;
			}
		}
		let is_changed = viewport.entity != new_viewport.entity;
		if is_changed {
			viewport.entity = new_viewport.entity;
		}
		let camera = viewport
			.entity
			.and_then(|entity| cameras.get_mut(entity).ok());
		let Some((group, camera, trackball, controller)) = camera else {
			viewport.entity = None;
			return None;
		};
		let RenderTarget::Window(window_ref) = camera.target else {
			return None;
		};
		let (window_id, window) = match window_ref {
			WindowRef::Primary => primary_windows.get_single_mut().ok(),
			WindowRef::Entity(id) => secondary_windows
				.get_mut(id)
				.ok()
				.map(|window| (id, window)),
		}?;
		Some((
			is_changed, window_id, window, group, camera, trackball, controller,
		))
	}
}
