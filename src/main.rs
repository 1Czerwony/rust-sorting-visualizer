use std::time::Duration;

use sdl2::{render::WindowCanvas, pixels::Color, rect::Rect};

use rand::seq::SliceRandom;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const RECT_WIDTH: u32 = 6;
const FPS: u32 = 200;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // init window
    let window = video_subsystem.window("joguinho bobinho", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem"
    );

    // init canvas
    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas"
    );

    let mut rect_arr = [Rect::new(0, SCREEN_HEIGHT as i32, 10, 10); (SCREEN_HEIGHT/RECT_WIDTH) as usize];

    'running: loop {
        // event handling
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'running,
                _ => {},
            }
        }

        // draw
        shuffle_array(&mut rect_arr);

        counting_sort(&mut canvas, &mut rect_arr)?;

        shuffle_array(&mut rect_arr);

        comb_sort(&mut canvas, &mut rect_arr)?;

        shuffle_array(&mut rect_arr);

        if cocktail_sort(&mut canvas, &mut rect_arr)? {
            break 'running;
        }

    }

    Ok(())
}

fn max_height(rect_arr: &[Rect]) -> u32 {
    let mut max = rect_arr[0].height();
    for i in 1..rect_arr.len() {
        if rect_arr[i].height() > max {
            max = rect_arr[i].height();
        }
    }
    max
}

fn shuffle_array(rect_arr: &mut [Rect]) {
    let mut rng = rand::thread_rng();
    let mut height_vec: Vec<u32> = (RECT_WIDTH..=SCREEN_HEIGHT).step_by(RECT_WIDTH as usize).collect();
    height_vec.shuffle(&mut rng);

    let mut x = 0;
    for i in 0..((SCREEN_HEIGHT/RECT_WIDTH) as usize){
        rect_arr[i].set_x(x);
        rect_arr[i].set_height(*height_vec.get(i).unwrap());
        rect_arr[i].set_width(RECT_WIDTH);
        rect_arr[i].set_bottom(SCREEN_HEIGHT as i32);
        x += 8;
    }
}

fn draw_vec(canvas: &mut WindowCanvas, rect_arr: &[Rect], highlighted: &[usize]) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    for i in 0..((SCREEN_HEIGHT/RECT_WIDTH) as usize) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        if highlighted.contains(&i) {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
        }
        canvas.fill_rect(rect_arr[i])?;
    }
    canvas.present();
    Ok(())
}

fn counting_sort(canvas: &mut WindowCanvas, rect_arr: &mut [Rect]) -> Result<bool, String> {

    let max: u32 = max_height(rect_arr);
 
    let mut count = vec![0; (max + 1) as usize];

    for i in 0..rect_arr.len() as usize {
        count[rect_arr[i].height() as usize] += 1; 
    }

    for i in 1..(max+1) as usize {
        count[i] += count[i-1];
    }

    let mut output = vec![0; rect_arr.len()];
    for i in (0..rect_arr.len() as usize).rev() {
        output[count[(rect_arr[i].height() as usize) - 1] as usize] = rect_arr[i].height();
        count[rect_arr[i].height() as usize] -= 1;

        draw_vec(canvas, rect_arr, &[i])?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    for i in 0..rect_arr.len() {
        rect_arr[i].set_height(output[i]);
        rect_arr[i].set_bottom(SCREEN_HEIGHT as i32);
        draw_vec(canvas, rect_arr, &[i])?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }

    Ok(true)
}

fn comb_sort(canvas: &mut WindowCanvas, rect_arr: &mut [Rect]) -> Result<bool, String> {
    let mut gap = rect_arr.len();

    let mut swapped = 1;

    while gap != 1 || swapped == 1 {
        gap = (gap * 10) / 13;
        if gap < 1 {
            gap = 1;
        }
        
        swapped = 0;

        for i in 0..(rect_arr.len() - gap) {
            draw_vec(canvas, rect_arr, &[i, i + gap])?;
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));

            if rect_arr[i].height() > rect_arr[i + gap].height() {
                let aux = rect_arr[i].height();
                rect_arr[i].set_height(rect_arr[i + gap].height());
                rect_arr[i + gap].set_height(aux);
                rect_arr[i].set_bottom(SCREEN_HEIGHT as i32);
                rect_arr[i + gap].set_bottom(SCREEN_HEIGHT as i32);

                swapped = 1;
            }
        }
    }

    Ok(true)
}

fn cocktail_sort(canvas: &mut WindowCanvas, rect_arr: &mut [Rect]) -> Result<bool, String> {
    let mut swapped = true;
    let mut start = 0;
    let mut end = rect_arr.len() - 1;

    while swapped {
        swapped = false;

        for i in 0..end {
            draw_vec(canvas, rect_arr, &[i, i + 1])?;
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));

            if rect_arr[i].height() > rect_arr[i + 1].height() {
                let aux = rect_arr[i].height();
                rect_arr[i].set_height(rect_arr[i + 1].height());
                rect_arr[i + 1].set_height(aux);
                rect_arr[i].set_bottom(SCREEN_HEIGHT as i32);
                rect_arr[i + 1].set_bottom(SCREEN_HEIGHT as i32);

                swapped = true;
            }
        }

        if !swapped {break;}

        swapped = false;   

        end -= 1;

        for i in (start..end).rev() {
            draw_vec(canvas, rect_arr, &[i, i + 1])?;
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FPS));

            if rect_arr[i].height() > rect_arr[i + 1].height() {
                let aux = rect_arr[i].height();
                rect_arr[i].set_height(rect_arr[i + 1].height());
                rect_arr[i + 1].set_height(aux);
                rect_arr[i].set_bottom(SCREEN_HEIGHT as i32);
                rect_arr[i + 1].set_bottom(SCREEN_HEIGHT as i32);

                swapped = true;
            }
        }

        start += 1;
    }

    Ok(true)
}