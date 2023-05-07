
// File: triangle_counter.rs

pub fn count_triangles(edges: &[(usize, usize)]) -> usize {
    let mut adj_list = vec![Vec::new(); edges.len()];
    for (i, &(u, v)) in edges.iter().enumerate() {
        adj_list[u].push(i);
        adj_list[v].push(i);
    }

    let mut num_triangles = 0;
    for &(u, v) in edges.iter() {
        let mut common_neighbors = Vec::new();
        for &w in &adj_list[u] {
            if edges[w].1 == v {
                common_neighbors.push(edges[w].0);
                common_neighbors.push(v);
                break;
            } else {
                common_neighbors.push(edges[w].1);
            }
        }
        for &w in &adj_list[v] {
            if edges[w].1 != u && common_neighbors.binary_search(&edges[w].1).is_ok() {
                num_triangles += 1;
            }
        }
    }

    // Since we count each triangle 3 times (once for each of its 3 vertices), we need to divide by 3
    num_triangles / 3
}

pub fn frac_closed_triangles(edges: &[(usize, usize)]) -> f64 {
    let mut num_closed_triangles = 0;
    let mut num_triangles = 0;

    let mut adj_list = vec![Vec::new(); edges.len()];
    for (i, &(u, v)) in edges.iter().enumerate() {
        adj_list[u].push(i);
        adj_list[v].push(i);
    }

    for &(u, v) in edges.iter() {
        let mut common_neighbors = Vec::new();
        for &w in &adj_list[u] {
            if edges[w].1 == v {
                common_neighbors.push(edges[w].0);
                common_neighbors.push(v);
                break;
            } else {
                common_neighbors.push(edges[w].1);
            }
        }
        for &w in &adj_list[v] {
            if edges[w].1 != u && common_neighbors.binary_search(&edges[w].1).is_ok() {
                num_triangles += 1;
                if common_neighbors.binary_search(&edges[w].0).is_ok() {
                    num_closed_triangles += 1;
                }
            }
        }
    }

    if num_triangles == 0 {
        0.0
    } else {
        num_closed_triangles as f64 / num_triangles as f64
    }
}
