def parse(input):
    return [
        [int(c) for c in m.translate(str.maketrans(
            {'+': ' ', '=': ' ', ',': ' ', '\n': ' '})).split() if c.isdigit()]
        for m in input.split('\n\n')
    ]


def intersection(a1, a2, b1, b2, c1, c2):
    x = (b1*c2-b2*c1)/(-b2*a1+b1*a2)
    y = (-a1*x+c1)/b1
    return (x, y)


class Day13:
    def part1(input):
        machines = parse(input)
        return sum(
            int(3 * a + b)
            for m in machines
            for a, b in [intersection(*m)]
            if a >= 0 and b >= 0 and a.is_integer() and b.is_integer()
        )

    def part2(input):
        delta = 10000000000000
        machines = parse(input)
        return sum(
            int(3 * a + b)
            for m in machines
            for a, b in [intersection(*m[:-2], m[-2] + delta, m[-1] + delta)]
            if a >= 0 and b >= 0 and a.is_integer() and b.is_integer()
        )
