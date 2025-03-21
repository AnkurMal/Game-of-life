use macroquad::prelude::*;

const CELL_SIZE: f32 = 10.;

#[macroquad::main(window_conf)]
async fn main() {
    let row = (screen_height()/CELL_SIZE) as usize;
    let column = (screen_width()/CELL_SIZE) as usize;

    let mut data = vec![vec![0; column]; row];
    for row in data.iter_mut() {
        for col in row {
            *col = rand::gen_range(0, 2);
        }
    }

    let manip = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1)
    ];

    loop {
        let mut new_data = data.clone();

        clear_background(BLACK);

        for i in 0..row {
            for j in 0..column {
                let mut sum = 0;

                for (x, y) in &manip {
                    let ni = i as isize + x;
                    let nj = j as isize + y;

                    if ni >= 0 && ni < row as isize &&
                    nj >= 0 && nj < column as isize {
                        sum += data[ni as usize][nj as usize];
                    }
                }

                if data[i][j] == 1 {
                    if !(2..=3).contains(&sum) {
                        new_data[i][j] = 0;
                    }
                }
                else if sum == 3 {
                    new_data[i][j] = 1;
                }
            }
        }

        data = new_data;
        for (i, col) in data.iter().enumerate() {
            for (j, &life) in col.iter().enumerate() {
                let color = if life == 1 { WHITE } else { BLACK };
                draw_rectangle(j as f32*CELL_SIZE, i as f32*CELL_SIZE, CELL_SIZE, CELL_SIZE, color);
            }
        }

        next_frame().await
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Game of Life".to_string(),
        window_width: 900,
        window_height: 900,
        icon: None,
        window_resizable: false,
        high_dpi: true,
        ..Default::default()
    }
}