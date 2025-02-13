from functools import cache


def parse(input):
    p, o = input.split('\n\n')
    return p.split(', '),  o.splitlines()


class Day19:

    def part1(input):
        patterns, onsen = parse(input)

        @cache
        def match_pattern(word):
            return not word or any(match_pattern(word.removeprefix(p))
                                   for p in patterns if word.startswith(p))
        return sum(map(match_pattern, onsen))

    def part2(input):
        patterns, onsen = parse(input)

        @cache
        def match_pattern(word):
            return sum(match_pattern(word.removeprefix(p))
                       for p in patterns if word.startswith(p)) + int(not word)
        return sum(map(match_pattern, onsen))
