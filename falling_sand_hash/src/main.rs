use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap;
fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    //let mut points: Vec<Vec<i32>> = Vec::new();
    // Create the window
    let height: u32 = 640;
    let width: u32 = 640;
    let window = video_subsystem
        .window("My SDL2 Window", width, height) // Adjust size as desired
        .position_centered()
        .build()
        .unwrap();
    //let mut points = vec![vec![0; width as usize]; height as usize];
    let mut points: HashMap<Point, i32> = HashMap::new();
    points.insert(Point::new(0,0), 0);
   /* for i in 0..height {
        for j in 0..width {
            points.insert(Point::new(j as i32, i as i32), 0);
        }
    }*/
    // Create a renderer to draw on the window
    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    let mut painting: bool = false;
    let mut mouseX: i32 = 0;
    let mut mouseY: i32 = 0;
    // Set background color
    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background
    canvas.set_draw_color(Color::RGB(250, 0, 0)); // Red background
                                                  // Event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        iteration(&mut points, &mut canvas);
        draw(points.clone(), &mut canvas);
        canvas.present();
        if painting {
            new_point(mouseX as usize, mouseY as usize, &mut points);
        }
        // std::thread::sleep(Duration::new(0, 5000000));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::MouseMotion { x, y, .. } => {
                    if y < height as i32 && x < width as i32 && y > 0 && x > 0 {
                        mouseX = x;
                        mouseY = y;
                    }
                }

                Event::MouseButtonDown { .. } => {
                    painting = true;
                }
                Event::MouseButtonUp { .. } => {
                    painting = false;
                }
                _ => {}
            }
        }
    }
}

fn draw(points: HashMap<Point, i32>, canvas: &mut Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background
    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 0, 180)); // Red background
    for (key, value) in points {
        if value == 1{
        canvas.draw_point(key);
        }
    }
}
fn new_point(x: usize, y: usize, points: &mut HashMap<Point, i32>) {
    points.insert(Point::new(x as i32, y as i32), 1);
}

fn iteration(points: &mut HashMap<Point, i32>, canvas: &mut Canvas<sdl2::video::Window>) {
    let mut pp: Point = Point::new(0, 0);

    for x in (0..640).rev() {
        for y in (0..640).rev() {
            pp.x = x;
            pp.y = y;
            if points.get(&pp).is_none()  {continue;}
            if *points.get(&pp).unwrap() == 1 {
            }
        }
    }
}
