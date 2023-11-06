use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Graph {
    nodes: Vec<HashSet<usize>>,
}

impl Graph {
    fn new_grid(width: usize, height: usize) -> Self {
        let mut nodes = vec![HashSet::new(); width * height];

        for row in 0..height {
            for col in 0..width {
                let index = row * width + col;

                if col > 0 {
                    nodes[index].insert(index - 1);
                }
                if col < width - 1 {
                    nodes[index].insert(index + 1);
                }
                if row > 0 {
                    nodes[index].insert(index - width);
                }
                if row < height - 1 {
                    nodes[index].insert(index + width);
                }
            }
        }

        Self { nodes }
    }

    fn generate_maze(&self) -> impl Iterator<target = > {}
}

impl std::convert::AsRef<Vec<HashSet<usize>>> for Graph {
    fn as_ref(&self) -> &Vec<HashSet<usize>> {
        &self.nodes
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn basic_grid() {
        let mut test = vec![
            HashSet::from_iter(vec![1, 3]),
            HashSet::from_iter(vec![0, 2, 4]),
            HashSet::from_iter(vec![1, 5]),
            HashSet::from_iter(vec![0, 4]),
            HashSet::from_iter(vec![1, 3, 5]),
            HashSet::from_iter(vec![2, 4]),
        ];

        let grid = Graph::new_grid(3, 2);

        assert_eq!(grid.as_ref(), &test);
    }
}