use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> impl std::fmt::Display {
    karger_algorithm(input)
}

pub fn part2(_input: &str) -> impl std::fmt::Display {
    "Not implemented"
}

fn karger_algorithm(input: &str) -> usize {
    let mut nodes1: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut edges1 = HashSet::new();
    for l in input.lines() {
        let (left, right) = l.split_once(": ").unwrap();
        for n in right.split_ascii_whitespace() {
            edges1.insert((n, left).min((left, n)));
            nodes1.entry(left).or_insert_with(HashSet::new).insert(n);
            nodes1.entry(n).or_insert_with(HashSet::new).insert(left);
        }
    }
    let mut edges = edges1.clone();
    let mut nodes = nodes1.clone();
    while edges.len() != 3 {
        edges = edges1.clone();
        nodes = nodes1.clone();
        while nodes.len() > 2 {
            //dbg!(edges.len(),nodes.len());
            let (u, v) = edges.iter().next().unwrap().clone();
            dbg!(u, v);
            if let Some(v_set) = nodes.remove(&v) {
                for &w in &v_set {
                    if w != u {
                        nodes.get_mut(u).unwrap().insert(w);
                        if let Some(w_set) = nodes.get_mut(&w) {
                            w_set.insert(u);
                            w_set.remove(&v);
                            edges.insert((u, w).min((w, u)));
                        }
                    }
                }
            }
            edges.retain(|&(a, b)| a != v && b != v);
        }
        //dbg!(&nodes,&edges);
    }
    nodes.into_values().map(|set| set.len()).product()
}

#[cfg(test)]
mod day25 {

    use super::*;

    const INPUT: &'static str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    #[ignore]
    fn part_1() {
        assert_eq!(part1(INPUT).to_string(), "54");
    }
}
