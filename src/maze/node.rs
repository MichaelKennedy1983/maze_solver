#![allow(unused)]
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Node {
    position: Position,
    // 0 = north, 1 = east, 2 = south, 3 = west
    neighbors: [Option<Rc<RefCell<Node>>>; 4],
}

// associate functions
impl Node {
    pub fn new(col: u32, row: u32) -> Node {
        Node {
            position: Position::new(col, row),
            neighbors: [None, None, None, None],
        }
    }
}

// methods
impl Node {
    pub fn col(&self) -> u32 {
        self.position.col
    }

    pub fn row(&self) -> u32 {
        self.position.row
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn north(&self) -> Option<Rc<RefCell<Node>>> {
        if let Some(n) = &self.neighbors[0] {
            return Some(Rc::clone(n));
        } else {
            return None;
        }
    }

    pub fn south(&self) -> Option<Rc<RefCell<Node>>> {
        if let Some(n) = &self.neighbors[2] {
            return Some(Rc::clone(n));
        } else {
            return None;
        }
    }

    pub fn east(&self) -> Option<Rc<RefCell<Node>>> {
        if let Some(n) = &self.neighbors[1] {
            return Some(Rc::clone(n));
        } else {
            return None;
        }
    }

    pub fn west(&self) -> Option<Rc<RefCell<Node>>> {
        if let Some(n) = &self.neighbors[3] {
            return Some(Rc::clone(n));
        } else {
            return None;
        }
    }

    pub fn set_north(&mut self, neighbor: Rc<RefCell<Node>>) {
        self.neighbors[0] = Some(neighbor);
    }

    pub fn set_south(&mut self, neighbor: Rc<RefCell<Node>>) {
        self.neighbors[2] = Some(neighbor);
    }

    pub fn set_east(&mut self, neighbor: Rc<RefCell<Node>>) {
        self.neighbors[1] = Some(neighbor);
    }

    pub fn set_west(&mut self, neighbor: Rc<RefCell<Node>>) {
        self.neighbors[3] = Some(neighbor);
    }

    pub fn neighbors(&self) -> &[Option<Rc<RefCell<Node>>>] {
        &self.neighbors
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    col: u32,
    row: u32,
}

impl Position {
    pub fn new(col: u32, row: u32) -> Position {
        Position { col, row }
    }
}

impl Position {
    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn row(&self) -> u32 {
        self.row
    }

    pub fn as_tuple(&self) -> (u32, u32) {
        (self.col, self.row)
    }
}
