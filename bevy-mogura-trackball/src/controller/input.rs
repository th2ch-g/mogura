use bevy::prelude::*;
use trackball::Fixed;

/// Trackball controller input mappings and settings.
#[derive(Component, Debug, Clone)]
pub struct TrackballInput {
	/// Trackball velocity for time-based input like pressed keys.
	pub velocity: TrackballVelocity,
	/// Wheel unit for coherent scaling. Default is 24 clicks per turn.
	///
	/// Device dependent setting as mouse wheel events usually lack a reference unit.
	pub wheel_unit: TrackballWheelUnit,

	/// Enables focus operation. Default is `true`.
	///
	/// Whether to slide towards mouse or single-finger touch position when [`Self::orbit_button`]
	/// is just pressed and released again or single-finger gesture is just started and ended again.
	/// Moving the cursor/finger slightly between pressed/started and released/ended events discards
	/// the focus operation in favor of the orbit operation.
	pub focus: bool,

	/// Key used to toggle `esdf`/`wasd` mapping. Default is [`KeyCode::KeyM`].
	pub gamer_key: Option<KeyCode>,
	/// Key used to toggle projection mode. Default is [`KeyCode::KeyP`].
	pub ortho_key: Option<KeyCode>,

	/// Key used to reset frame. Default is [`KeyCode::Enter`].
	pub reset_key: Option<KeyCode>,

	/// Mouse button used to look around. Default is [`MouseButton::Middle`].
	pub first_button: Option<MouseButton>,
	/// Key used to look around with single-finger touch. Default is [`KeyCode::ShiftLeft`].
	pub first_key: Option<KeyCode>,
	/// Key used to look left. Default is [`KeyCode::ArrowLeft`].
	pub first_left_key: Option<KeyCode>,
	/// Key used to look right. Default is [`KeyCode::ArrowRight`].
	pub first_right_key: Option<KeyCode>,
	/// Key used to look up. Default is [`KeyCode::ArrowUp`].
	pub first_up_key: Option<KeyCode>,
	/// Key used to look down. Default is [`KeyCode::ArrowDown`].
	pub first_down_key: Option<KeyCode>,

	/// Mouse button used to orbit camera. Default is [`MouseButton::Left`].
	pub orbit_button: Option<MouseButton>,
	/// Key used to screw/roll left. Default is [`KeyCode::KeyU`].
	pub screw_left_key: Option<KeyCode>,
	/// Key used to screw/roll right. Default is [`KeyCode::KeyO`].
	pub screw_right_key: Option<KeyCode>,
	/// Key used to orbit left. Default is [`KeyCode::KeyJ`].
	pub orbit_left_key: Option<KeyCode>,
	/// Key used to orbit right. Default is [`KeyCode::KeyL`].
	pub orbit_right_key: Option<KeyCode>,
	/// Key used to orbit up. Default is [`KeyCode::KeyI`].
	pub orbit_up_key: Option<KeyCode>,
	/// Key used to orbit down. Default is [`KeyCode::KeyK`].
	pub orbit_down_key: Option<KeyCode>,

	/// Mouse button used to slide camera. Default is [`MouseButton::Right`].
	pub slide_button: Option<MouseButton>,
	/// Key used to slide left. Default is [`KeyCode::KeyS`].
	pub slide_left_key: Option<KeyCode>,
	/// Key used to slide right. Default is [`KeyCode::KeyF`].
	pub slide_right_key: Option<KeyCode>,
	/// Key used to slide up. Default is [`KeyCode::KeyE`].
	pub slide_up_key: Option<KeyCode>,
	/// Key used to slide up. Default is [`KeyCode::KeyD`].
	pub slide_down_key: Option<KeyCode>,
	/// Key used to slide far. Default is [`KeyCode::KeyG`].
	pub slide_far_key: Option<KeyCode>,
	/// Key used to slide near. Default is [`KeyCode::KeyV`].
	pub slide_near_key: Option<KeyCode>,

	/// Key used to scale/zoom in. Default is [`KeyCode::KeyH`].
	pub scale_in_key: Option<KeyCode>,
	/// Key used to scale/zoom out. Default is [`KeyCode::KeyN`].
	pub scale_out_key: Option<KeyCode>,
}

impl TrackballInput {
	/// Maps `esdf`/`gv` to slide operations.
	///
	/// Key | Operation
	/// --- | ---------------------------
	/// `e` | Slides up.
	/// `s` | Slides left.
	/// `d` | Slides down.
	/// `f` | Slides right.
	/// `g` | Slides far (in/forward).
	/// `v` | Slides near (out/backward).
	///
	/// This mapping is symmetric to the `ijkl`/`hn` orbit mapping but less intuitive to gamers
	/// compared with [`Self::map_wasd`].
	pub fn map_esdf(&mut self) {
		self.slide_up_key = Some(KeyCode::KeyE);
		self.slide_down_key = Some(KeyCode::KeyD);
		self.slide_left_key = Some(KeyCode::KeyS);
		self.slide_right_key = Some(KeyCode::KeyF);
		self.slide_far_key = Some(KeyCode::KeyG);
		self.slide_near_key = Some(KeyCode::KeyV);
	}
	/// Maps `wasd`/`Space`/`ControlLeft` to slide operations.
	///
	/// Key           | Operation
	/// ------------- | ---------------------------
	/// `w`           | Slides far (in/forward).
	/// `a`           | Slides left.
	/// `s`           | Slides near (out/backward).
	/// `d`           | Slides right.
	/// `Space`       | Slides up (jump).
	/// `ControlLeft` | Slides down (crouch).
	///
	/// This mapping isn't symmetric to the `ijkl`/`hn` orbit mapping but more intuitive to gamers
	/// compared with [`Self::map_esdf`].
	pub fn map_wasd(&mut self) {
		self.slide_up_key = Some(KeyCode::Space);
		self.slide_down_key = Some(KeyCode::ControlLeft);
		self.slide_left_key = Some(KeyCode::KeyA);
		self.slide_right_key = Some(KeyCode::KeyD);
		self.slide_far_key = Some(KeyCode::KeyW);
		self.slide_near_key = Some(KeyCode::KeyS);
	}
}

