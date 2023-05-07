use std::collections::VecDeque;
pub fn bfs_distance(adj_list: &[Vec<usize>], start_node: usize, end_node: usize) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = vec![false; adj_list.len()];
    let mut distances = vec![0; adj_list.len()];

    visited[start_node] = true;
    queue.push_back(start_node);

    while let Some(current_node) = queue.pop_front() {
        if current_node == end_node {
            return Some(distances[end_node]);
        }

        for neighbor in &adj_list[current_node] {
            if !visited[*neighbor] {
                visited[*neighbor] = true;
                distances[*neighbor] = distances[current_node] + 1;
                queue.push_back(*neighbor);

                if *neighbor == end_node {
                    return Some(distances[end_node]);
                }
            }
        }
    }

    None
}