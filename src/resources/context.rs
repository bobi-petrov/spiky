#[derive(Clone, Copy, Default)]
pub struct Context {
    pub map_width: f32,
    pub bg_width: f32,
    pub bg_height: f32,
    pub x_correction: f32,
    pub y_correction: f32,
    pub bg_z_translation: f32,
    pub truss_z_translation: f32,
    pub platform_z_translation: f32,
    pub scale: f32,
}

impl Context {
    pub fn new() -> Self {
        Context {
            map_width: 2304.,
            bg_width: 576.,
            bg_height: 192.,
            x_correction: -(1200. / 2. - 576.), // - (screen_width / 2. - background_width)
            y_correction: -96.,                // (background_height / 2.) * -1.
            bg_z_translation: -50.,
            truss_z_translation: -40.,
            platform_z_translation: -10.,
            scale: 2.,
        }
    }
}
