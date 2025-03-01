use drm_fourcc::DrmFourcc;

pub struct Frame {
    pub id: usize,
    pub buffer: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub stride: usize,
    pub pixel_format: DrmFourcc,
}