import math

# Defining helper functions
def calculate_fuel(mass, total_fuel=0):
    fuel = math.floor(mass / 3) - 2
    
    while fuel > 0:
        total_fuel += fuel
        fuel = calculate_fuel(fuel, total_fuel=total_fuel)

        if fuel <= 0:
            break

    return total_fuel


fuel = calculate_fuel(1969)
print(fuel)

# Reading lines off files
f = open("input.txt", "r")
lines = f.readlines()

#values = sum([calculate_fuel(int(i)) for i in lines])
#print(values)


