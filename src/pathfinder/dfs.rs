use std::collections::HashMap;
use std::rc::Rc;

use crate::maze::node::Position;
use crate::maze::Maze;

pub fn dfs(maze: &Maze) -> (Option<Vec<Position>>, usize) {
    let start = maze.start();
    let end = maze.end();

    let mut stack = Vec::new();
    let mut visited = HashMap::with_capacity(maze.count());
    let mut completed = false;
    let mut count = 0;

    stack.push(Rc::clone(&start));
    visited.insert(start.borrow().position(), None);

    while let Some(current) = stack.pop() {
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
                    stack.push(Rc::clone(&n));
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
