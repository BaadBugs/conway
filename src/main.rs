extern crate image;
use image::{ImageBuffer, Rgba};
use rand::Rng;

const GRID_WIDTH: usize = 250; 
const GRID_HEIGHT: usize = 80; 

fn create_empty_grid() -> Vec<Vec<bool>> {
    vec![vec![false; GRID_WIDTH]; GRID_HEIGHT] 
}

fn generate_image(grid: &Vec<Vec<bool>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    const IMAGE_WIDTH: u32 = 1920;
    const IMAGE_HEIGHT: u32 = 1080;

    // Create a blank image
    let mut img = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for x in 0..IMAGE_WIDTH {
        for y in 0..IMAGE_HEIGHT {

            let grid_x = (x * GRID_WIDTH as u32 / IMAGE_WIDTH).min(GRID_WIDTH as u32 - 1);
            let grid_y = (y * GRID_HEIGHT as u32 / IMAGE_HEIGHT).min(GRID_HEIGHT as u32 - 1);

            let cell = grid[grid_y as usize][grid_x as usize];

            let color = if cell {
                Rgba([63, 3, 3, 255])
            } else {
                Rgba([0, 0, 0, 255])
            };

            img.put_pixel(x, y, color);
        }
    }

    img
}

fn randomize_grid(grid: &mut Vec<Vec<bool>>) {
    let mut rng = rand::thread_rng(); 

    for row in grid.iter_mut() {
        for cell in row.iter_mut() {
            let random_number: f64 = rng.gen();
            let cell_value = random_number > 0.3; 
            *cell = cell_value;
        }
    }
}

fn save_image(filename: &str, grid: &Vec<Vec<bool>>) {
    let image = generate_image(grid);
    image.save(filename).expect("Failed to save image");
}

fn update_grid(grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = grid.clone();

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let cell = grid[row][col];
            let live_neighbors = count_live_neighbors(&grid, row, col);

            if cell {
                if live_neighbors < 2 || live_neighbors > 3 {
                    new_grid[row][col] = false;
                }
            } else {
                if live_neighbors == 3 {
                    new_grid[row][col] = true;
                }
            }
        }
    }

    *grid = new_grid;
}

fn count_live_neighbors(grid: &Vec<Vec<bool>>, row: usize, col: usize) -> usize {
    let mut live_count = 0;
    let rows = GRID_HEIGHT as isize;
    let cols = GRID_WIDTH as isize;

    let offsets = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    for &(dr, dc) in &offsets {
        let r = row as isize + dr;
        let c = col as isize + dc;

        if r >= 0 && r < rows && c >= 0 && c < cols {
            if grid[r as usize][c as usize] {
                live_count += 1;
            }
        }
    }

    live_count
}

fn display_grid(grid: &Vec<Vec<bool>>) {
    println!("grid:");

    for row in 0..GRID_HEIGHT {
        for col in 0..GRID_WIDTH {
            let cell = grid[row][col];

            print!("{}", if cell { '@' } else { ' ' });
        }
        println!("");
    }
}

fn initialize_glider_gun(grid: &mut Vec<Vec<bool>>) {
    let glider_gun = [
        (5, 1), (6, 1), (5, 2), (6, 2), 
        (5, 11), (6, 11), (7, 11),
        (4, 12), (8, 12),
        (3, 13), (9, 13),
        (3, 14), (9, 14),
        (6, 15),
        (4, 16), (8, 16),
        (5, 17), (6, 17), (7, 17),
        (6, 18),
        (3, 21), (4, 21), (5, 21),
        (3, 22), (4, 22), (5, 22),
        (2, 23), (6, 23),
        (1, 25), (2, 25), (6, 25), (7, 25),
        (3, 35), (4, 35),
        (3, 36), (4, 36),
    ];

    for &(row, col) in &glider_gun {
        grid[row][col] = true;
    }
}

fn add_not_gate(grid: &mut Vec<Vec<bool>>) {
    let not_gate = [
        (11, 3), (11, 4), (11, 5), (11, 6), (11, 7),
        (12, 3), (12, 7),
        (13, 3), (13, 7),
        (14, 3), (14, 4), (14, 5), (14, 6), (14, 7),
    ];

    for &(row, col) in &not_gate {
        grid[row][col] = true;
    }
}

fn main() {
    let mut grid = create_empty_grid();

    //randomize_grid(&mut grid);

    initialize_glider_gun(&mut grid);
    //add_not_gate(&mut grid);

    let mut reset_counter = 0;
    loop {

        println!("\x1B[2J\x1B[H"); // clears terminal

        display_grid(&grid);
        update_grid(&mut grid);

        // if reset_counter > 1000{
        //     randomize_grid(&mut grid);
        //     reset_counter = 0;
        // }

        // reset_counter += 1;

        //save_image("output.png", &grid);

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
