//! Coherent virtual trackball controller/camera plugin for Bevy
//!
//! Run interactive [examples] in your browser using [WebAssembly] and [WebGL].
//!
//! [WebAssembly]: https://en.wikipedia.org/wiki/WebAssembly
//! [WebGL]: https://en.wikipedia.org/wiki/WebGL
//!
//! **NOTE**: Not all features are enabled by default, see [Optional Features](#optional-features).
//! On Linux the `bevy/wayland` or `bevy/x11` feature gate must be enabled for a successful build.
//!
//! # Camera Modes
//!
//! Supports multiple camera modes:
//!
//!   * Trackball mode rotates camera around target.
//!   * First-person mode rotates target around camera.
//!   * Spectator mode translates target and camera.
//!
//! # Coherence Features
//!
//! This is an alternative trackball technique using exponential map and parallel transport to
//! preserve distances and angles for inducing coherent and intuitive trackball rotations. For
//! instance, displacements on straight radial lines through the screen’s center are carried to arcs
//! of the same length on great circles of the trackball (e.g., dragging the mouse along an eights
//! of the trackball's circumference rolls the camera by 360/8=45 degrees, dragging the mouse from
//! the screen's center to its further edge *linearly* rotates the camera by 1 [radian], where the
//! trackball's diameter is the maximum of the screen's width and height). This is in contrast to
//! state-of-the-art techniques using orthogonal projection which distorts radial distances further
//! away from the screen’s center (e.g., the rotation accelerates towards the edge).
//!
//! [radian]: https://en.wikipedia.org/wiki/Radian
//!
//!   * Coherent and intuitive orbiting via the exponential map, see the underlying [`trackball`]
//!     crate which follows the recipe given in the paper of Stantchev, G.. “Virtual Trackball
//!     Modeling and the Exponential Map.”. [S2CID] [44199608]. See the [`exponential_map`] example.
//!   * Coherent first-person mode aka free look or mouse look with the world trackball centered at
//!     eye instead of target.
//!   * Coherent scaling by translating mouse wheel device units, see [`TrackballWheelUnit`]. Scales
//!     eye distance from current cursor position or centroid of finger positions projected onto
//!     focus plane.
//!   * Coherent linear/angular [`TrackballVelocity`] for sliding/orbiting or free look by
//!     time-based input (e.g., pressed key). By default, the linear velocity is deduced from the
//!     angular velocity (where target and eye positions define the world radius) which in turn is
//!     defined in units of vertical field of view per seconds and hence independent of the world
//!     unit scale.
//!
//! [S2CID]: https://en.wikipedia.org/wiki/S2CID_(identifier)
//! [44199608]: https://api.semanticscholar.org/CorpusID:44199608
//!
//! # Additional Features
//!
//!   * Time-free multi-touch gesture recognition for orbit, scale, slide, and focus (i.e., slide to
//!     cursor/finger position) operations.
//!   * Smoothing of movement implemented as fps-agnostic exponential ease-out.
//!   * Gimbal lock-free using quaternion instead of Euler angles.
//!   * Gliding clamp (experimental): The movement of a camera can be restricted to user-defined
//!     boundary conditions (e.g., to not orbit below the ground plane). When the movement is not
//!     orthogonal to a boundary plane, it is changed such that the camera glides along the boundary
//!     plane. Currently, only implemented for orbit and slide operations, see the [`gliding_clamp`]
//!     example.
//!   * Camera constellation: A camera is decoupled from its input controller and instead multiple
//!     cameras can be sensitive to zero or multiple selected controllers (e.g., a minimap
//!     controlled by the same controller of the main viewport).
//!   * Constellation clamp: Cameras sensitive to the same controller are referred to as a group
//!     and can be configured to clamp the movement for the whole group whenever a group member
//!     crosses a boundary condition (e.g., rigid and loose constellation clamp), see the
//!     [`constellation_clamp`] example.
//!   * Viewport stealing: This allows UI system (e.g., egui behind `bevy_egui` feature gate) to
//!     steal the viewport and hence capture the input instead, see the [`egui`] example.
//!   * Scale-preserving transitioning between orthographic and perspective projection mode.
//!   * Converting between scaling modes (i.e., fixed vertical or horizontal field of view or fixed
//!     unit per pixels). This defines whether the scene scales or the corresponding vertical or
//!     horizontal field of view adjusts whenever the height or width of the viewport is resized,
//!     see the [`scaling_modes`] example.
//!   * Object inspection mode scaling clip plane distances by measuring from target instead of eye.
//!     This benefits the precision of the depth map. Applicable, whenever the extend of the object
//!     to inspect is known and hence the near clip plane can safely be placed just in front of it.
//!   * `f64`-ready for large worlds (e.g., solar system scale) whenever Bevy is, see issue [#1680].
//!
//! [#1680]: https://github.com/bevyengine/bevy/issues/1680
//!
//! # Optional Features
//!
//! Following features are disabled unless their corresponding feature gate is enabled:
//!
//!   * `bevy_egui` for automatic viewport stealing whenever `egui` wants focus.
//!   * `serialize` for `serde` support of various structures of this crate and its dependencies.
//!   * `c11-orbit` for testing the behaviorally identical C implementation of the exponential map.
//!
//! # Roadmap
//!
//!   * Implement gliding clamp for first-person mode and scale operation, see
//!     [issue](https://github.com/qu1x/bevy_trackball/issues/5).
//!   * Support more camera modes out of the box by adding dedicated controllers for each mode, see
//!     [issue](https://github.com/qu1x/bevy_trackball/issues/3).
//!   * Support gamepad inputs, see [issue](https://github.com/qu1x/bevy_trackball/issues/4).
//!
//! # Input Mappings
//!
//! Following mappings are the defaults which can be customized, see [`TrackballInput`].
//!
//! Mouse (Buttons)         | Touch (Fingers)         | Keyboard | Operation
//! ----------------------- | ----------------------- | -------- | ---------------------------------
//! Left Press + Drag       | One + Drag              | `ijkl`   | Orbits around target.
//! ↳ at trackball's border | Two + Roll              | `uo`     | Rolls about view direction.
//! Middle Press + Drag     | Any + Drag + Left Shift | `↑←↓→`   | First-person mode.
//! Right Press + Drag      | Two + Drag              | `esdf`   | Slides trackball on focus plane.
//! &nbsp;                  | &nbsp;                  | `gv`     | Slides trackball in/out.
//! Scroll In/Out           | Two + Pinch Out/In      | `hn`     | Scales distance zooming in/out.
//! Left Press + Release    | Any + Release           | &nbsp;   | Slides to cursor/finger position.
//! &nbsp;                  | &nbsp;                  | `m`      | Toggle `esdf`/`wasd` mapping.
//! &nbsp;                  | &nbsp;                  | `p`      | Toggle orthographic/perspective.
//! &nbsp;                  | &nbsp;                  | `Enter`  | Reset camera transform.
//!
//! Alternatively, [`TrackballInput::map_wasd`] maps `wasd`/`Space`/`ControlLeft` to slide
//! operations where `ws` slides in/out and `Space`/`ControlLeft` slides up/down (jump/crouch).
//!
//! # Usage
//!
//! Add the [`TrackballPlugin`] followed by spawning a [`TrackballController`] together with a
//! [`TrackballCamera`] and a `Camera3dBundle` or try the interactive [examples].
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_trackball::prelude::*;
//!
//! // Add the trackball plugin.
//! fn main() {
//! 	App::new()
//! 		.add_plugins(DefaultPlugins)
//! 		.add_plugins(TrackballPlugin)
//! 		.add_systems(Startup, setup)
//! 		.run();
//! }
//!
//! // Add a trackball controller and trackball camera to a camera 3D bundle.
//! fn setup(mut commands: Commands) {
//! 	let [target, eye, up] = [Vec3::ZERO, Vec3::Z * 10.0, Vec3::Y];
//! 	commands.spawn((
//! 		TrackballController::default(),
//! 		TrackballCamera::look_at(target, eye, up),
//! 		Camera3d::default(),
//! 	));
//!
//! 	// Set up your scene...
//! }
//! ```
//!
//! [examples]: https://qu1x.dev/bevy_trackball
//! [`exponential_map`]: https://qu1x.dev/bevy_trackball/exponential_map.html
//! [`gliding_clamp`]: https://qu1x.dev/bevy_trackball/gliding_clamp.html
//! [`constellation_clamp`]: https://qu1x.dev/bevy_trackball/constellation_clamp.html
//! [`egui`]: https://qu1x.dev/bevy_trackball/egui.html
//! [`scaling_modes`]: https://github.com/qu1x/bevy_trackball/blob/main/examples/scaling_modes.rs

