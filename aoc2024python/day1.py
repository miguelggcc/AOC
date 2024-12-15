class Day1:
    def part1(input):
        data =  [*map(int,input.split())]
        left = sorted(data[0::2])
        right = sorted(data[1::2])
        return sum(map(lambda a, b: abs(a - b), left, right))
            
    def part2(input):
        data =  [*map(int,input.split())]
        count = {}
        for b in  data[1::2]:
            count[b] = count.setdefault(b, 0) + 1
        return sum(a*count.get(a,0) for a in data[0::2])

