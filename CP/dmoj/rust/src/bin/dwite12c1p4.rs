use std::io::Read;
use std::iter::Peekable;
use std::str::Chars;

// ==================================================================
// Define Node
// ==================================================================

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Self { value, left, right }
    }
}

// ==================================================================
// Main
// ==================================================================
fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    for line in input.lines() {
        println!("{}", solve(line));
    }
}

fn solve(line: &str) -> String {
    let tree = build_tree(line);
    let candy = get_total_candy(&tree);
    let min_roads = min_walk(&tree);
    format!("{} {}", min_roads, candy)
}

// ==================================================================
// Parse and build tree
// ==================================================================
fn build_tree(line: &str) -> Node {
    let mut chars = line.chars().peekable();
    parse_node(&mut chars)
}

fn parse_node(it: &mut Peekable<Chars>) -> Node {
    skip_ws(it);
    match it.peek() {
        Some('(') => {
            it.next();
            let left = parse_node(it);
            let right = parse_node(it);
            skip_ws(it);
            if it.peek() == Some(&')') {
                it.next();
            }
            Node::new(0, Some(Box::new(left)), Some(Box::new(right)))
        }
        _ => {
            let mut num = String::new();
            while let Some(&c) = it.peek() {
                if c.is_ascii_digit() {
                    num.push(c);
                    it.next();
                } else {
                    break;
                }
            }
            let value: i32 = num.parse().unwrap();
            Node::new(value, None, None)
        }
    }
}

fn skip_ws(it: &mut Peekable<Chars>) {
    while let Some(&' ') = it.peek() {
        it.next();
    }
}

// ==================================================================
// find total candy
// ==================================================================

fn get_total_candy(root: &Node) -> i32 {
    let mut res = 0;

    fn dfs(node: &Node, res: &mut i32) {
        *res += node.value;

        if let Some(l) = &node.left {
            dfs(l, res);
        }
        if let Some(r) = &node.right {
            dfs(r, res);
        }
    }

    dfs(root, &mut res);
    res
}

// ==================================================================
// find minimum walk
// ==================================================================
fn min_walk(root: &Node) -> i32 {
    let (e, h) = edges_and_height(root);
    2 * e - h
}

fn edges_and_height(node: &Node) -> (i32, i32) {
    match (&node.left, &node.right) {
        (None, None) => (0, 0),
        (Some(l), Some(r)) => {
            let (el, hl) = edges_and_height(l);
            let (er, hr) = edges_and_height(r);

            let e = el + er + 2;
            let h = 1 + hl.max(hr);

            (e, h)
        }
        _ => unreachable!("this is a full binary tree"),
    }
}
