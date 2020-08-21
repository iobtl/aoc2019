INPUT = "193651-649729"


def get_range():
    numbers = [int(num) for num in INPUT.split('-')]
    return numbers[0], numbers[1]


if __name__ == '__main__':
    password_count = 0

    lower, upper = get_range()
    for num in range(lower, upper):
        adjacent_flag = 0
        increasing_flag = 1
        repeated = 0
        prev = ''
        for char in str(num):
            if char == prev:
                if repeated < 1:
                    adjacent_flag = 1
                else:
                    adjacent_flag = 0

                repeated += 1

            else:
                repeated = 0

            if char < prev:
                increasing_flag = 0

            prev = char

        if adjacent_flag and increasing_flag:
            print(num)
            password_count += 1


    print(password_count)
