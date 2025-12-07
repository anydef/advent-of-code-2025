use std::fs::read_to_string;

type Scroll = bool;
type Map = Vec<Vec<Cell>>;
#[derive(Debug, Clone)]
struct Cell {
    scroll: Scroll,
    row: usize,
    col: usize,
}

impl Cell {
    fn new(scroll: Scroll, row: usize, col: usize) -> Self {
        Cell { scroll, row, col }
    }

    fn x_pos(&self, d: &Direction) -> Option<usize> {
        if d.x.is_negative() && self.col == 0 {
            return None;
        }
        if d.x.is_negative() {
            Some(self.col - (d.x.abs() as usize))
        } else {
            Some(self.col + (d.x.abs() as usize))
        }
    }

    fn y_pos(&self, d: Direction) -> Option<usize> {
        if d.y.is_negative() && self.row == 0 {
            return None;
        }
        if d.y.is_negative() {
            Some(self.row - (d.y.abs() as usize))
        } else {
            Some(self.row + (d.y.abs() as usize))
        }
    }
}

struct Direction {
    x: i8,
    y: i8,
}

impl Direction {
    fn new(x: i8, y: i8) -> Self {
        if x.abs() > 1 || y.abs() > 1 {
            panic!("Invalid direction");
        }
        Direction { x, y }
    }
}

const NW: Direction = Direction { x: -1, y: -1 };
const N: Direction = Direction { x: 0, y: -1 };
const NE: Direction = Direction { x: 1, y: -1 };
const E: Direction = Direction { x: 1, y: 0 };
const W: Direction = Direction { x: -1, y: 0 };
const SE: Direction = Direction { x: 1, y: 1 };
const S: Direction = Direction { x: 0, y: 1 };
const SW: Direction = Direction { x: -1, y: 1 };

const DIRECTIONS: [Direction; 8] = [NW, N, NE, E, SE, S, SW, W];

trait Nav {
    fn check_neighbors(&self, map: &Map) -> u8;
    fn has_neighbor(&self, d: Direction, map: &Map) -> bool;
}

impl Nav for Cell {
    fn check_neighbors(&self, map: &Map) -> u8 {

        let mut neighbors_count: u8 = 0;
        for d in DIRECTIONS {
            if self.has_neighbor(d, map) {
                neighbors_count += 1;
            }
        }
        neighbors_count
    }

    fn has_neighbor(&self, d: Direction, map: &Map) -> bool {

        let x_pos = self.x_pos(&d);
        let y_pos = self.y_pos(d);
        if x_pos.is_none() || y_pos.is_none() {
            return false;
        }

        let y = y_pos.expect("Missed None check");
        let x = x_pos.expect("Missed None check");

        if x > map[0].len() - 1 || y > map.len() - 1 {
            return false;
        }

        map[y][x].scroll
    }
}

fn main() {
    println!("Day 04");
    let scrolls_map = read_scrolls_from_file("resources/day04/input.txt");
    // let scrolls_map = read_scrolls_from_file("resources/day04/example_input.txt");
    let mut scrolls = map_to_cells(scrolls_map);
    let mut moved_scrolls_total = 0;
    loop {
        let (map_after_move, moved_scrolls) = move_scrolls(&scrolls);
        println!("Movable scrolls count: {}", moved_scrolls);
        if moved_scrolls == 0 {
            break;
        }
        moved_scrolls_total += moved_scrolls;
        scrolls = map_after_move;
    }
    println!("Total moved scrolls {}", moved_scrolls_total);
}

fn move_scrolls(scrolls: &Map) -> (Map, usize) {
    let mut movable_scrolls = 0;
    let mut after_move_map = Map::new();

    for row in scrolls {
        let mut after_move_row: Vec<Cell> = vec![];

        for cell in row {
            if !cell.scroll {
                after_move_row.push(Cell {scroll: false, row: cell.row, col: cell.col});
                continue;
            }
            let neighbors = cell.check_neighbors(scrolls);
            if neighbors < 4 && cell.scroll {
                println!("Cell at ({}, {}) is movable with {} neighbors", cell.row, cell.col, neighbors);
                movable_scrolls += 1;
                after_move_row.push(Cell {scroll: false, row: cell.row, col: cell.col});
            } else {
                after_move_row.push(Cell {scroll: true, row: cell.row, col: cell.col});
            }
        }
        after_move_map.push(after_move_row);
    }

    (after_move_map, movable_scrolls)
}

