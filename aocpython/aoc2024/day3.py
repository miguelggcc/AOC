import re


def find_pairs(input):
    pattern = r'mul\((\d+),(\d+)\)'
    matches = re.findall(pattern, input)
    return sum(map(lambda pair: int(pair[0])*int(pair[1]), matches))


class Day3:
    def part1(input):
        return find_pairs(input)

    def part2(input):
        pattern = r"don\'t\(\).+?do\(\)?"
        input = input.replace('\n', '') + 'do()'
        matches = re.findall(pattern, input)
        for m in matches:
            input = input.replace(m, "")
        return find_pairs(input)
