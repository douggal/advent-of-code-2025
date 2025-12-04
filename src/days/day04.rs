use std::time::Instant;
use image::{ImageBuffer, Rgb};
use ab_glyph::{FontArc, PxScale, Glyph, point, Font};




////////////////////////////////////////////////////////////////
// Advent of Code 2025 Day 4
// Link: <a href="...">https://adventofcode.com/2025/day/4</a>
// 4 December 2025
////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq)]
pub struct GridCoordinate {
    q:i32,
    r:i32,
}

impl GridCoordinate {
    pub fn new(q:i32, r:i32) -> GridCoordinate {
        GridCoordinate{q, r}
    }

    pub fn to_string(&self) -> String {
        format!("({}, {})", self.q, self.r)
    }
}

#[derive(Debug)]
pub struct PalletPosition {
    coord: GridCoordinate,
    s: String,
}

#[allow(unused)]
impl PalletPosition {
    pub fn new(q:i32, r:i32) -> PalletPosition {
        PalletPosition {coord: GridCoordinate::new(q, r), s: String::new()}
    }

    pub fn to_string(&self) -> String {
        format!("[{} {}]", self.s.as_str(), self.coord.to_string())
    }

    pub fn get_label_at(&self, q:i32, r:i32) -> &String {
        &self.s
    }
}



