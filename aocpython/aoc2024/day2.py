def is_safe(report):
    diff = [j-i for i, j in zip(report[:-1], report[1:])]
    return all(d > 0 and d < 4 for d in diff) or all(d < 0 and d > -4 for d in diff)


class Day2:

    def part1(input):
        return sum(is_safe([*map(int, line.split())]) for line in input.splitlines())

    def part2(input):
        def safe_removed(l):
            return any(is_safe(l[:i]+l[i+1:]) for i in range(len(l)))
        return sum(safe_removed([*map(int, line.split())]) for line in input.splitlines())
