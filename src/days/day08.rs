use std::time::Instant;
////////////////////////////////////////////////////////////////
// Advent of Code 2025 Day 8
// Link: <a href="...">https://adventofcode.com/2025/day/8</a>
// 8 December 2025
////////////////////////////////////////////////////////////////

// Model playground as a graph of vertices/nodes
// Each vertex has:
//   - coordinates
//   - name or label
//   - list of distances to every other vertex
//   - list of connected vertices, initially empty

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Coordinate {
    x:u64,
    y:u64,
    z:u64,
}

impl Coordinate {
    // Associated function to create a new Coordinate
    fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }

    pub fn to_string(&self) -> String {
        format!("({},{},{})", self.x,self.y,self.z)
    }
}

#[derive(Clone,Debug,PartialEq, PartialOrd)]
pub struct Vertex {
    id: u64,
    coord: Coordinate,
    name: String,
    distances: Vec<f64>,
    connected_to: Vec<u64>,
}

pub struct Graph {
    pub name: String,
    pub nbr_vertices: u64,  // number vertices
    pub nbr_edges: u64,  // number edges
    pub vertices: Vec<Vertex>,  // vector of vectors
}

#[allow(unused)]
impl Graph {
    // Constructor-like function
    pub fn new(name: String, nbr_vertices: u64) -> Self {
        Self {
            name,
            nbr_vertices: nbr_vertices,
            nbr_edges: 0,
            vertices: Vec::new(),
        }
    }

    pub fn build_graph(name: String, input_vec: Vec<&str>) -> Graph {
        // initially all the junction boxes are not connected

        let nbr_vertices : u64 = input_vec.len() as u64;
        let mut my_graph = Graph::new(name, nbr_vertices);
        my_graph.nbr_edges = 0u64;

        let mut vertices : Vec<Vertex> = Vec::new();
        for (id, line) in input_vec.into_iter().enumerate() {
            // parse (x,y,z) coords, create vertex, add to graph

            // Split by comma and parse into u64
            let parts: Vec<u64> = line
                .split(',')
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect();

            let mut distances: Vec<f64> = Vec::new();
            let mut connected_to: Vec<u64> = Vec::new();
            vertices.push(Vertex {
                id: id as u64,
                coord: Coordinate::new(parts[0], parts[1], parts[2] ),
                name: id.to_string(),
                distances: distances,
                connected_to: connected_to,
            })
        }
        // sort the vertices for easy reading during debugging
        // for v in my_graph.vertices.iter_mut() {
        //     v.sort();
        // }

        my_graph.vertices = vertices;

        my_graph
    }

    fn add_edge(&mut self, from: u64, to: u64) {
        self.vertices[from as usize].connected_to.push(to);
        self.vertices[to as usize].connected_to.push(from);
        self.nbr_edges += 1;
    }


    pub fn display(&self) {
        println!("A Graph, name = {}, with {} Vertices and {} Edges.  List of Vertices:", self.name, self.nbr_vertices, self.nbr_edges);
        for i in 0..self.vertices.len() {
            println!("{} - {}", i, self.vertices[i].coord.to_string());

            let output = self.vertices[i].connected_to
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join(",");

            println!("...Connected to: {}", output);

        }
    }
}


#[allow(dead_code)]
#[allow(unused_variables)]
pub fn run() {
    println!("AoC 2025 Day 8");

    // Read the puzzle data file contents into a string
    let filename = "./inputs/day08-test.txt";
    // let filename = "./inputs/day08.txt";

    // Read the puzzle data file contents into a string
    let input = std::fs::read_to_string(filename)
        .expect("Failed to read input file for Day 8");

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

    //////////
    // Part 1
    //////////

    // model the playground a graph of vertices
    let mut graph = Graph::build_graph(String::from("Playground"), input_vec);

    // compute distances of each vertex to all the others
    for v_id in 0..graph.nbr_vertices {
        // println!("{:?}", v);
        for to_id in 0..graph.nbr_vertices {
            let p = &graph.vertices[v_id as usize].coord;
            let q = &graph.vertices[to_id as usize].coord;
            let radicand = (p.x as f64 - q.x as f64).powi(2) + (p.y as f64 - q.y as f64).powi(2) + (p.z as f64 - q.z as f64).powi(2);
            let distance = radicand.sqrt();
            graph.vertices[v_id as usize].distances.push(distance);
        }
    }

    // print graph if not too big
    if graph.nbr_vertices < 50 {
        graph.display();
    }
    else {
        println!("My graph {} has {} vertices and {} edges", graph.name, graph.nbr_vertices, graph.nbr_edges);
        println!("Graph to big to display");
    }

    // find two vertices with the smallest distance between them
    // for each vertex find the closest node
    // find the two vertices with least distance between them.
    let mut min_dist_ids: Vec<u64> = Vec::new();
    let mut min_dists: Vec<f64> = Vec::new();
    for v_id in 0..graph.nbr_vertices {

        let mut closest = 0u64;
        let mut closest_dist = f64::INFINITY;
        for to_id in 0..graph.nbr_vertices {
            if v_id != to_id {
                if graph.vertices[v_id as usize].distances[to_id as usize] < closest_dist {
                    closest_dist = graph.vertices[v_id as usize].distances[to_id as usize];
                    closest = to_id;
                }
            }
        }
        min_dist_ids.push(closest);
        min_dists.push(closest_dist);
    }

    // Debug: display output
    // for (id, x) in min_dist_ids.iter().zip(min_dists).enumerate() {
    //     println!("{}: {},{}", id, x.0, x.1);
    // }
    let mut min: Vec<_> = min_dist_ids.iter().zip(min_dists).collect();
    min.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    for item in min.iter().take(2) {
        println!("{} - {} {}", item.0, item.1.to_string(), graph.vertices[*item.0 as usize].coord.to_string());
    }
    graph.vertices[*min[0].0 as usize].connected_to.push(*min[1].0);
    graph.vertices[*min[1].0 as usize].connected_to.push(*min[0].0);

    // print graph if not too big
    if graph.nbr_vertices < 50 {
        graph.display();
    }
    else {
        println!("My graph {} has {} vertices and {} edges", graph.name, graph.nbr_vertices, graph.nbr_edges);
        println!("Graph to big to display");
    }


    let answer_p1 = 0;
    println!("Part 1 answer {}", answer_p1);
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
