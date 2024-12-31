use bevy::prelude::*;

use super::{TrackballCamera, TrackballEvent, TrackballSetup};

const LOOPS: usize = 100;

#[allow(clippy::needless_pass_by_value)]
#[allow(clippy::too_many_arguments)]
pub fn trackball_constellation(
	mut trackball_events: EventReader<TrackballEvent>,
	mut cameras: Query<(Entity, &mut TrackballCamera)>,
) {
	for trackball_event in trackball_events.read() {
		let mut min_delta = trackball_event.delta;
		let mut loops = 0;
		loop {
			let mut bound = false;
			for (group, mut trackball) in &mut cameras {
				if group != trackball_event.group
					&& !trackball.group.contains_key(&trackball_event.group)
				{
					continue;
				}
				if let Some(clamp) = &trackball.clamp {
					if let Some((delta, loops)) =
						clamp.compute(&trackball.frame, &trackball.scope, &min_delta)
					{
						if loops == clamp.loops() {
							warn!("Using partial clamp after {loops} loops (entity {group:?})");
						} else {
							debug!("Found camera clamp after {loops} loops (entity {group:?})");
						}
						if trackball
							.group
							.get(&trackball_event.group)
							.copied()
							.unwrap_or(true)
						{
							bound = true;
							min_delta = delta;
							break;
						}
						trackball.delta = Some(delta);
					}
				}
			}
			if bound {
				if loops == LOOPS {
					warn!("Using partial clamp after {LOOPS} loops");
					break;
				}
				loops += 1;
			} else {
				debug!("Found common clamp after {loops} loops");
				break;
			}
		}
		for (group, mut trackball) in &mut cameras {
			if group != trackball_event.group
				&& !trackball.group.contains_key(&trackball_event.group)
			{
				continue;
			}
			let delta = if trackball
				.group
				.get(&trackball_event.group)
				.copied()
				.unwrap_or(true)
			{
				min_delta
			} else {
				trackball.delta.take().unwrap_or(trackball_event.delta)
			};
			trackball.frame = delta.transform(&trackball.frame);
			trackball.frame.renormalize();
			if let Some(setup) = trackball_event.setup {
				match setup {
					TrackballSetup::Ortho(ortho) => {
						let ortho = ortho.unwrap_or_else(|| !trackball.scope.ortho());
						trackball.scope.set_ortho(ortho);
					}
					TrackballSetup::Reset => {
						trackball.frame = trackball.reset;
					}
				}
			}
		}
	}
}
