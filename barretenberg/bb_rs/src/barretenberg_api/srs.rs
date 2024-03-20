use super::{bindgen, traits::SerializeBuffer};

pub unsafe fn init_srs(points_buf: &[u8], num_points: u32, g2_point_buf: &[u8]) {
    bindgen::srs_init_srs(
        points_buf.to_buffer().as_slice().as_ptr(),
        num_points.to_be_bytes().as_ptr() as *const u32,
        g2_point_buf.to_buffer().as_slice().as_ptr(),
    );
}
