from functools import cache


def parse(input):
    p, o = input.split('\n\n')
    return p.split(', '),  o.splitlines()


class Day19:

    def part1(input):
        patterns, onsen = parse(input)

        @cache
        def match_pattern(word):
            for p in patterns:
                if word.startswith(p) and match_pattern(word.removeprefix(p)):
                    return True
            return not word
        return sum(map(match_pattern, onsen))

    def part2(input):
        patterns, onsen = parse(input)

        @cache
        def match_pattern(word):
            ans = sum(match_pattern(word.removeprefix(p))
                      for p in patterns if word.startswith(p))

            return ans + int(not word)
        return sum(map(match_pattern, onsen))
