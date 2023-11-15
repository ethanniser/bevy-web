use std::collections::HashSet;
use std::iter::Iterator;

// make bitfield
#[derive(Debug, Clone)]
struct Neighbors {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Debug, Clone)]
struct Maze {
    // h*w long - indexed by x + y * w
    nodes: Vec<Neighbors>,
    width: usize,
    height: usize,
    start: usize,
    end: usize,
}

impl Maze {
    fn create_generator(&mut self) -> MazeGenerator {
        MazeGenerator::new(self)
    }
}

struct MazeGenerator<'a> {
    maze: &'a mut Maze,
    visited: HashSet<usize>,
    stack: Vec<usize>,
}

enum GeneratorState {
    Visited(usize),
    Backtrack(usize),
}

impl<'a> MazeGenerator<'a> {
    fn new(maze: &'a mut Maze) -> Self {

        Self {
            maze,
            visited: HashSet::new(),
            stack: Vec::new(),
        }
    }
}

impl<'a> Iterator for MazeGenerator<'a> {
    type Item = GeneratorState;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn foo() {}
}
