pub mod xtc;


pub trait TrajectoryData {
    fn frame(&self) -> Vec<Frame>;
}


#[derive(Clone, Debug)]
pub struct Frame {
    id: usize,
    position_history: Vec<[f32; 3]>,
}


