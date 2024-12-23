import re

class Day3:
    def part1(input):
        pattern = r'mul\((\d+),(\d+)\)'
        matches = re.findall(pattern, input)
        return sum(map(lambda pair: int(pair[0])*int(pair[1]), matches))
    
    def part2(input):
        pattern = r"don\'t\(\).+?do\(\)?"
        input = input.replace('\n', '') + 'do()'
        matches = re.findall(pattern, input)
        for m in matches:
            input = input.replace(m, "")
        pattern = r'mul\((\d+),(\d+)\)'
        matches = re.findall(pattern, input)
        return sum(map(lambda pair: int(pair[0])*int(pair[1]), matches))
