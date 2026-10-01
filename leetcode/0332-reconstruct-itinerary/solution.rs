use std::collections::HashMap;

impl Solution {
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        for t in &tickets {
            graph.entry(t[0].as_str()).or_default().push(t[1].as_str());
        }

        for dests in graph.values_mut() {
            dests.sort_unstable_by(|a, b| b.cmp(a));
        }

        let mut route = Vec::with_capacity(tickets.len() + 1);
        let mut stack = vec!["JFK"];

        while let Some(&airport) = stack.last() {
            if let Some(next) = graph.get_mut(airport).and_then(Vec::pop) {
                stack.push(next);
            } else {
                route.push(stack.pop().unwrap());
            }
        }

        route.reverse();
        route.into_iter().map(String::from).collect()
    }
}