use bevy::prelude::*;
use camera::trackball_camera;
pub use camera::TrackballCamera;
use constellation::trackball_constellation;
use controller::trackball_controller;
pub use controller::{
	TrackballController, TrackballInput, TrackballVelocity, TrackballViewport, TrackballWheelUnit,
};
pub use trackball;
use trackball::{
	nalgebra::{Point3, Unit, UnitQuaternion, Vector3},
	Delta,
};

/// Prelude to get started quickly.
pub mod prelude {
	pub use super::{
		trackball::{
			approx::{
				abs_diff_eq, abs_diff_ne, assert_abs_diff_eq, assert_abs_diff_ne,
				assert_relative_eq, assert_relative_ne, assert_ulps_eq, assert_ulps_ne,
				relative_eq, relative_ne, ulps_eq, ulps_ne, AbsDiffEq, RelativeEq, UlpsEq,
			},
			nalgebra::{Isometry3, Point3, Unit, UnitQuaternion, Vector3},
			Bound, Clamp, Delta, Fixed, Frame, Plane, Scope,
		},
		TrackballCamera, TrackballController, TrackballEvent, TrackballInput, TrackballPlugin,
		TrackballSetup, TrackballSystemSet, TrackballVelocity, TrackballViewport,
		TrackballWheelUnit,
	};
}
mod camera;
mod constellation;
mod controller;

