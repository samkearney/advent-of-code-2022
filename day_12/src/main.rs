// OK look, yes, I'm using a pathfinding library. But I'm doing this to learn a new language, and I
// don't really know algorithms, and I can only learn one thing at a time because I have baby
// brain. Lay off
use pathfinding::directed::dijkstra::dijkstra;

#[derive(PartialEq, Copy, Clone)]
enum NeighborDirection {
    Up,
    Down,
}

#[derive(Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Node {
    x: usize,
    y: usize,
}

impl Node {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn elevation(&self, grid: &[Vec<char>]) -> char {
        match grid[self.y][self.x] {
            'S' => 'a',
            'E' => 'z',
            elev => elev,
        }
    }

    pub fn is_end(&self, grid: &[Vec<char>]) -> bool {
        grid[self.y][self.x] == 'E'
    }

    pub fn neighbors(&self, grid: &[Vec<char>], direction: NeighborDirection) -> Vec<(Node, u32)> {
        let mut result = Vec::new();

        if self.x != 0 {
            // Left
            self.add_neighbor_if_possible(
                Node::new(self.x - 1, self.y),
                grid,
                direction,
                &mut result,
            );
        }
        if self.x < grid[0].len() - 1 {
            // Right
            self.add_neighbor_if_possible(
                Node::new(self.x + 1, self.y),
                grid,
                direction,
                &mut result,
            );
        }
        if self.y != 0 {
            // Up
            self.add_neighbor_if_possible(
                Node::new(self.x, self.y - 1),
                grid,
                direction,
                &mut result,
            );
        }
        if self.y < grid.len() - 1 {
            self.add_neighbor_if_possible(
                Node::new(self.x, self.y + 1),
                grid,
                direction,
                &mut result,
            );
        }

        result
    }

    fn add_neighbor_if_possible(
        &self,
        next: Node,
        grid: &[Vec<char>],
        direction: NeighborDirection,
        node_list: &mut Vec<(Node, u32)>,
    ) {
        if Self::can_move_to(self.elevation(grid), next.elevation(grid), direction) {
            node_list.push((next, 1))
        }
    }

    fn can_move_to(current: char, next: char, direction: NeighborDirection) -> bool {
        let next = next as u32;
        let current = current as u32;

        if direction == NeighborDirection::Up {
            next <= current || next == current + 1
        } else {
            next >= current || next == current - 1
        }
    }
}

fn main() {
    let grid = parse_input();

    part_1(&grid);
    part_2(&grid);
}

fn part_1(grid: &[Vec<char>]) {
    let start = find_node_with_value(grid, 'S');

    // https://i.redd.it/r2r1tsdasmf01.jpg
    let (route, cost) = dijkstra(
        &start,
        |node| node.neighbors(grid, NeighborDirection::Up),
        |node| node.is_end(grid),
    )
    .expect("Couldn't find path to target!");

    println!("\nDistance from start to end: {}\n", cost);

    // Just for fun, show the grid
    let mut path_grid = grid.to_owned();
    for node in route {
        path_grid[node.y][node.x] = '\u{2588}';
    }
    print_grid(&path_grid);
}

fn part_2(grid: &[Vec<char>]) {
    let start = find_node_with_value(grid, 'E');

    let (route, cost) = dijkstra(
        &start,
        |node| node.neighbors(grid, NeighborDirection::Down),
        |node| node.elevation(grid) == 'a',
    )
    .expect("Couldn't find path to target!");

    println!("\nDistance from any 'a' elevation to end: {}\n", cost);

    // Just for fun, show the grid
    let mut path_grid = grid.to_owned();
    for node in route {
        path_grid[node.y][node.x] = '\u{2588}';
    }
    print_grid(&path_grid);
}

fn find_node_with_value(grid: &[Vec<char>], value: char) -> Node {
    let flattened_pos = grid
        .iter()
        .flat_map(|row| row.iter())
        .position(|ch| *ch == value)
        .unwrap();
    Node::new(flattened_pos % grid[0].len(), flattened_pos / grid[0].len())
}

fn parse_input() -> Vec<Vec<char>> {
    let lines = aoc::read_lines("input.txt").expect("Couldn't open input.txt for reading");

    lines
        .map_while(|line| line.map_or(None, |text| Some(text.chars().collect())))
        .collect()
}

fn print_grid(grid: &[Vec<char>]) {
    for row in grid {
        for col in row {
            print!("{}", col)
        }
        println!();
    }
}
