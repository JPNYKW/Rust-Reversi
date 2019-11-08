use std::f64;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

const GRID_SIZE: f64 = 50.0;
const WINDOW_WIDTH: f64 = 640.0;
const WINDOW_HEIGHT: f64 = 640.0;

pub struct App {
    gl: GlGraphics
}

impl App {
    fn render(&mut self, args: &RenderArgs, _board: [[usize; 8]; 8]) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.14, 0.14, 0.14, 1.0];
        const WHITE: [f32; 4] = [0.85, 0.85, 0.85, 1.0];
        const GREEN: [f32; 4] = [0.03, 0.51, 0.23, 1.0];

        let square = rectangle::square(
            -GRID_SIZE * 4.0,
            -GRID_SIZE * 4.0,
            GRID_SIZE * 8.0
        );

        let stone = rectangle::square(0.0, 0.0, 20.0);

        let (x, y) = (args.window_size[0] / 2.0,
                      args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |_c, gl| {
            clear(WHITE, gl);

            // GRID
            let mut _x = 0.0;
            let mut _y = GRID_SIZE * -4.0 + GRID_SIZE;
            let transform = _c.transform.trans(x, y);

            let dx = [-GRID_SIZE, GRID_SIZE, GRID_SIZE, GRID_SIZE, GRID_SIZE, -GRID_SIZE, -GRID_SIZE, -GRID_SIZE];
            let dy = [GRID_SIZE, GRID_SIZE, GRID_SIZE, -GRID_SIZE, -GRID_SIZE, -GRID_SIZE, -GRID_SIZE, GRID_SIZE];

            // BACKGROUND
            rectangle(GREEN, square, transform, gl);

            for _i in 0..7 {
                _x = GRID_SIZE * -4.0 + GRID_SIZE;
                for _j in 0..7 {
                    for _k in 0..4 {
                        line(BLACK, 2.0, [
                            _x + dx[_k * 2], _y + dy[_k * 2],
                            _x + dx[_k * 2 + 1], _y + dy[_k * 2 + 1]
                        ], transform, gl);
                    }
                    _x += GRID_SIZE;
                }
                _y += GRID_SIZE;
            }

            // STONES
            // circle_arc(BLACK, stone, transform, gl);
            // circle_arc(BLACK, 10.0, 0.0, f64::consts::PI*1.9999, stone, transform, gl);
            _y = GRID_SIZE * -4.0 + GRID_SIZE;
            for _i in 0..8 {
                _x = GRID_SIZE * -4.0 + GRID_SIZE;
                for _j in 0..8 {
                    if _board[_i][_j] > 0 {
                        let trans = _c.transform.trans(
                            _x + GRID_SIZE * 6.0 - 15.0,
                            _y + GRID_SIZE * 6.0 - 15.0
                        );

                        circle_arc(
                            if _board[_i][_j] == 1 { WHITE } else { BLACK },
                            10.0, 0.0, f64::consts::PI * 1.9999, stone, trans, gl
                        );
                    }
                    _x += GRID_SIZE;
                }
                _y += GRID_SIZE;
            }
        });
    }
}

fn count_stones(_board: [[usize; 8]; 8]) -> [usize; 2] {
    let mut stones: [usize; 2] = [0; 2];

    for i in 0..8 {
        for j in 0..8 {
            if _board[i][j] > 0 {
                stones[_board[i][j] - 1] += 1;
            }
        }
    }

    return stones;
}

fn get_reversed_board(_x: usize, _y: usize, _stone: usize, _board: [[usize; 8]; 8]) -> [[usize; 8]; 8] {
    let opponent_stone = if _stone == 1 { 2 } else { 1 };
    let dx: [i32; 8] = [0, -1, -1, -1,  0,  1, 1, 1];
    let dy: [i32; 8] = [1,  1,  0, -1, -1, -1, 0, 1];
    let mut new_board = _board;

    for id in 0..8 {
        let mut _x_pos = _x as i32 + dx[id];
        let mut _y_pos = _y as i32 + dy[id];
        if _x_pos < 0 || _x_pos > 7 || _y_pos < 0 || _y_pos > 7 { continue; }

        if _board[_y_pos as usize][_x_pos as usize] == opponent_stone {
            let mut flag = true;
            let mut count_max = 0;

            loop {
                count_max += 1;
                _x_pos += dx[id];
                _y_pos += dy[id];

                if _x_pos < 0 || _x_pos > 7 || _y_pos < 0 || _y_pos > 7 || _board[_y_pos as usize][_x_pos as usize] == 0 {
                    flag = false;
                    break;
                } else if _board[_y_pos as usize][_x_pos as usize] == _stone {
                    break;
                }
            }

            if flag {
                _x_pos = _x as i32;
                _y_pos = _y as i32;

                for _i in 0..count_max {
                    _x_pos += dx[id];
                    _y_pos += dy[id];
                    new_board[_y_pos as usize][_x_pos as usize] = _stone;
                }
            }
        }
    }

    return new_board;
}

fn alert() {
    println!("\n==================================================");
    println!("You can't put there.");
    println!("==================================================");
}

fn main() {
    let mut id_x: usize = 0;
    let mut id_y: usize = 0;
    let mut is_black_turn = true;
    let mut board: [[usize; 8]; 8] = [[0; 8]; 8];
    board[3][3] = 1;
    board[3][4] = 2;
    board[4][3] = 2;
    board[4][4] = 1;

    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new(
            "Reversi v1.1",
            [WINDOW_WIDTH, WINDOW_HEIGHT]
        )
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
    let mut cursor = [0.0, 0.0];

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, board);
        }

        if let Some(Button::Mouse(button)) = e.press_args() {
            if button == piston::MouseButton::Left {
                if board[id_y][id_x] > 0 {
                    alert();
                } else {
                    let stone: usize = if is_black_turn { 2 } else { 1 };

                    if board == get_reversed_board(id_x, id_y, stone, board) {
                        alert();
                    } else {
                        board[id_y][id_x] = stone;
                        board = get_reversed_board(id_x, id_y, stone, board);
                        is_black_turn = !is_black_turn;
                        println!("{:?}", count_stones(board));
                    }
                }
            }
        }

        e.mouse_cursor(|pos| {
            cursor = pos;
            let left_x = WINDOW_WIDTH / 2.0 - GRID_SIZE * 4.0;
            let top_y = WINDOW_HEIGHT / 2.0 - GRID_SIZE * 4.0;
            let (normal_x, normal_y) = (pos[0] - left_x, pos[1] - top_y);
            id_x = (normal_x / GRID_SIZE) as usize;
            id_y = (normal_y / GRID_SIZE) as usize;
        });
    }
}
