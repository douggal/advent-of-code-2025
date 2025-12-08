use std::time::Instant;


////////////////////////////////////////////////////////////////
// Advent of Code 2025 Day 7
// Link: <a href="...">https://adventofcode.com/2025/day/7</a>
// 7 December 2025
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

#[derive(Debug, Clone)]
pub struct Carre {
    coord: GridCoordinate,
    s: String,
    cnt: u64,
}

#[allow(unused)]
impl Carre {
    pub fn new(q:i32, r:i32) -> Carre {
        Carre {coord: GridCoordinate::new(q, r), s: String::new(), cnt: 0}
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
    println!("AoC 2025 Day 7");

    // Read the puzzle data file contents into a string
    let filename = "./inputs/day07-test.txt";
    // let filename = "./inputs/day07.txt";

    // Read the puzzle data file contents into a string
    let input = std::fs::read_to_string(filename)
        .expect("Failed to read input file for Day 7");

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


    // let manifold diagram be a Vec<Vec<Tile>>
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
    let mut manifold: Vec<Vec<Carre>> = (0..rows)
        .map(|q| (0..cols).map(|c| Carre { coord: GridCoordinate {q:q as i32,r:c as i32}, s:(q * cols + c + 1).to_string(), cnt: 0 }).collect())
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
                manifold[row][col].s = '.'.to_string();
            }
            else {
                manifold[row][col].s = ch.to_string();
            }
        }
    }

    // Debug: Print out the grid to validate visually that it is correct
    // for row in &manifold {
    //     println!("{}", row.iter().map(|floor_position| floor_position.to_string()).collect::<Vec<String>>().join(" "));
    // }

    // Track program runtime by elapsed time as shown by a "clock on the wall"
    let stop_watch = Instant::now();

    //////////
    // Part 1
    //////////

    // just process row by row? looking ahead 1
    for row in 0..manifold.len() {
        for col in 0..manifold[row].len() {
            let carre_contains = manifold[row][col].s.as_str();
            if carre_contains == "S" || carre_contains == "|" || carre_contains == "^" {
                // println!("{}", carre.to_string());
                if carre_contains == "S" || carre_contains == "|" {
                    let q = manifold[row][col].coord.q + 1;
                    let r = manifold[row][col].coord.r;
                    if q >= 0 && q < rows as i32 && r >= 0 && r < cols as i32 {
                        let is_splitter = if manifold[q as usize][r as usize].s == '^'.to_string() { true } else { false };
                        if !is_splitter {
                            // falls straight down
                            manifold[q as usize][r as usize].s = '|'.to_string();
                        } else {
                            // has to be a splitter
                            // beam split, update count
                            manifold[row][col].cnt += 1;

                            // update right and left
                            let right = r + 1;
                            let left = r - 1;
                            if right < cols as i32 {
                                if manifold[q as usize][right as usize].s != "^".to_string() {
                                    manifold[q as usize][right as usize].s = '|'.to_string();
                                    //manifold[q as usize][right as usize].cnt += 1;
                                    println!("Updated {:?}", manifold[q as usize][right as usize]);
                                }

                            }
                            if left >= 0i32 {
                                if manifold[q as usize][left as usize].s != "^".to_string() {
                                    manifold[q as usize][left as usize].s = '|'.to_string();
                                    //manifold[q as usize][right as usize].cnt += 1;
                                    println!("Updated {:?}", manifold[q as usize][left as usize]);
                                }

                            }
                        }
                    }
                }
            }
        }
    }

    // Debug: Print out the grid to validate visually that it is correct
    println!("after: ");
    for row in &manifold {
        println!("{}", row.iter().map(|floor_position| floor_position.to_string()).collect::<Vec<String>>().join(" "));
    }

    let mut splits_sum :u64 = 0;
    for row in &manifold {
        for carre in row {
            // Add number up and watch out for possible overflow condition while doing so

                if let Some(total) = splits_sum.checked_add(carre.cnt) {
                    splits_sum = total;
                } else {
                    println!("Overflow occurred while adding up the answers for part 1.");
                }

        }
    }

    println!("Part 1 answer {}", splits_sum);
    let lap1 = stop_watch.elapsed();
    println!("Elapsed time part 1: {:.2?}\n", lap1);


    //////////
    // Part 2
    //////////
    let answer_p2 = 0;
    println!("Part 2 answer {}", answer_p2);
    println!("Elapsed time part 2: {:.2?}", stop_watch.elapsed() - lap1);

    println!("\nTotal elapsed runtime: {:.2?}", stop_watch.elapsed());
}