fn map_to_cells(scrolls_map: Vec<Vec<Scroll>>) -> Map {
    let mut cells: Vec<Vec<Cell>> = vec![];
    for (row_idx, row) in scrolls_map.iter().enumerate() {
        let mut scrolls_row: Vec<Cell> = vec![];
        for (col_idx, cell) in row.iter().enumerate() {


            let cell = if *cell {
                Cell::new(true, row_idx, col_idx)
            } else {
                Cell::new(false, row_idx, col_idx)
            };
            scrolls_row.push(cell.clone());
            // print!("{:?}", cell);
        }
        cells.push(scrolls_row);
    }
    cells
}

fn read_scrolls_from_file(filename: &str) -> Vec<Vec<Scroll>> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .map(|s| {
            s.chars()
                .map(|c| {
                    if c == '.' {
                        false
                    } else if c == '@' {
                        true
                    } else {
                        panic!("Invalid input")
                    }
                })
                .collect::<Vec<Scroll>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation() {
        let scrolls_map = read_scrolls_from_file("resources/day04/example_input.txt");
        let cells = map_to_cells(scrolls_map);
        let cell = &cells[0][3];
        assert_eq!(cell.scroll, true);
        assert_eq!(cell.row, 0);
        assert_eq!(cell.col, 3);
    }

    #[test]
    fn test_check_neighbor_oob_x_negative() {
        let scrolls_map = vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ];
        let cells = map_to_cells(scrolls_map);
        let cell = &cells[0][0];
        let has_neighbor = cell.has_neighbor(W, &cells);
        assert_eq!(has_neighbor, false);
    }

    #[test]
    fn test_check_neighbor_oob_x_positive() {
        let scrolls_map = vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ];
        let cells = map_to_cells(scrolls_map);
        let cell = &cells[2][2];
        let has_neighbor = cell.has_neighbor(E, &cells);
        assert_eq!(has_neighbor, false);
    }

    #[test]
    fn test_check_neighbor_oob_y_negative() {
        let scrolls_map = vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ];
        let cells = map_to_cells(scrolls_map);
        let cell = &cells[0][0];
        let has_neighbor = cell.has_neighbor(N, &cells);
        assert_eq!(has_neighbor, false);
    }

    #[test]
    fn test_check_neighbor_oob_y_positive() {
        let scrolls_map = vec![
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
        ];
        let cells = map_to_cells(scrolls_map);
        let cell = &cells[2][2];
        let has_neighbor = cell.has_neighbor(S, &cells);
        assert_eq!(has_neighbor, false);
    }

    #[test]
    fn test_cell_without_scroll_no_neighbors() {
        let scrolls_map = vec![
            vec![true, true, true],
            vec![true, false, true],
            vec![true, true, true],
        ];
        let cells = map_to_cells(scrolls_map);
        let cell = &cells[1][1];
        let has_neighbor = cell.has_neighbor(S, &cells);
        assert_eq!(has_neighbor, false);

        let count_neighbors = cell.check_neighbors(&cells);
        assert_eq!(count_neighbors, 0);
    }


    #[test]
    fn test_check_neighbor_in_bounds() {
        let scrolls_map = vec![
            vec![true, false, false],
            vec![false, true, false],
            vec![false, false, true],
        ];
        let cells = map_to_cells(scrolls_map);
        let cell = &cells[1][1];
        let has_neighbor = cell.has_neighbor(NE, &cells);
        assert_eq!(has_neighbor, false);

        let has_neighbor = cell.has_neighbor(NW, &cells);
        assert_eq!(has_neighbor, true);

        let has_neighbor = cell.has_neighbor(SE, &cells);
        assert_eq!(has_neighbor, true);

        let has_neighbor = cell.has_neighbor(S, &cells);
        assert_eq!(has_neighbor, false);
    }
}
