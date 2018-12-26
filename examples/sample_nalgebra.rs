use evcxr_displayers::EvcxrDisplay;
use nalgebra as na;

pub fn main() {
    let m = na::Matrix3x4::new(11, 12, 13, 14,
                    21, 22, 23, 24,
                    31, 32, 33, 34);
    m.evcxr_display();
}

#[test]
fn test_sample_nalgebra() {
    main()
}
