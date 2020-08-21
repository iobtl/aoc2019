orbit_mapping = dict()
sum = 0
start_callsign = 'YOU'
target_callsign = 'SAN'


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

    you_object = orbit_mapping[start_callsign][0]
    target_object = orbit_mapping[target_callsign][0]

    transfers = 0
    you_orbits = []
    target_orbits = []
    while you_object:
        if isinstance(you_object, list):
            you_object = you_object[0]
        you_orbits.append(you_object)
        you_object = orbit_mapping.get(you_object)

    while target_object:
        if isinstance(target_object, list):
            target_object = target_object[0]
        target_orbits.append(target_object)
        target_object = orbit_mapping.get(target_object)

    common_orbit = None
    for i in you_orbits:
        if i in target_orbits:
            common_orbit = i
            break

    transfers = you_orbits.index(common_orbit) + \
        target_orbits.index(common_orbit)
    print(transfers)
