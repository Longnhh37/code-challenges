use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        fn dfs<'a>(graph: &mut HashMap<&'a str, Vec<&'a str>>, dest: &'a str, path: &mut Vec<&'a str>) {
            while let Some(next) = graph.get_mut(dest).and_then(Vec::pop) {
                dfs(graph, next, path);
            }
            path.push(dest);
        }

        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        for t in &tickets {
            graph.entry(t[0].as_str()).or_insert_with(Vec::new).push(t[1].as_str());
        }

        for dests in graph.values_mut() {
            dests.sort_unstable_by(|a, b| b.cmp(a));
        }

        let mut path = Vec::new();
        dfs(&mut graph, "JFK", &mut path);

        path.iter().rev().map(|s| s.to_string()).collect()
    }
}
