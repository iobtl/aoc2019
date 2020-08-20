infinity = float('inf')


class Wire:
    def __init__(self):
        self.x = 0
        self.y = 0
        self.pos = []

    def record_position(self):
        self.pos.append((self.x, self.y))

    def move(self, instruction):
        direction, steps = instruction[0], int(instruction[1:])
        if direction == 'U':
            for step in range(steps):
                self.y += 1
                self.record_position()

        elif direction == 'R':
            for step in range(steps):
                self.x += 1
                self.record_position()

        elif direction == 'D':
            for step in range(steps):
                self.y -= 1
                self.record_position()

        elif direction == 'L':
            for step in range(steps):
                self.x -= 1
                self.record_position()

    def intersect(self, wire):
        return set(self.pos).intersection(wire.pos)


def parse_instructions(instructions):
    instructions = [instructions[i].rstrip(
        '\n') for i in range(len(instructions))]
    return instructions[0].split(','), instructions[1].split(',')


if __name__ == '__main__':
    with open("input.txt", "r") as f:
        instructions = f.readlines()
        first_wire_instructions, second_wire_instructions = parse_instructions(
            instructions)

    first_wire = Wire()
    second_wire = Wire()

    for instruction in first_wire_instructions:
        first_wire.move(instruction)

    for instruction in second_wire_instructions:
        second_wire.move(instruction)

    intersection_points = first_wire.intersect(second_wire)

    smallest_distance = infinity
    smallest_steps = infinity
    for intersection_point in intersection_points:
        distance = abs(intersection_point[0]) + abs(intersection_point[1])
        combined_steps = first_wire.pos.index(
            intersection_point) + second_wire.pos.index(intersection_point)

        if distance < smallest_distance:
            smallest_distance = distance

        if combined_steps < smallest_steps:
            smallest_steps = combined_steps

    print(smallest_distance)
    print(smallest_steps)
