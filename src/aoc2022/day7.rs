use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete,
    combinator::{all_consuming, map},
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

pub fn part1(input: &str) -> u64 {
    let mut lines = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);
    let root_dir = get_data(&mut lines, String::from("root"));
    get_size(&root_dir)
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input
        .lines()
        .map(|line| all_consuming(parse_line)(line).finish().unwrap().1);

    let root_dir = get_data(&mut lines, String::from("root"));
    let free_space = 70000000 - root_dir.total_size;
    let to_delete = 30000000 - free_space;
    get_size_to_delete(&root_dir, to_delete)
}

fn get_data(lines: &mut impl Iterator<Item = Line>, name: String) -> Dir {
    let mut dir = Dir {
        name,
        total_size: 0,
        files: vec![],
        directories: vec![],
    };
    loop {
        match lines.next() {
            Some(Line::Command(command)) => match command {
                Command::Cd(Cd::Up(d)) => {
                    let sub_dir = get_data(lines, d);
                    dir.total_size += sub_dir.total_size;
                    dir.directories.push(sub_dir);
                }
                Command::Cd(Cd::Down) => break,
                _ => (),
            },

            Some(Line::File(file)) => {
                dir.total_size += file.size;
                dir.files.push(file);
            }
            None => break,
            _ => (),
        }
    }
    dir
}

const MAX_SIZE: u64 = 100000;
fn get_size(dir: &Dir) -> u64 {
    let size = if dir.total_size <= MAX_SIZE {
        //Only sum those under 100000 in size
        dir.total_size
    } else {
        0
    };

    size + dir.directories.iter().map(get_size).sum::<u64>()
}

fn get_size_to_delete(dir: &Dir, value: u64) -> u64 {
    dir.directories
        .iter()
        .map(|d| get_size_to_delete(d, value))
        .filter(|s| s > &value)
        .min()
        .map_or(dir.total_size, |s| s)
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Directory,
    File(File),
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Cd),
}

#[derive(Debug)]
enum Cd {
    Up(String),
    Down,
    Bar,
}

fn ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("ls")(input)?;
    Ok((input, Command::Ls))
}

fn cd(input: &str) -> IResult<&str, Command> {
    let (input, c) = preceded(tag("cd "), is_a("qwertyuiopasdfghjklzxcvbnm./"))(input)?;
    let cd = match c {
        "/" => Cd::Bar,
        ".." => Cd::Down,
        _ => Cd::Up(c.to_string()),
    };
    Ok((input, Command::Cd(cd)))
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$ ")(input)?;
    let (input, command) = alt((ls, cd))(input)?;
    Ok((input, command))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        map(parse_command, Line::Command),
        map(parse_directory, |_s| Line::Directory),
        map(parse_file, Line::File),
    ))(input)
}

fn parse_directory(input: &str) -> IResult<&str, String> {
    let (input, d) = preceded(tag("dir "), is_a("qwertyuiopasdfghjklzxcvbnm."))(input)?;
    Ok((input, d.to_string()))
}

fn parse_file(input: &str) -> IResult<&str, File> {
    let (input, (size, name)) =
        separated_pair(complete::u64, tag(" "), is_a("qwertyuiopasdfghjklzxcvbnm."))(input)?;
    let file = File {
        name: name.to_string(),
        size,
    };
    Ok((input, file))
}

#[allow(dead_code)]
#[derive(Debug)]
struct Dir {
    name: String,
    total_size: u64,
    files: Vec<File>,
    directories: Vec<Dir>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_2() {
        let input = "$ cd /
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

        assert_eq!(part1(input), 95437);
        assert_eq!(part2(input), 24933642);
    }
}