/// Plugin adding and configuring systems and their resources.
///
/// Halts [`TrackballSystemSet::Controller`] for supported UI systems (i.e., `bevy_egui` feature
/// gate) whenever they request focus by marking the active viewport as stolen.
///
/// See [`TrackballViewport::set_stolen`] in order to steal the viewport and hence exclusively
/// consume its input events for UI systems that are not yet supported behind feature gate.
#[derive(Default)]
pub struct TrackballPlugin;

/// Event sent from [`TrackballController`] component to group of [`TrackballCamera`] components.
#[derive(Event)]
pub struct TrackballEvent {
	/// Entity of [`TrackballController`] component which sent this event.
	///
	/// Read by group of [`TrackballCamera`] components which knows about this entity.
	pub group: Entity,
	/// Delta transform from initial to final [`Frame`] of [`TrackballCamera`].
	///
	/// [`Frame`]: trackball::Frame
	pub delta: Delta<f32>,
	/// Setup of [`TrackballCamera`].
	pub setup: Option<TrackballSetup>,
}

impl TrackballEvent {
	/// Creates [`Delta::First`] event for camera `group`.
	#[must_use]
	#[inline]
	pub const fn first(group: Entity, pitch: f32, yaw: f32, yaw_axis: Unit<Vector3<f32>>) -> Self {
		Self {
			group,
			delta: Delta::First {
				pitch,
				yaw,
				yaw_axis,
			},
			setup: None,
		}
	}
	/// Creates [`Delta::Track`] event for camera `group`.
	#[must_use]
	#[inline]
	pub const fn track(group: Entity, vec: Vector3<f32>) -> Self {
		Self {
			group,
			delta: Delta::Track { vec },
			setup: None,
		}
	}
	/// Creates [`Delta::Orbit`] event for camera `group`.
	#[must_use]
	#[inline]
	pub const fn orbit(group: Entity, rot: UnitQuaternion<f32>, pos: Point3<f32>) -> Self {
		Self {
			group,
			delta: Delta::Orbit { rot, pos },
			setup: None,
		}
	}
	/// Creates [`Delta::Slide`] event for camera `group`.
	#[must_use]
	#[inline]
	pub const fn slide(group: Entity, vec: Vector3<f32>) -> Self {
		Self {
			group,
			delta: Delta::Slide { vec },
			setup: None,
		}
	}
	/// Creates [`Delta::Scale`] event for camera `group`.
	#[must_use]
	#[inline]
	pub const fn scale(group: Entity, rat: f32, pos: Point3<f32>) -> Self {
		Self {
			group,
			delta: Delta::Scale { rat, pos },
			setup: None,
		}
	}
	/// Creates [`TrackballSetup::Reset`] event for camera `group`.
	#[must_use]
	#[inline]
	pub const fn reset(group: Entity) -> Self {
		Self {
			group,
			delta: Delta::Frame,
			setup: Some(TrackballSetup::Reset),
		}
	}
	/// Creates [`TrackballSetup::Ortho`] event for camera `group`.
	#[must_use]
	#[inline]
	pub const fn ortho(group: Entity, ortho: Option<bool>) -> Self {
		Self {
			group,
			delta: Delta::Frame,
			setup: Some(TrackballSetup::Ortho(ortho)),
		}
	}
}

