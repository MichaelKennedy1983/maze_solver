use priority_queue::PriorityQueue;

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::i64;
use std::rc::Rc;

use crate::maze::node::{Node, Position};
use crate::maze::Maze;

pub fn dijk(maze: &Maze) -> (Option<Vec<Position>>, usize) {
    let start = maze.start();
    let end = maze.end();
    let start_pos = start.borrow().position();
    let end_pos = end.borrow().position();

    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::with_capacity(maze.count());

    let mut completed = false;
    let mut count = 0;

    let start_nd = NodeDist::new(Rc::clone(&start), 0);
    queue.push(start_nd);
    visited.insert(start_pos, (0, None));

    while let Some(current) = queue.pop() {
        count += 1;

        let cnode = current.node;
        let cpos = cnode.borrow().position();
        let cdist = current.distance;

        if cpos == end_pos {
            completed = true;
            break;
        }

        for maybe_n in cnode.borrow().neighbors() {
            if let Some(n) = maybe_n {
                let npos = n.borrow().position();
                let mut ndist = (i64::from(npos.col()) - i64::from(cpos.col())).abs()
                    + (i64::from(npos.row()) - i64::from(cpos.row())).abs();
                ndist += cdist;

                if !visited.contains_key(&npos) || ndist < visited[&npos].0 {
                    let n_nd = NodeDist::new(Rc::clone(&n), ndist);
                    queue.push(n_nd);
                    visited.insert(npos, (ndist, Some(cpos)));
                }
            }
        }
    }

    if completed {
        let mut path = Vec::new();
        path.push(end_pos);
        let mut current = visited[&end_pos];
        while let (_, Some(cpos)) = current {
            path.push(cpos);
            current = visited[&cpos];
        }

        (Some(path.into_iter().rev().collect()), count)
    } else {
        (None, count)
    }
}

pub fn dijk2(maze: &Maze) -> (Option<Vec<Position>>, usize) {
    let start = maze.start();
    let end = maze.end();
    let start_pos = start.borrow().position();
    let end_pos = end.borrow().position();

    let mut queue = PriorityQueue::new();
    let mut visited = HashMap::with_capacity(maze.count());

    let mut completed = false;
    let mut count = 0;

    let start_nd = NodeDist::new(Rc::clone(&start), 0);
    queue.push(start_pos, start_nd);
    visited.insert(start_pos, (0, None));

    while let Some((cpos, current)) = queue.pop() {
        count += 1;

        let cnode = current.node;
        let cdist = current.distance;

        if cpos == end_pos {
            completed = true;
            break;
        }

        for maybe_n in cnode.borrow().neighbors() {
            if let Some(n) = maybe_n {
                let npos = n.borrow().position();
                let mut ndist = (i64::from(npos.col()) - i64::from(cpos.col())).abs()
                    + (i64::from(npos.row()) - i64::from(cpos.row())).abs();
                ndist += cdist;

                if visited.contains_key(&npos) && ndist < visited[&npos].0 {
                    let n_nd = NodeDist::new(Rc::clone(&n), ndist);
                    queue.change_priority(&npos, n_nd);
                    visited.insert(npos, (ndist, Some(cpos)));
                }
                if !visited.contains_key(&npos) {
                    let n_nd = NodeDist::new(Rc::clone(&n), ndist);
                    queue.push(npos, n_nd);
                    visited.insert(npos, (ndist, Some(cpos)));
                }
            }
        }
    }

    if completed {
        let mut path = Vec::new();
        path.push(end_pos);
        let mut current = visited[&end_pos];
        while let (_, Some(cpos)) = current {
            path.push(cpos);
            current = visited[&cpos];
        }

        (Some(path.into_iter().rev().collect()), count)
    } else {
        (None, count)
    }
}

#[derive(Debug, Eq, Clone)]
struct NodeDist {
    node: Rc<RefCell<Node>>,
    distance: i64,
}

impl NodeDist {
    pub fn new(node: Rc<RefCell<Node>>, distance: i64) -> NodeDist {
        NodeDist { node, distance }
    }
}

impl PartialEq for NodeDist {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for NodeDist {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.distance.cmp(&self.distance))
    }
}

impl Ord for NodeDist {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}
