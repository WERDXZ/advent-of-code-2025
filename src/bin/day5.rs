use std::io::{BufRead, stdin};
use std::ops::RangeInclusive;

#[derive(Debug)]
struct IntervalTree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    pivot: u64,
    // Ranges containing the pivot, sorted by start (for left traversal)
    ranges: Vec<RangeInclusive<u64>>,
    left: Option<Box<Node>>,  // ranges entirely < pivot
    right: Option<Box<Node>>, // ranges entirely > pivot
}

impl IntervalTree {
    fn new() -> Self {
        IntervalTree { root: None }
    }

    /// Build tree from a list of ranges
    fn build(ranges: Vec<RangeInclusive<u64>>) -> Self {
        if ranges.is_empty() {
            return IntervalTree { root: None };
        }
        IntervalTree {
            root: Some(Box::new(Self::build_node(ranges))),
        }
    }

    fn build_node(ranges: Vec<RangeInclusive<u64>>) -> Node {
        // Find pivot as median of all endpoints
        let mut endpoints: Vec<u64> = ranges.iter().flat_map(|r| [*r.start(), *r.end()]).collect();
        endpoints.sort_unstable();
        let pivot = endpoints[endpoints.len() / 2];

        let mut containing = Vec::new(); // ranges containing pivot
        let mut left_ranges = Vec::new(); // ranges entirely left of pivot
        let mut right_ranges = Vec::new(); // ranges entirely right of pivot

        for range in ranges {
            if *range.end() < pivot {
                left_ranges.push(range);
            } else if *range.start() > pivot {
                right_ranges.push(range);
            } else {
                containing.push(range);
            }
        }

        Node {
            pivot,
            ranges: containing,
            left: if left_ranges.is_empty() {
                None
            } else {
                Some(Box::new(Self::build_node(left_ranges)))
            },
            right: if right_ranges.is_empty() {
                None
            } else {
                Some(Box::new(Self::build_node(right_ranges)))
            },
        }
    }

    /// Find all intervals that overlap with a query point
    fn query_point(&self, point: u64) -> Vec<&RangeInclusive<u64>> {
        let mut result = Vec::new();
        Self::query_point_node(&self.root, point, &mut result);
        result
    }

    fn query_point_node<'a>(
        node: &'a Option<Box<Node>>,
        point: u64,
        result: &mut Vec<&'a RangeInclusive<u64>>,
    ) {
        let Some(n) = node else { return };

        // Add all ranges at this node that contain the point
        for range in &n.ranges {
            if range.contains(&point) {
                result.push(range);
            }
        }

        // Recurse into appropriate subtree
        if point < n.pivot {
            Self::query_point_node(&n.left, point, result);
        } else if point > n.pivot {
            Self::query_point_node(&n.right, point, result);
        }
    }

    /// Find all intervals that overlap with a query range
    fn query_range(&self, query: &RangeInclusive<u64>) -> Vec<&RangeInclusive<u64>> {
        let mut result = Vec::new();
        Self::query_range_node(&self.root, query, &mut result);
        result
    }

    fn query_range_node<'a>(
        node: &'a Option<Box<Node>>,
        query: &RangeInclusive<u64>,
        result: &mut Vec<&'a RangeInclusive<u64>>,
    ) {
        /// Check if two ranges overlap
        fn ranges_overlap(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
            a.start() <= b.end() && b.start() <= a.end()
        }
        let Some(n) = node else { return };

        // Check ranges at this node
        for range in &n.ranges {
            if ranges_overlap(range, query) {
                result.push(range);
            }
        }

        // Check left subtree if query extends left of pivot
        if *query.start() < n.pivot {
            Self::query_range_node(&n.left, query, result);
        }

        // Check right subtree if query extends right of pivot
        if *query.end() > n.pivot {
            Self::query_range_node(&n.right, query, result);
        }
    }
}

fn main() {
    let ranges: Vec<RangeInclusive<u64>> = stdin()
        .lock()
        .lines()
        .map_while(|line| {
            line.ok().and_then(|line| {
                line.split_once('-').and_then(|(start, end)| {
                    start
                        .parse::<u64>()
                        .ok()
                        .and_then(|start| end.parse::<u64>().ok().map(|end| start..=end))
                })
            })
        })
        .collect();

    let tree = IntervalTree::build(ranges);

    let queries: u64 = stdin()
        .lock()
        .lines()
        .map_while(|line| line.ok().and_then(|line| line.parse::<u64>().ok()))
        .map(|v| if tree.query_point(v).is_empty() { 0 } else { 1 })
        .sum();

    println!("{}", queries);
}
