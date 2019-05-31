use evcxr_displayers::Displayable;
use nalgebra as na;

pub fn main() {
    let m = na::Matrix3x4::new(11, 12, 13, 14,
                    21, 22, 23, 24,
                    31, 32, 33, 34);
    m.display().unwrap();
}
