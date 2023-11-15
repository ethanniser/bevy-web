use bevy::prelude::Vec2 as BevyVec2;
use rand::seq::SliceRandom;
use std::collections::HashSet;
use std::iter::Iterator;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vec2 {
    pub x: usize,
    pub y: usize,
}

impl Vec2 {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl std::convert::Into<BevyVec2> for Vec2 {
    fn into(self) -> BevyVec2 {
        BevyVec2::new(self.x as f32, self.y as f32)
    }
}

// make bitfield
#[derive(Debug, Clone)]
pub struct Neighbors {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

#[derive(Debug, Clone)]
pub struct Maze {
    // h*w long - indexed by x + y * w
    edges: Vec<Neighbors>,
    width: usize,
    height: usize,
    start: Vec2,
    end: Vec2,
}

impl Maze {
    pub fn create_generator(&mut self) -> MazeGenerator {
        MazeGenerator::new(self)
    }

    pub fn add_edge(&mut self, from: Vec2, to: Vec2) {
        todo!();
    }

    pub fn get_unvisited_neighbors(&self, cords: Vec2) -> Box<[Vec2]> {
        let index = cords.x + cords.y * self.width;
        let neighbors = &self.edges[index];

        let mut unvisited = Vec::new();

        if neighbors.up {
            unvisited.push(Vec2::new(cords.x, cords.y + 1));
        }
        if neighbors.down {
            unvisited.push(Vec2::new(cords.x, cords.y - 1));
        }
        if neighbors.left {
            unvisited.push(Vec2::new(cords.x - 1, cords.y));
        }
        if neighbors.right {
            unvisited.push(Vec2::new(cords.x + 1, cords.y));
        }

        unvisited.into_boxed_slice()
    }
}

pub struct MazeGenerator<'a> {
    maze: &'a mut Maze,
    visited: HashSet<usize>,
    stack: Vec<usize>,
    previous_state: Option<GeneratorState>,
}

#[derive(Debug, Clone)]
pub enum GeneratorState {
    Exploring(Vec2),
    Considering { from: Vec2, options: Box<[Vec2]> },
    Backtracking(Vec2),
}

impl<'a> MazeGenerator<'a> {
    fn new(maze: &'a mut Maze) -> Self {
        Self {
            maze,
            visited: HashSet::new(),
            stack: Vec::new(),
            previous_state: None,
        }
    }
}

impl<'a> Iterator for MazeGenerator<'a> {
    type Item = GeneratorState;
    /*
    considers all unvisited neighbors of current node
    randomly chooses one to visit next
    continues until all nodes visited
     */
    fn next(&mut self) -> Option<Self::Item> {
        match &self.previous_state {
            // first time
            None => todo!(),
            // all other times
            Some(prev_state) => match prev_state {
                GeneratorState::Exploring(cords) => todo!(),
                GeneratorState::Considering { from, options } => todo!(),
                GeneratorState::Backtracking(cords) => todo!(),
            },
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn foo() {}
}
