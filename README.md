#Maze Solver
##About
A short program which solves mazes using various algorithms and written
in Rust. It is adapted from Dr. Mike Pound's [video](https://www.youtube.com/watch?v=rop0W4QDOUI)
for Computerphile and Python [code](https://github.com/mikepound/mazesolving).
My goal with this project was to practice using the Rust language and reacquaint myself with search algorithms.

##Build
Assuming Rust's build tool and package manager, Cargo, is already
installed:  
1. clone repository  
2. change directory to the cloned repository  
3. run 'cargo build' in the repository directory  

##Run
maze_solver input \[-a algorithm\] \[-o output directory\]

##Input
Taken from Dr. Pound's documentation:  
- Each maze is black and white. White represents paths, black represents
  walls.  
- All mazes are surrounded entirely by black walls.  
- One white square exists on the top row of the image, and is the start
  of the maze.  
- One white square exists on the bottom row of the image, and is the end
  of the maze.  

There are some example mazes in the ./img directory.
