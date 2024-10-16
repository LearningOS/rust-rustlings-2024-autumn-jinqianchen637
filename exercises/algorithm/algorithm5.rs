/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

//I AM DONE
use std::collections::VecDeque;
use std::collections::HashSet;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visit_order = vec![];
        let mut visit_set = HashSet::new();
        let mut task_vdq: VecDeque<usize> = VecDeque::new();
        task_vdq.push_back(start);
        let mut flag_not_finish = true;
        while flag_not_finish {
            let one_point = task_vdq.pop_front();
            match one_point {
                None => { 
                    flag_not_finish = false;
                    continue;
                },
                Some(point) => {
                    if visit_set.contains(&point) {
                        continue;
                    }else {
                        visit_set.insert(point);
                    }
                    visit_order.push(point);
                    for sub_points in &self.adj[point] {
                        task_vdq.push_back(*sub_points);
                    }
                }
            }
        }
        visit_order
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

