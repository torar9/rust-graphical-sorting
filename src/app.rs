use piston::input::*;
use piston_window::*;
use std::{time::Duration, thread};
use opengl_graphics::{ GlGraphics, OpenGL };
use ::rand::prelude::*;

use color::Color;
use object::Object;
use bubblesort::*;


pub struct App
{
    gl: GlGraphics,
    amount: usize,//Amount of elements in sort array
    list: Vec<Object<u32>>,//Vector that contains elements to sort
    push: bool,//Is used to Start/Stop sorting
    is_rendered: bool,//Prevents multiple execution of do_cycle in case of thread race
    selected: usize,
    speed: u64,//Amount of time to sleep thread in ms, faster -> decrease; value slower -> increase value
    sort: BubbleSort,
}

impl App
{
    pub fn new(ogl: OpenGL, amount: usize) -> App
    {
        let mut it = App
        {
            gl: GlGraphics::new(ogl),
            amount: amount,
            list: Vec::with_capacity(amount),
            push: false,//Wait for user input to start sorting
            is_rendered: false,
            sort: BubbleSort::new(),
            speed: 50,//Sleep thread for 50 ms by default
            selected: 0,
        };

        for _ in 0..amount
        {
            let mut rng = thread_rng();
            let num = rng.gen_range(1, 300);
            it.list.push(Object::new(Color::new(1.0, 1.0, 1.0, 1.0), num));
        }

        it
    }

    pub fn render(&mut self, args: &RenderArgs)
    {
        self.gl.clear_stencil(0);
        self.gl.clear_color([0.0, 0.0, 0.0, 1.0]);

        let selected = self.selected;
        let amount = self.amount;
        for (i, e) in self.list.iter_mut().enumerate()
        {
            /*Each element in list gets its own object with its width
                and height based on screen size and its value*/
            let w_per_item = (args.width as f64) / amount as f64;
            let x = w_per_item as f64 * i as f64;//x position of element
            let height = e.value as f64;
            let y = args.height as f64 - height;//y position of element

            e.set_xy(x, y);
            e.width = w_per_item;
            e.height = height;
            if i == selected
            {
                e.is_selected = true;
            }

            e.draw(&mut self.gl, args);
            e.reset_options();//Resets object's properties such as is_active to prevent multiple is_active elements
        }
        self.is_rendered = true;
    }

    fn bubble_sort(&mut self)
    {
        self.is_rendered = false;

        match self.sort.do_cycle(&mut self.list)
        {
            Some(()) => { self.push = false; },
            None => {}
        }
    }

    pub fn update(&mut self, upd: &UpdateArgs)
    {
        if self.push && self.is_rendered//Prevents multiple execution of do_cycle in case of thread race
        {
            thread::sleep(Duration::from_millis(self.speed));
            self.bubble_sort();
        }
    }

    pub fn on_input_release(&mut self, inp: &Button)
    {
        match inp
        {
            _ => {}
        }
    }

    pub fn on_input(&mut self, inp: &Button)
    {
        match inp
        {
            Button::Keyboard(Key::Left) =>
            {
                println!("pressed: Left");
                if self.selected > 0
                {
                    self.selected -= 1;
                }
                else
                {
                    self.selected = self.amount - 1;
                }
            }
            Button::Keyboard(Key::Right) =>
            {
                println!("pressed: Right");
                if self.selected < self.amount - 1
                {
                    self.selected += 1;
                }
                else
                {
                    self.selected = 0;
                }
            }
            Button::Keyboard(Key::Q) =>
            {
                println!("pressed: Q");//Start/Stop button
                self.push = !self.push;
            }
            Button::Keyboard(Key::Down) =>
            {
                println!("pressed: Down, current speed: {}", self.speed);//Decrease speed by increasing thread sleep time
                self.speed += 5;
            }
            Button::Keyboard(Key::Up) =>
            {
                println!("pressed: Up, current speed: {}", self.speed);//Increase speed by decreasing thread sleep time
                if self.speed != 0
                {
                    self.speed -= 5;
                }
            }
            _ => {}
        }
    }
}
