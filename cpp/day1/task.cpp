#include <iostream>
#include <fstream>
#include <cmath>
#include "task.h"
using namespace std;

int calculate_fuel(int mass) {
    int fuel = floor(mass / 3) - 2;
    if (fuel > 0) {
        return fuel + calculate_fuel(fuel);
    } else {
        fuel = 0;
        return fuel;
    }
}

int main() {
    int fuel = 0;
    int n;
    ifstream infile("input.txt");

    while (infile >> n) {
        fuel += calculate_fuel(n);
    }
    cout << "Total amount of fuel needed: " << fuel << endl;

    return 0;
}