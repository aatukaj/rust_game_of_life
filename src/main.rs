use macroquad::prelude::*;
use rayon::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
enum CellState {
    Alive,
    Dead,
}
enum BrushShape {
    Square,
    CheckerBoard,
    Circle,
    Diamond,
}
const BRUSHSHAPES: [BrushShape; 4] = [
    BrushShape::Square,
    BrushShape::CheckerBoard,
    BrushShape::Circle,
    BrushShape::Diamond,
];

const SCALE: f32 = 2.0;

const NEIGHBOUR_POSITIONS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn in_bounds(x: i32, y: i32, max_x: usize, max_y: usize) -> bool {
    0 <= x && x < max_x as i32 && 0 <= y && y < max_y as i32
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("GOL"),
        window_width: 1600,
        window_height: 900,
        fullscreen: true,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let height: usize = (screen_height() / SCALE) as usize;
    let width: usize = (screen_width() / screen_height() * height as f32) as usize + 1;

    let mut brush_radius = 20;
    let mut brush_index: usize = 0;

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

        let m_wheel = mouse_wheel().1;
        if m_wheel != 0.0 {
            brush_radius += m_wheel.signum() as i32
        }

        let m_left = is_mouse_button_down(MouseButton::Left);
        let m_right = is_mouse_button_down(MouseButton::Right);

        if m_left || m_right {
            let new_state = if m_left {
                CellState::Alive
            } else {
                CellState::Dead
            };

            let (x, y) = mouse_position();
            let x_i = (x / screen_width() * width as f32) as i32;
            let y_i = (y / screen_height() * height as f32) as i32;
            for x_offset in -brush_radius..=brush_radius {
                for y_offset in -brush_radius..=brush_radius {
                    let (x, y) = (x_i + x_offset, y_i + y_offset);
                    if in_bounds(x, y, width, height) {
                        match BRUSHSHAPES[brush_index] {
                            BrushShape::Square => {
                                cells[(x + y * width as i32) as usize] = new_state.clone()
                            }
                            BrushShape::CheckerBoard => {
                                if x % 2 != y % 2 {
                                    cells[(x + y * width as i32) as usize] = new_state.clone()
                                }
                            }
                            BrushShape::Circle => {
                                let (dx, dy) = (x - x_i, y - y_i);
                                let dist = dx * dx + dy * dy;
                                if dist < brush_radius * brush_radius && x % 2 != y % 2 {
                                    cells[(x + y * width as i32) as usize] = new_state.clone()
                                }
                            }
                            BrushShape::Diamond => {
                                let (dx, dy) = (x - x_i, y - y_i);
                                if dx.abs() + dy.abs() <= brush_radius {
                                    cells[(x + y * width as i32) as usize] = new_state.clone()
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        if is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::LeftAlt) {
            brush_index = (brush_index + 1) % BRUSHSHAPES.len();
        }
        if is_key_down(KeyCode::Space) {
            for cell in cells.iter_mut() {
                *cell = CellState::Dead
            }
        }

        //update the grid every 0.05 seconds
        if get_time() - last_time > 0.05 {
            last_time = get_time();
            let new_cells: Vec<CellState> = cells
                .par_iter()
                .enumerate()
                .map(|(i, cell_state)| {
                    let x = i % width;
                    let y = i / width;

                    let mut neighbours = 0;

                    for pos in NEIGHBOUR_POSITIONS {
                        let new_x = x as i32 + pos.0;
                        let new_y = y as i32 + pos.1;
                        if in_bounds(new_x, new_y, width, height) {
                            if cells[new_x as usize + new_y as usize * width] == CellState::Alive {
                                neighbours += 1
                            }
                        }
                    }

                    match (neighbours, cell_state) {
                        (2 | 3, CellState::Alive) => CellState::Alive,
                        (3, CellState::Dead) => CellState::Alive,
                        _ => CellState::Dead,
                    }
                })
                .collect();

            //biggest bottleneck
            for (i, cell_state) in cells.iter().enumerate() {
                let x = (i % width) as u32;
                let y = (i / width) as u32;
                img.set_pixel(
                    x,
                    y,
                    match cell_state {
                        CellState::Alive => RED,
                        CellState::Dead => {
                            let red = img.get_pixel(x, y).r;
                            Color::new(red * 0.7, 0.0, 0.0, 1.0)
                        }
                    },
                );
            }

            cells = new_cells;
            tex.update(&img);
        }
        next_frame().await
    }
}