/// Setup of [`TrackballCamera`] as part of [`TrackballEvent`].
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum TrackballSetup {
	/// Reset camera frame.
	Reset,
	/// Set projection mode.
	///
	///   * Orthographic with `Some(true)`
	///   * Perspective with `Some(false)`
	///   * Toggle with `None`
	Ortho(Option<bool>),
}

/// System sets configured by [`TrackballPlugin`].
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
#[non_exhaustive]
pub enum TrackballSystemSet {
	/// Trackball controller system translating [`TrackballInput`] of [`TrackballController`]
	/// components into [`TrackballEvent`] for [`TrackballSystemSet::Constellation`].
	Controller,
	/// Trackball constellation system translating [`TrackballEvent`] from [`TrackballController`]
	/// components into new [`Frame`] and [`Scope`] of [`TrackballCamera`] components.
	///
	/// [`Frame`]: trackball::Frame
	/// [`Scope`]: trackball::Scope
	Constellation,
	/// Trackball camera system translating [`Frame`] and [`Scope`] of [`TrackballCamera`]
	/// components into [`Transform`] and [`Projection`] bundles (e.g., `Camera3DBundle`).
	///
	/// [`Frame`]: trackball::Frame
	/// [`Scope`]: trackball::Scope
	Camera,
}

impl Plugin for TrackballPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<TrackballViewport>()
			.add_event::<TrackballEvent>()
			.add_systems(
				Update,
				(
					trackball_controller
						.in_set(TrackballSystemSet::Controller)
						.run_if(not(TrackballViewport::stolen)),
					trackball_constellation.in_set(TrackballSystemSet::Constellation),
					trackball_camera.in_set(TrackballSystemSet::Camera),
				)
					.chain(),
			);
		#[cfg(feature = "bevy_egui")]
		{
			use bevy_egui::{EguiContext, EguiSet};

			fn egui_viewport_theft(
				mut viewport: ResMut<TrackballViewport>,
				mut contexts: Query<&mut EguiContext>,
			) {
				let stolen = contexts.iter_mut().next().is_some_and(|mut context| {
					let context = context.get_mut();
					context.wants_pointer_input() || context.wants_keyboard_input()
				});
				viewport.set_stolen(stolen.then_some(2));
			}

			app.add_systems(
				Update,
				egui_viewport_theft
					.after(EguiSet::InitContexts)
					.before(TrackballSystemSet::Controller),
			);
		}
	}
}