#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    println!("AoC 2025 Day 4");

    // Read the puzzle data file contents into a string
    // let filename = "./inputs/day04-test.txt";
    let filename = "./inputs/day04.txt";

    // Read the puzzle data file contents into a string
    let input = std::fs::read_to_string(filename)
        .expect("Failed to read input file for Day 4");

    // Cast the input as a Vector<String> with leading and trailing
    // whitespace trimmed, or as best suites each puzzle
    let input_vec = Vec::from(
        input
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<&str>>(),
    );


    // Debug:  Visually validate the puzzle input: Check for missing first and/or last row, etc!
    // dbg!(&input);
    #[cfg(debug_assertions)]
    {
        println!("Input file: {}", filename);
        println!("Lines of data in input: {}", input_vec.len());
        //println!("Raw input: {:?}", input);
        println!("Input as Vec<String>:\n{}", input_vec.join("\n"));
        println!("End input inspection\n");
    }

    // Track program runtime by elapsed time as shown by a "clock on the wall"
    let stop_watch = Instant::now();

    // let grid: Vec<Vec<Tile>>
    // (q,r) == (y, x) == (row, column), reverse of typical Cartesian coord point
    // q increases going down and r increases to the right
    //
    // [1 (0, 0)]	[2 (0, 1)]	[3 (0, 2)] ... [10 (0, 9)]
    // [4 (1, 0)]	[5 (1, 1)]	[6 (1, 2)] ...
    // [7 (2, 0)]	[8 (2, 1)]	[9 (2, 2)] ...
    // ...                                      [100 (9, 9)]

    // Grid capacity is known
    let rows = input_vec.len();
    let cols = input_vec[0].len();

    println!("Rows {:?}, Cols {:?}", rows, cols);

    // Create a grid
    let mut grid: Vec<Vec<PalletPosition>> = (0..rows)
        .map(|q| (0..cols).map(|c| PalletPosition { coord:GridCoordinate{q:q as i32,r:c as i32}, s:(q * cols + c + 1).to_string()}).collect())
        .collect();

    // Print out the grid to validate visually that it is correct
    // for row in &grid {
    //     println!("{}", row.iter().map(|floor_position| floor_position.to_string()).collect::<Vec<String>>().join(" "));
    // }

    // Load the starting positions of the paper rolls into the grid
    for (row, line) in input_vec.into_iter().enumerate() {
        // println!("Index or row in grid {}: Line {}", row, line); // Row number
        for (col, ch) in line.chars().into_iter().enumerate() {
            if ch == '.' {
                grid[row][col].s = '.'.to_string();
            }
            else {
                grid[row][col].s = ch.to_string();
            }
        }
    }

    // Debug: Print out the grid to validate visually that it is correct
    // for row in &grid {
    //     println!("{}", row.iter().map(|floor_position| floor_position.to_string()).collect::<Vec<String>>().join(" "));
    // }

    // Start from (0,0), upper left
    let start_coord = GridCoordinate{q:0,r:0};
    let test_coords = vec![(-1,-1),(-1,0),(-1,1),
                           (0,-1),         (0,1),
                           (1,-1), (1,0), (1,1)];

    //////////
    // Part 1
    //////////

    let mut can_be_moved:Vec<i32> = vec![];
    for row in &grid {
        for pallet_pos in row {
            // println!("{}", pallet_position.to_string());
            let mut cnt = 0;
            if pallet_pos.s.as_str() == "@" {
                for coord in test_coords.iter() {
                    let q = pallet_pos.coord.q + coord.0;
                    let r = pallet_pos.coord.r + coord.1;
                    if q >= 0 && q < rows as i32 && r >= 0 && r < cols as i32 {
                        let contents = grid[q as usize][r as usize].s.to_string();
                        if contents == "@" {
                            cnt += 1;
                        }
                    }
                }
                if cnt < 4 {
                    can_be_moved.push(1);
                }
            }
            // println!("{:?}, Cnt: {}", grid[pallet_pos.coord.q as usize][pallet_pos.coord.r as usize], cnt);

        }
    }


    let answer_p1 = can_be_moved.iter().sum::<i32>();
    println!("Part 1 answer {}", answer_p1);
    let lap1 = stop_watch.elapsed();
    println!("Elapsed time part 1: {:.2?}\n", lap1);



    //////////
    // Part 2
    //////////

    let mut count_can_be_moved = 0;
    loop {
        let mut moved_this_run = 0;
        let mut can_be_moved2: Vec<GridCoordinate> = vec![];
        for row in &grid {
            for pallet_pos in row {
                // println!("{}", pallet_position.to_string());
                let mut cnt = 0;
                if pallet_pos.s.as_str() == "@" {
                    for coord in test_coords.iter() {
                        let q = pallet_pos.coord.q + coord.0;
                        let r = pallet_pos.coord.r + coord.1;
                        if q >= 0 && q < rows as i32 && r >= 0 && r < cols as i32 {
                            let contents = grid[q as usize][r as usize].s.to_string();
                            if contents == "@" {
                                cnt += 1;
                            }
                        }
                    }
                    if cnt < 4 {
                        can_be_moved2.push(GridCoordinate { q: pallet_pos.coord.q, r: pallet_pos.coord.r });
                        count_can_be_moved += 1;
                        moved_this_run+=1;
                    }
                }
                // println!("{:?}, Cnt: {}", grid[pallet_pos.coord.q as usize][pallet_pos.coord.r as usize], cnt);
            }
        }

        // remove paper rolls
        for roll in &can_be_moved2 {
            grid[roll.q as usize][roll.r as usize].s = ".".to_string();
        }

        if moved_this_run == 0{
            break; // Exit when no more paper rolls can be moved
        }
    }


    let answer_p2 = count_can_be_moved;
    println!("Part 2 answer ... {answer_p2}");
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed() - lap1);

    // Create an image of the final state after Part 2 run.

    let img_size = 700;
    let rows = grid.len();
    let cols = grid[0].len();
    let cell_width = img_size / cols;
    let cell_height = img_size / rows;

    // Create blank image (white background)
    let mut img = ImageBuffer::from_pixel(img_size as u32, img_size as u32, Rgb([255, 255, 255]));

    // Load font (place a .ttf file in your project directory)
    let font_data = include_bytes!("FiraCodeNerdFontMono-Bold.ttf");
    let font = FontArc::try_from_slice(font_data).unwrap();

    let scale = PxScale::from(cell_height as f32 * 0.8);

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, ch) in row.iter().enumerate() {
            let x = (col_idx * cell_width) as f32;
            let y = ((row_idx + 1) * cell_height) as f32;

            // Correct: use `point(x, y)` for position
            let glyph_id = font.glyph_id(grid[row_idx][col_idx].s.chars().next().unwrap());
            let glyph = Glyph {
                id: glyph_id,
                scale,
                position: point(x, y),
            };

            // Rasterize glyph
            if let Some(outlined) = font.outline_glyph(glyph) {
                outlined.draw(|gx, gy, coverage| {
                    let px = gx as f32 + outlined.px_bounds().min.x;
                    let py = gy as f32 + outlined.px_bounds().min.y;
                    let px_i = px.round() as i32;
                    let py_i = py.round() as i32;

                    if px_i >= 0 && py_i >= 0 && px_i < img_size as i32 && py_i < img_size as i32 {
                        let pixel = img.get_pixel_mut(px_i as u32, py_i as u32);
                        let val = (coverage * 255.0) as u8;
                        *pixel = Rgb([0u8, 0u8, 0u8]) // Black text
                    }
                });
            }
        }
    }

    img.save("./output_images/day04p2.png").unwrap();




    println!("\nTotal elapsed runtime: {:.2?}", stop_watch.elapsed());
}
