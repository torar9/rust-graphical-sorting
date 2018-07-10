extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston_window;
extern crate rand;

use piston_window::*;
use piston::{event_loop::*, input::*, window::WindowSettings};
use opengl_graphics::{ OpenGL };

mod app;
mod color;
mod point;
mod object;

pub use app::*;

fn main()
{
    let opengl = OpenGL::V4_3;

    let mut window: PistonWindow = WindowSettings::new("Sorting", [800, 600])
        .exit_on_esc(true)
        .opengl(opengl)
        .vsync(true)
        .build()
        .unwrap();

    let mut app: App = App::new(opengl, 50);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window)
    {
        if let Some(ren) = e.render_args()
        {
            app.render(&ren);
        }

        if let Some(upd) = e.update_args()
        {
            app.update(&upd);
        }

        if let Some(press) = e.press_args()
        {
            app.on_input(&press);
        }

        if let Some(rele) = e.release_args()
        {
            app.on_input_release(&rele);
        }
    }
}
