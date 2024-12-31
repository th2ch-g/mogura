use std::collections::HashMap;

use bevy::prelude::*;
use trackball::{approx::AbsDiffEq, nalgebra::Point2, Clamp, Delta, Fixed, Frame, Scope};

/// Trackball camera component mainly defined by [`Frame`] and [`Scope`].
#[derive(Component, Debug)]
pub struct TrackballCamera {
	/// Camera frame defining [`Transform`].
	///
	/// Comprises following properties:
	///
	///   * target position as trackball center
	///   * camera eye rotation on trackball surface (incl. roll, gimbal lock-free using quaternion)
	///   * trackball radius
	pub frame: Frame<f32>,
	old_frame: Frame<f32>,
	/// Camera scope defining [`Projection`].
	///
	/// Comprises following properties:
	///
	///   * field of view angle (default is 45 degrees) and its mode of either [`Fixed::Ver`]
	///     (default), [`Fixed::Hor`], or [`Fixed::Upp`].
	///   * projection mode of either perspective (default) or orthographic (scale preserving)
	///   * clip planes either measured from eye (default) or target (object inspection mode)
	pub scope: Scope<f32>,
	old_scope: Scope<f32>,
	old_max: Point2<f32>,
	/// Blend half-life from 0 (fast) to 1000 (slow) milliseconds. Default is `40.0`.
	///
	/// It is the time passed until halfway of fps-agnostic exponential ease-out.
	pub blend: f32,
	/// Camera frame to reset to when [`TrackballInput::reset_key`] is pressed.
	///
	/// [`TrackballInput::reset_key`]: crate::TrackballInput::reset_key
	pub reset: Frame<f32>,
	/// User boundary conditions clamping camera [`Frame`].
	///
	/// Allows to limit target/eye position or minimal/maximal target/eye distance or up rotation.
	pub clamp: Option<Box<dyn Clamp<f32>>>,
	pub(crate) delta: Option<Delta<f32>>,
	/// Additional [`TrackballController`] entities to which this camera is sensitive.
	///
	/// It is always sensitive to its own controller if it has one. A mapped value of `true` will
	/// clamp the active controller as well and hence all other cameras of this group whenever this
	/// camera is clamped. If `false`, only this camera is clamped whereas other cameras of this
	/// group continue to follow the active controller.
	///
	/// [`TrackballController`]: crate::TrackballController
	/// [`TrackballEvent`]: crate::TrackballEvent
	pub group: HashMap<Entity, bool>,
}

impl TrackballCamera {
	/// Defines camera with `target` position and `eye` position inclusive its roll attitude (`up`).
	#[must_use]
	pub fn look_at(target: Vec3, eye: Vec3, up: Vec3) -> Self {
		let frame = Frame::look_at(target.into(), &eye.into(), &up.into());
		Self {
			frame,
			old_frame: Frame::default(),
			scope: Scope::default(),
			old_scope: Scope::default(),
			old_max: Point2::default(),
			blend: 40.0,
			reset: frame,
			clamp: None,
			delta: None,
			group: HashMap::default(),
		}
	}
	/// Defines scope, see [`Self::scope`].
	#[must_use]
	#[allow(clippy::type_complexity)]
	pub const fn with_scope(mut self, scope: Scope<f32>) -> Self {
		self.scope = scope;
		self
	}
	/// Defines blend half-life, see [`Self::blend`].
	#[must_use]
	pub const fn with_blend(mut self, blend: f32) -> Self {
		self.blend = blend;
		self
	}
	/// Defines reset frame, see [`Self::reset`].
	#[must_use]
	#[allow(clippy::type_complexity)]
	pub const fn with_reset(mut self, reset: Frame<f32>) -> Self {
		self.reset = reset;
		self
	}
	/// Defines user boundary conditions, see [`Self::clamp`].
	#[must_use]
	#[allow(clippy::type_complexity)]
	pub fn with_clamp(mut self, clamp: impl Clamp<f32>) -> Self {
		self.clamp = Some(Box::new(clamp));
		self
	}
	/// Adds additional controller to which this camera is sensitive, see [`Self::group`].
	#[must_use]
	pub fn add_controller(mut self, id: Entity, rigid: bool) -> Self {
		self.group.insert(id, rigid);
		self
	}
}

#[allow(clippy::needless_pass_by_value)]
pub fn trackball_camera(
	time: Res<Time>,
	mut cameras: Query<(
		&Camera,
		&mut TrackballCamera,
		&mut Transform,
		&mut Projection,
	)>,
) {
	for (camera, mut trackball, mut transform, mut projection) in &mut cameras {
		let Some(max) = camera.logical_viewport_size().map(Point2::from) else {
			continue;
		};
		#[allow(clippy::float_cmp)]
		let new_zat = trackball.frame.distance() != trackball.old_frame.distance();
		if trackball.frame != trackball.old_frame {
			if trackball.old_frame == Frame::default() {
				trackball.old_frame = trackball.frame;
			}
			let blend = (trackball.blend * 1e-3).clamp(0.0, 1.0);
			let blend = 1.0 - 0.5f32.powf(time.delta_secs() / blend);
			trackball.old_frame = trackball
				.old_frame
				.abs_diff_ne(&trackball.frame, f32::EPSILON.sqrt())
				.then(|| {
					trackball
						.old_frame
						.try_lerp_slerp(&trackball.frame, blend, 0.0)
						.map(|mut frame| {
							frame.renormalize();
							frame
						})
				})
				.flatten()
				.unwrap_or(trackball.frame);
			let view = trackball.old_frame.view();
			transform.translation = view.translation.into();
			transform.rotation = view.rotation.into();
		}
		let new_scope = trackball.scope != trackball.old_scope;
		let new_max = max != trackball.old_max;
		trackball.old_scope = trackball.scope;
		trackball.old_max = max;
		let fov = trackball.scope.fov();
		let zat = trackball.old_frame.distance();
		let (near, far) = trackball.scope.clip_planes(zat);
		if trackball.scope.ortho() {
			if new_scope || new_max || new_zat {
				let (_max, upp) = fov.max_and_upp(zat, &max);
				*projection = Projection::Orthographic(OrthographicProjection {
					near,
					far,
					scale: upp,
					..OrthographicProjection::default_3d()
				});
			}
		} else if new_scope || (new_max && !matches!(fov, Fixed::Ver(_fov))) {
			let fov = fov.to_ver(&max).into_inner();
			let aspect_ratio = max.x / max.y;
			*projection = Projection::Perspective(PerspectiveProjection {
				fov,
				aspect_ratio,
				near,
				far,
			});
		}
	}
}
