use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::collections::HashMap; // 0.8.0
fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    //let mut points: Vec<Vec<i32>> = Vec::new();
    // Create the window
    let height: u32 = 500;
    let width: u32 = 500;
    let window = video_subsystem
        .window("My SDL2 Window", width, height) // Adjust size as desired
        .position_centered()
        .build()
        .unwrap();
    //let mut points = vec![vec![0; width as usize]; height as usize];
    //  let mut points: HashMap<Point, i32> = HashMap::new();
    let mut points: HashMap<(i32, i32), i32> = HashMap::new();
    /* for i in 0..height {
        for j in 0..width {
            points.insert(Point::new(j as i32, i as i32), 0);
        }
    }*/
    // Create a renderer to draw on the window
    let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
    let mut painting: bool = false;
    let mut deleting: bool = false;
    let mut mouse_x: i32 = 0;
    let mut mouse_y: i32 = 0;
    let mut pen_size: i32 = 1;
    // Set background color
                                                  // Event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        if points.len() != 0 {
            iteration(&mut points);
            draw(points.clone(), &mut canvas);
        }
        canvas.present();

        if painting  && !deleting {
            new_point(mouse_x, mouse_y, &mut points, 1, pen_size);
        }
        else if painting && deleting{

            delete_point(mouse_x, mouse_y, &mut points,pen_size);
        }
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
                        mouse_x = x;
                        mouse_y = y;
                    }
                }

                Event::MouseButtonDown { .. } => {
                    painting = true;
                }
                Event::MouseButtonUp { .. } => {
                    painting = false;
                }
                Event::KeyDown {keycode: Some(Keycode::D) , .. } => {
                    deleting = true;
                }
                Event::KeyDown {keycode: Some(Keycode::P) , .. } => {
                    deleting = false;
                }Event::KeyDown {keycode: Some(Keycode::Up) , .. } => {
                    pen_size += 2;
                }
                Event::KeyDown {keycode: Some(Keycode::Down) , .. } => {
                    pen_size -= 2;
                }

                _ => {}
            }
        }
    }
}

fn draw(points: HashMap<(i32, i32), i32>, canvas: &mut Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0)); // Black background
    canvas.clear();
    canvas.set_draw_color(Color::RGB(180, 0, 180)); // Red background
    for key in points.keys() {
        let _ = canvas.draw_point(Point::new(key.0, key.1));
    }
}
fn new_point(x: i32, y: i32, points: &mut HashMap<(i32, i32), i32>, val: i32, pen_size:i32) {
    for i in 0..pen_size {
    points.insert((x+i, y), val);
    points.insert((x-i, y), val);
    points.insert((x, y+i), val);
    points.insert((x, y-i), val);
    points.insert((x+i, y+i), val);
    points.insert((x-i, y-i), val);
    points.insert((x-i, y+i), val);
    points.insert((x+i, y-i), val);
    }
}
fn delete_point(x: i32, y: i32, points: &mut HashMap<(i32, i32), i32>, pen_size:i32) {
     for i in 0..pen_size {
    points.remove(&(x+i, y));
    points.remove(&(x-i, y));
    points.remove(&(x, y+i));
    points.remove(&(x, y-i));
    points.remove(&(x+i, y+i));
    points.remove(&(x-i, y-i));
    points.remove(&(x-i, y+i));
    points.remove(&(x+i, y-i));
    }
}
fn iteration(points: &mut HashMap<(i32, i32), i32>) {
    let new_points = points.clone();
    for (key, _value) in new_points {
        let num = rand::thread_rng().gen_range(0..1);
        if key.1 >= 499 {
            continue;
        }
        if !points.contains_key(&(key.0, key.1 + 1)) {
            delete_point(key.0, key.1, points,1);
            new_point(key.0, key.1 + 1, points, 1,1);
        } else {
            if num == 0 {
                if !points.contains_key(&(key.0 + 1, key.1 + 1)) {
                    delete_point(key.0, key.1, points,1);
                    new_point(key.0 + 1, key.1 + 1, points, 1,1);
                } else if !points.contains_key(&(key.0 - 1, key.1 + 1)) {
                    delete_point(key.0, key.1, points,1);
                    new_point(key.0 - 1, key.1 + 1, points, 1,1);
                }
            } else {
                if !points.contains_key(&(key.0 - 1, key.1 + 1)) {
                    delete_point(key.0, key.1, points,1);
                    new_point(key.0 - 1, key.1 + 1, points, 1,1);
                } else if !points.contains_key(&(key.0 + 1, key.1 + 1)) {
                    delete_point(key.0, key.1, points,1);
                    new_point(key.0 + 1, key.1 + 1, points, 1,1);
                }
            }
        }
    }
}
