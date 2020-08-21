orbit_mapping = dict()
sum = 0


def search_indirect_orbits(orbiter):
    if isinstance(orbiter, list):
        orbiter = orbiter[0]
    orbit_object = orbit_mapping.get(orbiter)
    if orbit_object:
        return 1 + search_indirect_orbits(orbit_object)
    else:
        return 0


if __name__ == '__main__':
    with open("input.txt", "r") as f:
        lines = f.readlines()
        orbit_pairs = [i.rstrip('\n').split(')') for i in lines]

    for orbit_object, orbiter in orbit_pairs:
        orbit_mapping.setdefault(orbiter, []).append(orbit_object)

    for (orbiter, orbit_object) in orbit_mapping.items():
        total_objects = len(orbit_object)
        sum += total_objects

        for indirect_orbit in orbit_object:
            res = search_indirect_orbits(indirect_orbit)
            sum += res

    print(sum)
