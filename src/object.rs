use opengl_graphics::{ GlGraphics };
use piston_window::RenderArgs;
use graphics::rectangle;

use point::Point;
use color::Color;

pub struct Object<T>
{
    col: Color,
    pos: Point<f64>,
    pub width: f64,
    pub height: f64,
    pub value: T,
    pub is_selected: bool,
    pub is_active: bool,
}

impl<T> Object<T>
{
    pub fn new(col: Color, val: T) -> Object<T>
    {
        Object
        {
            col: col,
            pos: Point::new(0.0, 0.0),
            width: 0.0,
            height: 0.0,
            value: val,
            is_selected: false,
            is_active: false,
        }
    }

    pub fn draw(&mut self, gl: &mut GlGraphics, args: &RenderArgs)
    {
        let rect = [self.pos.x, self.pos.y, self.width, self.height];

        gl.draw(args.viewport(), |c, gl|
            {
                if self.is_selected
                {
                    rectangle([0.0, 1.0, 0.0, 1.0], rect, c.transform, gl);
                }
                else if self.is_active
                {
                    rectangle([0.0, 0.0, 1.0, 1.0], rect, c.transform, gl);
                }
                else
                {
                    rectangle([1.0, 1.0, 1.0, 1.0], rect, c.transform, gl);
                }
            });
    }

    pub fn set_xy(&mut self, x: f64, y:f64)
    {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn reset_options(&mut self)
    {
        self.is_active = false;
        self.is_selected = false;
    }
}
