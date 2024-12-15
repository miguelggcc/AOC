from functools import reduce
from operator import mul, add
from itertools import product
from functools import cache


def concat(a, b):
    return int(f"{b}{a}")


@cache
def mul_add(n):
    return list(product([mul, add], repeat=n))


@cache
def mul_add_concat(n):
    return list(product([mul, add, concat], repeat=n))


def solve(input, operations):
    count = 0
    for line in input.splitlines():
        nums = [*map(int, line.replace(":", " ").split())]
        all_ops = operations(len(nums)-2)
        if any(nums[0] == reduce(lambda acc, op_and_num: op_and_num[0](op_and_num[1], acc), zip(ops, nums[2:]), nums[1])for ops in all_ops):
            count += nums[0]
    return count


class Day7:

    def part1(input):
        return solve(input, mul_add)

    def part2(input):
        return solve(input, mul_add_concat)
