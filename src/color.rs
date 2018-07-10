pub static RED:   &'static [f64] = &[1.0, 0.0, 0.0, 1.0];
pub static GREEN: &'static [f64] = &[0.0, 1.0, 0.0, 1.0];
pub static BLUE:  &'static [f64] = &[0.0, 0.0, 1.0, 1.0];

pub struct Color
{
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color
{
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color
    {
        Color{r: r, g: g, b: b, a: a}
    }
}
