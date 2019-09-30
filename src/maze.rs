#![allow(unused)]
pub mod errors;
pub mod node;

use image::GenericImageView;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use errors::MazeError;
use node::Node;

#[derive(Debug, Clone)]
pub struct Maze {
    width: u32,
    height: u32,
    count: usize,
    start: Rc<RefCell<Node>>,
    end: Rc<RefCell<Node>>,
}

// associate functions
impl Maze {
    pub fn new(img: &image::RgbImage) -> Result<Maze, MazeError> {
        const WALL: image::Rgb<u8> = image::Rgb { data: [0, 0, 0] };

        let width = img.width();
        let height = img.height();
        let buf = img;

        let mut start: Option<Rc<RefCell<Node>>> = None;
        let mut top_nodes: HashMap<u32, Rc<RefCell<Node>>> = HashMap::new();
        let mut count = 0;

        // find start
        for x in 1..width - 1 {
            if buf[(x, 0)] != WALL {
                let new = Rc::new(RefCell::new(Node::new(x, 0)));
                top_nodes.insert(x, Rc::clone(&new));
                start = Some(new);
                count += 1;
                break;
            }
        }

        let start = if let Some(s) = start {
            s
        } else {
            return Err(MazeError::NoStartError);
        };

        for y in 1..height - 1 {
            // track if adjacent horizontal pixels are a path
            let mut prv: bool;
            let mut cur = false;
            let mut nxt = buf[(1, y)] != WALL;

            let mut left_node: Option<Rc<RefCell<Node>>> = None;

            for x in 1..width - 1 {
                prv = cur;
                cur = nxt;
                nxt = buf[(x + 1, y)] != WALL;

                let mut n: Option<Rc<RefCell<Node>>> = None;

                if !cur {
                    // on wall
                    continue;
                }

                if prv {
                    // PATH PATH PATH
                    if nxt {
                        // Create node if paths above or below
                        if buf[(x, y - 1)] != WALL || buf[(x, y + 1)] != WALL {
                            let new = Rc::new(RefCell::new(Node::new(x, y)));
                            if let Some(node) = &left_node {
                                node.borrow_mut().set_east(Rc::clone(&new));
                                new.borrow_mut().set_west(Rc::clone(&node));
                            }
                            left_node = Some(Rc::clone(&new));
                            n = Some(new);
                        }
                    // PATH PATH WALL
                    // create path at end of corridor
                    } else {
                        let new = Rc::new(RefCell::new(Node::new(x, y)));
                        if let Some(node) = &left_node {
                            node.borrow_mut().set_east(Rc::clone(&new));
                            new.borrow_mut().set_west(Rc::clone(&node));
                        }
                        left_node = None;
                        n = Some(new);
                    }
                } else {
                    // WALL PATH PATH
                    // create path at start of corridor
                    if nxt {
                        let new = Rc::new(RefCell::new(Node::new(x, y)));
                        left_node = Some(Rc::clone(&new));
                        n = Some(new);

                    // WALL PATH WALL
                    } else {
                        // create node only if dead end
                        if buf[(x, y - 1)] == WALL || buf[(x, y + 1)] == WALL {
                            n = Some(Rc::new(RefCell::new(Node::new(x, y))));
                        }
                    }
                }

                // If node isn't none, we can try to connect it
                if let Some(node) = n {
                    // clear above, connect to top node
                    if buf[(x, y - 1)] != WALL {
                        let top = top_nodes.get(&x);
                        if let Some(tn) = top {
                            tn.borrow_mut().set_south(Rc::clone(&node));
                            node.borrow_mut().set_north(Rc::clone(&tn));
                        }
                    }

                    // if clear below, put this node in the top row for next connection
                    if buf[(x, y + 1)] != WALL {
                        top_nodes.insert(x, Rc::clone(&node));
                    } else {
                        top_nodes.remove(&x);
                    }

                    count += 1;
                }
            }
        }

        // last row
        let mut end: Option<Rc<RefCell<Node>>> = None;
        let y = height - 1;
        for x in 1..width - 1 {
            if buf[(x, y)] != WALL {
                let new = Rc::new(RefCell::new(Node::new(x, y)));
                let top = top_nodes.get(&x);
                if let Some(tn) = top {
                    tn.borrow_mut().set_south(Rc::clone(&new));
                    new.borrow_mut().set_north(Rc::clone(tn));
                }
                end = Some(new);
                count += 1;
                break;
            }
        }

        let end = if let Some(node) = end {
            node
        } else {
            return Err(MazeError::NoExitError);
        };

        Ok(Maze {
            width,
            height,
            count,
            start,
            end,
        })
    }
}

// methods
impl Maze {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn start(&self) -> Rc<RefCell<Node>> {
        Rc::clone(&self.start)
    }

    pub fn end(&self) -> Rc<RefCell<Node>> {
        Rc::clone(&self.end)
    }
}
