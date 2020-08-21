if __name__ == '__main__':
    with open("input.txt", "r") as f:
        ops = f.read().split(',')
        ops[-1] = ops[-1].split('\n')[0]
        ops = [int(i) for i in ops]

    pos = 0
    while (ops[pos] != 99):
        if ops[pos] == 1:
            ops[ops[pos+3]] = ops[ops[pos+1]] + ops[ops[pos+2]]
            pos += 4

        elif ops[pos] == 2:
            ops[ops[pos+3]] = ops[ops[pos+1]] * ops[ops[pos+2]]
            pos += 4

        elif ops[pos] == 3:
            print("Opcode 3 encountered. Enter input.")
            x = int(input())
            ops[ops[pos+1]] = x
            pos += 2

        elif ops[pos] == 4:
            print(ops[ops[pos+1]])
            pos += 2

        # Parameter modes
        else:
            arithmetic_op = ops[pos] % 100
            firstp_mode = (ops[pos] // 100) % 10
            firstp = ops[pos+1] if firstp_mode else ops[ops[pos+1]]

            if arithmetic_op <= 2:
                secondp_mode = (ops[pos] // 1000) % 10
                secondp = ops[pos+2] if secondp_mode else ops[ops[pos+2]]
            else:
                if arithmetic_op == 3:
                    x = int(input())
                    ops[ops[pos+1]] = x
                    pos += 2

                elif arithmetic_op == 4:
                    print(firstp)
                    pos += 2

                continue

            thirdp = firstp + secondp if arithmetic_op == 1 else firstp * secondp
            ops[ops[pos+3]] = thirdp
            pos += 4



