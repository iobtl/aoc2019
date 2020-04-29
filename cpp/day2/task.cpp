#include <iostream>
#include <fstream>
#include <vector>
#include <cassert>
using namespace std;

int main() {
    vector<int> vec = {1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,6,19,1,9,19,23,1,6,23,27,1,10,27,31,1,5,31,35,2,6,35,39,1,5,39,43,1,5,43,47,2,47,6,51,1,51,5,55,1,13,55,59,2,9,59,63,1,5,63,67,2,67,9,71,1,5,71,75,2,10,75,79,1,6,79,83,1,13,83,87,1,10,87,91,1,91,5,95,2,95,10,99,2,9,99,103,1,103,6,107,1,107,10,111,2,111,10,115,1,115,6,119,2,119,9,123,1,123,6,127,2,127,10,131,1,131,6,135,2,6,135,139,1,139,5,143,1,9,143,147,1,13,147,151,1,2,151,155,1,10,155,0,99,2,14,0,0};
    vector<int> mut;
    int opcode, input_one, input_two, output;

    // Iterating through the possible range of values for the verb and noun
    for (int n = 0; n <= 99; ++n) {
        if (mut.size() == 0) {
            for (int j = 0; j < vec.size(); ++j) {
                mut.push_back(vec[j]);
            }
        }
        for (int v = 0; v <= 99; ++v) {
            if (mut.size() == 0) {
                for (int j = 0; j < vec.size(); ++j) {
                    mut.push_back(vec[j]);
                }
            }
            mut[1] = n;
            mut[2] = v;
            for (int i = 0; i < mut.size(); i += 4) {
                opcode = mut[i];
                input_one = mut[i+1];
                input_two = mut[i+2];
                output = mut[i+3];

                if (opcode == 99) {
                    break;
                } 
                else if (opcode == 1) {
                    int add = mut[input_one] + mut[input_two];
                    mut[output] = add;
                }
                else {
                    int multiply = mut[input_one] * mut[input_two];
                    mut[output] = multiply;
                }
            }
            // Desired output is met
            if (mut[0] == 19690720) {
                break;
            }
            else {
                mut.clear();
            }
        }
        if (mut[0] == 19690720) {
            break;
        }
    } 
    assert (mut[0] == 19690720);
    cout << "The value of the noun is: " << mut[1] << endl;
    cout << "The value of the verb is: " << mut[2] << endl;
    cout << "Resulting value: " << 100 * mut[1] + mut[2] << endl;

    return 0;
}