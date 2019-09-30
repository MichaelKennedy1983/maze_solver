use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::maze::node::Position;
use crate::maze::Maze;

pub fn bfs(maze: &Maze) -> (Option<Vec<Position>>, usize) {
    let start = maze.start();
    let end = maze.end();

    let mut completed = false;
    let mut queue = VecDeque::new();
    let mut visited = HashMap::with_capacity(maze.count());
    let mut count = 0;

    visited.insert(start.borrow().position(), None);
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        count += 1;

        if current == end {
            completed = true;
            break;
        }

        let cpos = current.borrow().position();

        for maybe_n in current.borrow().neighbors() {
            if let Some(n) = maybe_n {
                let npos = n.borrow().position();

                if !visited.contains_key(&npos) {
                    queue.push_back(Rc::clone(&n));
                    visited.insert(npos, Some(cpos));
                }
            }
        }
    }

    if completed {
        let epos = end.borrow().position();

        let mut path = Vec::new();
        let mut current = visited[&epos];

        path.push(epos);
        while let Some(cpos) = current {
            path.push(cpos);
            current = visited[&cpos];
        }

        (Some(path.into_iter().rev().collect()), count)
    } else {
        (None, count)
    }
}
