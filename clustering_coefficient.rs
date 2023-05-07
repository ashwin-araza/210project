
pub fn clustering_coefficient(edges: &[(usize, usize)]) -> f64 {
    let n = edges.iter().flat_map(|&(u, v)| vec![u, v]).max().unwrap() + 1;
    let mut adj_list = vec![Vec::new(); n];

    for &(u, v) in edges.iter() {
        adj_list[u].push(v);
        adj_list[v].push(u);
    }

    let mut clustering_coeffs = Vec::new();
    for i in 0..n {
        let adj_i = &adj_list[i];
        let k = adj_i.len();
        if k < 2 {
            continue;
        }
        let mut num_triangles = 0;
        for j in 0..k {
            let adj_j = &adj_list[adj_i[j]];
            for m in j+1..k {
                if adj_j.binary_search(&adj_i[m]).is_ok() {
                    num_triangles += 1;
                }
            }
        }
        let num_possible_triangles = k * (k - 1) / 2;
        let cc_i = if num_possible_triangles > 0 {
            num_triangles as f64 / num_possible_triangles as f64
        } else {
            0.0
        };
        clustering_coeffs.push(cc_i);
    }

    if clustering_coeffs.is_empty() {
        0.0
    } else {
        clustering_coeffs.iter().sum::<f64>() / clustering_coeffs.len() as f64
    }
}