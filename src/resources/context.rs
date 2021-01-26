#[derive(Clone, Copy, Default)]
pub struct Context {
    pub map_width: f32,
    pub bg_width: f32,
    pub bg_height: f32,
    pub scale: f32,
    pub x_correction: f32,
    pub y_correction: f32,
}

impl Context {
    pub fn new() -> Self {
        Context {
            map_width: 2304.,
            bg_width: 576.,
            bg_height: 192.,
            x_correction: -(1158. / 2. - 576.),
            y_correction: -(96.),
            scale: 1.,
        }
    }
}
