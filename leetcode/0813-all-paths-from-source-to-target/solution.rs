
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn backtrack(graph: &[Vec<i32>], u: usize, path: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            path.push(u as i32);

            if u == graph.len() - 1 {
                res.push(path.clone());
            } else {
                for &v in &graph[u] {
                    backtrack(graph, v as usize, path, res);
                }
            }

            path.pop();
        }

        let mut res: Vec<Vec<i32>> = Vec::new();
        backtrack(&graph, 0, &mut Vec::new(), &mut res);
        res
    }
}
