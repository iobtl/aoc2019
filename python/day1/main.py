import math

def calculate_fuel(mass):
    fuel = math.floor(mass / 3) - 2
    if fuel <= 0:
        return 0

    return fuel + calculate_fuel(fuel)

if __name__ == '__main__':
    with open("input.txt", "r") as f:
        masses = [i.split('\n')[0] for i in f.readlines()]
        f.close()

    mass_fuel = [calculate_fuel(int(mass)) for mass in masses]
    print(sum(mass_fuel))


