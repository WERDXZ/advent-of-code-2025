use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    hash::Hash,
    io::{BufRead, stdin},
    rc::Rc,
    str::FromStr,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node(u64, u64, u64);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Edge(usize, usize, u64);

#[derive(Debug, Clone, Eq)]
struct Record(Rc<RefCell<u64>>);

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Hash for Record {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(',').collect();
        if parts.len() != 3 {
            return Err(());
        }
        let x = parts[0].parse::<u64>().map_err(|_| ())?;
        let y = parts[1].parse::<u64>().map_err(|_| ())?;
        let z = parts[2].parse::<u64>().map_err(|_| ())?;
        Ok(Node(x, y, z))
    }
}

impl Node {
    fn squared_distance(&self, other: &Node) -> u64 {
        let dx = self.0 as i64 - other.0 as i64;
        let dy = self.1 as i64 - other.1 as i64;
        let dz = self.2 as i64 - other.2 as i64;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.2.cmp(&other.2)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input: Vec<Node> = stdin()
        .lock()
        .lines()
        .map_while(Result::ok)
        .map(|v| v.parse().unwrap())
        .collect();

    let mut edges = input
        .iter()
        .enumerate()
        .flat_map(|(i, node)| {
            input[(i + 1)..]
                .iter()
                .enumerate()
                .map(move |(j, v)| Edge(i, i + 1 + j, node.squared_distance(v)))
        })
        .collect::<Vec<Edge>>();

    edges.sort();

    let mut map: HashMap<usize, Rc<RefCell<u64>>> = HashMap::new();
    let mut rcs: HashSet<Record> = HashSet::new();
    let mut last_merge: Option<(usize, usize)> = None;
    let n = input.len();

    for edge in &edges {
        let rc1 = map.get(&edge.0).cloned();
        let rc2 = map.get(&edge.1).cloned();

        let merged = match (rc1, rc2) {
            (Some(rc1), Some(rc2)) => {
                if !Rc::ptr_eq(&rc1, &rc2) {
                    *rc1.borrow_mut() += *rc2.borrow();
                    for v in map.values_mut() {
                        if Rc::ptr_eq(v, &rc2) {
                            *v = rc1.clone();
                        }
                    }
                    rcs.remove(&Record(rc2));
                    true
                } else {
                    false
                }
            }
            (Some(rc), None) => {
                *rc.borrow_mut() += 1;
                map.insert(edge.1, rc);
                true
            }
            (None, Some(rc)) => {
                *rc.borrow_mut() += 1;
                map.insert(edge.0, rc);
                true
            }
            (None, None) => {
                let rc = Rc::new(RefCell::new(2));
                rcs.insert(Record(rc.clone()));
                map.insert(edge.0, rc.clone());
                map.insert(edge.1, rc);
                true
            }
        };

        if merged {
            last_merge = Some((edge.0, edge.1));
        }

        // MST complete when we have 1 component with all nodes
        if rcs.len() == 1 && map.len() == n {
            break;
        }
    }

    let (i, j) = last_merge.unwrap();
    println!("{}", input[i].0 * input[j].0);
}
