pub mod xtc;
use crate::trajectory_data::xtc::XtcData;

pub fn trajectory_loader(topology_file: &str, trajectory_file: &str) -> Box<dyn TrajectoryData> {
    let extension = std::path::Path::new(trajectory_file)
        .extension()
        .and_then(|ext| ext.to_str());
    if let Some(extension) = extension {
        match extension {
            "xtc" => Box::new(XtcData::load(topology_file, trajectory_file)),
            _ => {
                unimplemented!("This extension is not supported.");
            }
        }
    } else {
        panic!("trajectory_file: {} has no extension.", trajectory_file);
    }
}

pub trait TrajectoryData: Sync + Send {
    fn frames(&self) -> &Vec<Frame>;

    fn frame(&self, frame_id: usize) -> &Frame {
        &self.frames()[frame_id]
    }

    fn load(topology_file: &str, trajectory_file: &str) -> Self
    where
        Self: Sized;
}

#[derive(Clone, Debug)]
pub struct Frame {
    frame_id: usize,

    // contains all atom's position for each frame
    positions: Vec<[f32; 3]>,
}

impl Frame {
    pub fn new(frame_id: usize, positions: Vec<[f32; 3]>) -> Frame {
        Frame {
            frame_id,
            positions,
        }
    }

    pub fn frame_id(&self) -> usize {
        self.frame_id
    }

    pub fn positions(&self) -> &Vec<[f32; 3]> {
        &self.positions
    }
}
