from functools import cache


@cache
def blink(stone, i):
    if i == 0:
        return 1
    if stone == 0:
        return blink(1, i-1)
    if len(s:=str(stone)) % 2 == 0:
        h = len(s)//2
        return blink(int(s[:h]), i-1)+blink(int(s[h:]), i-1)
    return blink(stone*2024, i-1)


class Day11:
    def part1(input):
        return sum(blink(int(s), 25) for s in input.split())

    def part2(input):
        return sum(blink(int(s), 75) for s in input.split())