impl Default for TrackballInput {
	fn default() -> Self {
		Self {
			velocity: TrackballVelocity::default(),
			wheel_unit: TrackballWheelUnit::default(),

			focus: true,

			gamer_key: Some(KeyCode::KeyM),

			ortho_key: Some(KeyCode::KeyP),

			reset_key: Some(KeyCode::Enter),

			first_key: Some(KeyCode::ShiftLeft),
			first_button: Some(MouseButton::Middle),
			first_left_key: Some(KeyCode::ArrowLeft),
			first_right_key: Some(KeyCode::ArrowRight),
			first_up_key: Some(KeyCode::ArrowUp),
			first_down_key: Some(KeyCode::ArrowDown),

			orbit_button: Some(MouseButton::Left),
			screw_left_key: Some(KeyCode::KeyU),
			screw_right_key: Some(KeyCode::KeyO),
			orbit_left_key: Some(KeyCode::KeyJ),
			orbit_right_key: Some(KeyCode::KeyL),
			orbit_up_key: Some(KeyCode::KeyI),
			orbit_down_key: Some(KeyCode::KeyK),

			slide_button: Some(MouseButton::Right),
			slide_up_key: Some(KeyCode::KeyE),
			slide_down_key: Some(KeyCode::KeyD),
			slide_left_key: Some(KeyCode::KeyS),
			slide_right_key: Some(KeyCode::KeyF),
			slide_far_key: Some(KeyCode::KeyG),
			slide_near_key: Some(KeyCode::KeyV),

			scale_in_key: Some(KeyCode::KeyH),
			scale_out_key: Some(KeyCode::KeyN),
		}
	}
}

/// [`TrackballInput`] setting translating between linear and angular velocity.
#[derive(Debug, Clone, Copy)]
pub enum TrackballVelocity {
	/// Linear velocity.
	Linear(f32),
	/// Angular velocity.
	Angular(f32),
}

impl TrackballVelocity {
	/// Converts to angular velocity where `r` is the radius.
	#[must_use]
	pub fn to_angular(self, r: f32) -> Self {
		match self {
			Self::Angular(w) => Self::Angular(w),
			Self::Linear(v) => Self::Angular(v / r),
		}
	}
	/// Converts to linear velocity where `r` is the radius.
	#[must_use]
	pub fn to_linear(self, r: f32) -> Self {
		match self {
			Self::Angular(w) => Self::Linear(w * r),
			Self::Linear(v) => Self::Linear(v),
		}
	}
	/// Underlying quantity.
	#[must_use]
	pub const fn into_inner(self) -> f32 {
		match self {
			Self::Angular(w) => w,
			Self::Linear(v) => v,
		}
	}
}

impl Default for TrackballVelocity {
	/// Angular velocity of 45 degrees per second.
	///
	/// That is the default fixed vertical field of view per second.
	fn default() -> Self {
		Self::Angular(Fixed::default().into_inner())
	}
}

/// [`TrackballInput`] setting translating wheel units in coherent scale denominators.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum TrackballWheelUnit {
	/// Wheel clicks per turn (cpt).
	///
	/// Defaults to 24 cpt, that are 15°=360°/24 per click which agrees with most devices. Some
	/// devices have 18 cpt instead, that are 20°=360/18 per click.
	///
	/// You can count the cpt of your device by marking the start of your wheel before rotating it a
	/// full turn. Each noticeable step when rotating is called a wheel click, not to be confused
	/// with a middle mouse button click.
	Cpt(f32),
	/// Wheel clicks per second (cps).
	///
	/// Some devices scroll smooth without noticeable steps when rotating. They specify their units
	/// in cps (e.g., 1 000 cps). This unit will be translated to the trackball's angular velocity.
	Cps(f32),
}

impl TrackballWheelUnit {
	/// Coherent [`Scale`] denominator.
	///
	/// [`Scale`]: trackball::Scale
	#[must_use]
	pub fn denominator(self, w: f32) -> f32 {
		match self {
			Self::Cpt(cpt) => cpt,
			Self::Cps(cps) => cps / w,
		}
	}
}

impl Default for TrackballWheelUnit {
	/// 24 cpt, that are 15°=360°/24 per click.
	fn default() -> Self {
		Self::Cpt(24.0)
	}
}
