if __name__ == '__main__':
    with open("input.txt", "r") as f:
        ops = f.read().split(',')
        ops[-1] = ops[-1].split('\n')[0]
        ops = [int(i) for i in ops]

    pos = 0
    while (ops[pos] != 99):
        if ops[pos] == 1:
            ops[ops[pos+3]] = ops[ops[pos+1]] + ops[ops[pos+2]]
        elif ops[pos] == 2:
            ops[ops[pos+3]] = ops[ops[pos+1]] * ops[ops[pos+2]]

        pos += 4

    print(ops[0])
