from functools import cmp_to_key


def parse(input):
    data = input.split('\n\n')
    rules = {}
    for r in data[0].split():
        pair = [n for n in r.split('|')]
        rules.setdefault(pair[0], set()).add(pair[1])
        rules.setdefault(pair[1], set())
    return (rules, [[n for n in u.split(',')] for u in data[1].split()])


class Day5:

    def part1(input):
        rules, updates = parse(input)

        def only_correct(update):
            for i in reversed(range(len(update)-1)):
                for j in range(i+1, len(update)):
                    if update[i] in rules[update[j]]:
                        return 0
            return int(update[len(update)//2])

        return sum([only_correct(u) for u in updates])

    def part2(input):
        rules, updates = parse(input)

        def sort(update):
            for i in reversed(range(len(update)-1)):
                for j in range(i+1, len(update)):
                    if update[i] in rules[update[j]]:
                        sorted_update = sorted(update, key=cmp_to_key(
                            lambda a, b: (b in rules[a]) - (a in rules[b])))
                        return int(sorted_update[len(update)//2])
            return 0

        return sum([sort(u) for u in updates])
