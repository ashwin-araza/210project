use std::fs::File;
use std::io::{BufRead, BufReader};

mod triangle_counter;
mod clustering_coefficient;
mod distance;

fn main() {
    let file = File::open("roadNet-CA.txt").unwrap();
    let reader = BufReader::new(file);

    let mut edges = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<_> = line.split_whitespace().collect();
        let from = parts[0].parse::<usize>().unwrap();
        let to = parts[1].parse::<usize>().unwrap();
        edges.push((from, to));
    }

    let num_triangles = triangle_counter::count_triangles(&edges);
    let frac_closed_triangles = triangle_counter::frac_closed_triangles(&edges);
    let clustering_coefficient = clustering_coefficient::clustering_coefficient(&edges);
    let distance = distance::bfs_distance(&make_adj_list(&edges), 0, 10000);

    println!("Number of triangles: {}", num_triangles);
    println!("Fraction of closed triangles: {}", frac_closed_triangles);
    println!("Clustering coefficient: {}", clustering_coefficient);
    match distance {
        Some(dist) => println!("Distance between nodes 0 and 100000: {}", dist),
        None => println!("No path found between nodes 0 and 100000."),
    }
}

fn make_adj_list(edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let max_node = edges.iter().flat_map(|&(u, v)| vec![u, v]).max().unwrap();
    let mut adj_list = vec![Vec::new(); max_node + 1];

    for &(u, v) in edges {
        adj_list[u].push(v);
        adj_list[v].push(u);
    }

    adj_list
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use crate::distance::bfs_distance;

    fn get_test_file_path() -> PathBuf {
        let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src");
        path.push("roadNet-CA.txt");
        path
    }

    #[test]
    fn test_count_triangles() {
        let file_path = get_test_file_path();
        let file = File::open(&file_path).unwrap();
        let reader = BufReader::new(file);

        let mut edges = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<_> = line.split_whitespace().collect();
            let from = parts[0].parse::<usize>().unwrap();
            let to = parts[1].parse::<usize>().unwrap();
            edges.push((from, to));
        }

        let num_triangles = triangle_counter::count_triangles(&edges);
        assert_eq!(num_triangles, 3497009);
    }

    #[test]
    fn test_frac_closed_triangles() {
        let file_path = get_test_file_path();
        let file = File::open(&file_path).unwrap();
        let reader = BufReader::new(file);

        let mut edges = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<_> = line.split_whitespace().collect();
            let from = parts[0].parse::<usize>().unwrap();
            let to = parts[1].parse::<usize>().unwrap();
            edges.push((from, to));
        }

        let frac_closed_triangles = triangle_counter::frac_closed_triangles(&edges);
        assert_eq!(frac_closed_triangles, 0.26372746919820145);
    }

    #[test]
    fn test_clustering_coefficient() {
        let file_path = get_test_file_path();
        let file = File::open(&file_path).unwrap();
        let reader = BufReader::new(file);

        let mut edges = Vec::new();
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<_> = line.split_whitespace().collect();
            let from = parts[0].parse::<usize>().unwrap();
            let to = parts[1].parse::<usize>().unwrap();
            edges.push((from, to));
        }

        let clustering_coefficient = clustering_coefficient::clustering_coefficient(&edges);
        assert_eq!(clustering_coefficient, 0.036593827741341865);
    }
    #[test]
    fn test_bfs_distance() {
        let file_path = get_test_file_path();
        let file = File::open(&file_path).unwrap();
        let reader = BufReader::new(file);
    
        let mut adj_list = vec![Vec::new(); 197128];
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<_> = line.split_whitespace().collect();
            let from = parts[0].parse::<usize>().unwrap();
            let to = parts[1].parse::<usize>().unwrap();
            if from >= adj_list.len() || to >= adj_list.len() {
                let new_len = std::cmp::max(from, to) + 1;
                adj_list.resize_with(new_len, || Vec::new());
            }
            adj_list[from].push(to);
            adj_list[to].push(from);
        }
    
        let start_node = 0;
        let end_node = 100000;
        let expected_distance = 105;
    
        let distance = bfs_distance(&adj_list, start_node, end_node).unwrap();
    
        assert_eq!(distance, expected_distance);
    }
}






