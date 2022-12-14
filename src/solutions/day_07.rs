use super::super::tree::*;

const INPUT: &str = include_str!("day_07.txt");

fn parse_input(input: &str) -> FSNode {
    let root = FSNode::new(String::from("/"));
    let mut current = root.zipper();
    let mut lines = input.lines();

    lines.next(); // Skip the first line since it is `cd /`
    for command in lines {
        match command {
            s if s.starts_with("$ cd ..") => current = current.parent(),
            s if s.starts_with("$ cd") => {
                let child_path = s.strip_prefix("$ cd ").unwrap();
                current = current.child(&String::from(child_path)).unwrap()
            }
            s if s.starts_with("$ ls") => (),
            s if s.starts_with("dir") => {
                let new_dir_path = s.strip_prefix("dir ").unwrap();
                current = current.add_child(FSNode::new(String::from(new_dir_path)));
            }
            s if s.starts_with(char::is_numeric) => {
                let mut parts = s.split(' ');
                let size = parts.next().unwrap().parse().unwrap();
                let name = String::from(parts.next().unwrap());
                current = current.add_child(FSNode::File(name, size))
            }
            _ => unreachable!("Command {} has unknown format", command),
        };
    }
    current.finish()
}

fn nodes_below_size(start: &FSNode, size: usize) -> Vec<&FSNode> {
    all_nodes(start)
        .iter()
        .filter(|&&node| node.size_below() < size && node.is_directory())
        .copied()
        .collect()
}

fn all_nodes(start: &FSNode) -> Vec<&FSNode> {
    let mut result = vec![];
    start.visit_nodes(|node| result.push(node));
    result
}

fn smallest_directory_that_fits(start: &FSNode, minimum_space_required: usize) -> usize {
    all_nodes(start)
        .iter()
        .filter_map(|node| {
            let size = node.size_below();
            if size >= minimum_space_required && node.is_directory() {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .unwrap()
}

pub fn print_solution() {
    let filesystem = parse_input(INPUT);
    let deletion_candidates = nodes_below_size(&filesystem, 100_000);
    let freed_space: usize = deletion_candidates.iter().map(|&n| n.size_below()).sum();
    println!("Amount of space that can be freed: {freed_space}");
    let total_space = 70_000_000;
    let space_required = 30_000_000;
    let free_space = total_space - filesystem.size_below();
    let space_to_free = space_required - free_space;
    let smallest_dir = smallest_directory_that_fits(&filesystem, space_to_free);
    println!("Smallest directory that frees enough space: {smallest_dir}");
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::collections::HashMap;

    const SAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    const SHORT_SAMPLE: &str = "$ cd /
$ ls
14848514 b.txt
8504156 c.dat
dir d
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    fn s(input: &str) -> String {
        String::from(input)
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(SHORT_SAMPLE),
            FSNode::Node(
                s("/"),
                HashMap::from([
                    (s("b.txt"), FSNode::File(s("b.txt"), 14848514)),
                    (s("c.dat"), FSNode::File(s("c.dat"), 8504156)),
                    (
                        s("d"),
                        FSNode::Node(
                            s("d"),
                            HashMap::from([
                                (s("j"), FSNode::File(s("j"), 4060174)),
                                (s("d.log"), FSNode::File(s("d.log"), 8033020)),
                                (s("d.ext"), FSNode::File(s("d.ext"), 5626152)),
                                (s("k"), FSNode::File(s("k"), 7214296))
                            ])
                        )
                    ),
                ])
            )
        )
    }

    #[test]
    fn test_size_below() {
        let nodes = parse_input(SAMPLE);
        assert_eq!(nodes.size_below(), 48381165)
    }

    #[test]
    fn test_visit() {
        let mut node_names = vec![];
        parse_input(SHORT_SAMPLE).visit_nodes(|node| node_names.push(node.name()));
        node_names.sort();
        assert_eq!(
            node_names,
            vec![
                s("/"),
                s("b.txt"),
                s("c.dat"),
                s("d"),
                s("d.ext"),
                s("d.log"),
                s("j"),
                s("k")
            ]
        )
    }

    #[test]
    fn test_nodes_below_size() {
        let input = parse_input(SAMPLE);
        let below_100k = nodes_below_size(&input, 100000);
        assert_eq!(
            vec![s("a"), s("e")],
            below_100k
                .iter()
                .map(|&n| n.name())
                .collect::<Vec<String>>(),
        )
    }

    #[test]
    fn test_smallest_dir() {
        let input = parse_input(SAMPLE);
        assert_eq!(smallest_directory_that_fits(&input, 8381165), 24933642);
    }
}
