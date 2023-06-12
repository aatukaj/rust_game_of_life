use macroquad::prelude::*;

#[derive(Clone, Debug)]
enum CellState {
    Alive,
    Dead,
}
const SCALE: f32 = 4.0;

fn in_bounds(x: i32, y: i32, max_x: usize, max_y: usize) -> bool {
    0 <= x && x < max_x as i32 && 0 <= y && y < max_y as i32
}

#[macroquad::main("gameoflife")]
async fn main() {
    let height: usize = (screen_height() / SCALE) as usize;
    let width: usize = (screen_width() / screen_height() * height as f32) as usize + 1;

    let mut img = Image::gen_image_color(width as u16, height as u16, BLACK);
    let tex = Texture2D::from_image(&img);
    tex.set_filter(FilterMode::Nearest);

    println!("{}", img.width());

    let mut cells = vec![CellState::Dead; width * height];

    for cell in cells.iter_mut() {
        if rand::gen_range(0, 5) == 0 {
            *cell = CellState::Alive;
        }
    }
    let mut last_time = get_time();

    let neighbour_positions: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    loop {
        println!("fps:{}", get_fps());
        draw_texture_ex(
            tex,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2 {
                    x: screen_width(),
                    y: screen_height(),
                }),
                ..Default::default()
            },
        );
        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            let x_i = (x / screen_width() * width as f32) as i32;
            let y_i = (y / screen_height() * height as f32) as i32;
            for x_offset in 0..4 {
                for y_offset in 0..4 {
                    if in_bounds(x_i + x_offset, y_i + y_offset, width, height) {
                        cells[(x_i + x_offset + (y_i + y_offset)* width as i32) as usize] = CellState::Alive;
                    }
                }
            }
            
        }

        if get_time() - last_time > 0.1 {
            let mut new_cells: Vec<CellState> = Vec::new();
            last_time = get_time();
            for y in 0..height {
                for x in 0..width {
                    let mut neighbours = 0;
                    let cell = &cells[x + y * width];

                    for pos in neighbour_positions.iter() {
                        let new_x = x as i32 + pos.0;
                        let new_y = y as i32 + pos.1;
                        if in_bounds(new_x, new_y, width, height) {
                            match cells[new_x as usize + new_y as usize * width] {
                                CellState::Alive => neighbours += 1,
                                CellState::Dead => {}
                            }
                        }
                    }
                    new_cells.push(match (neighbours, cell) {
                        (2 | 3, CellState::Alive) => CellState::Alive,
                        (3, CellState::Dead) => CellState::Alive,
                        _ => CellState::Dead,
                    });

                    img.set_pixel(
                        x as u32,
                        y as u32,
                        match cell {
                            CellState::Alive => RED,
                            CellState::Dead => BLACK,
                        },
                    );
                }
            }
            cells = new_cells;
            tex.update(&img);
        }
        next_frame().await
    }
}
