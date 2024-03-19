use super::{srs_init_srs, traits::SerializeBuffer};

pub unsafe fn api_init_srs(points_buf: &[u8], num_points: u32, g2_point_buf: &[u8]) {
    srs_init_srs(
        points_buf.to_buffer().as_slice().as_ptr(),
        num_points.to_be_bytes().as_ptr() as *const u32,
        g2_point_buf.to_buffer().as_slice().as_ptr(),
    );
}
