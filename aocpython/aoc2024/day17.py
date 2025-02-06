class Day17:
    def part1(input):
        r,p = input.split('\n\n')
        reg = [int(s[12:]) for s in r.splitlines()]
        print(reg[0])
        program = [int(n) for n in p[9:].split(',')]
        out = []
        i = 0
        
        def combo(op):
            if op > 3:
                op = reg[op-4]
            return op
    
        
        while i+1 < len(program):
            op = program[i+1]
            
            match program[i]:
                case 0:
                    reg[0] = reg[0]>>combo(op)
                case 1:
                    reg[1]^=op
                case 2:
                    reg[1] = combo(op)%8
                case 3:
                    if reg[0]!=0:
                        i = op-2
                case 4:
                    reg[1]^=reg[2]
                case 5:
                    out.append(str(combo(op)%8))
                case 6:
                    reg[1] = reg[0]>>combo(op)
                case 7:
                    reg[2] = reg[0]>>combo(op)
            i+=2
            print(format(reg[0], 'b'))
        print([(format(int(o), 'b')) for o in out])
        return ",".join(out)

    def part2(input):
        return "Not implemented"
 