#include <iostream>
#include <string>
#include <chrono>

using namespace std;

string run(char* s) {
    // Your code goes here
}

int main(int argc, char** argv) {
    if (argc < 2) {
        cout << "Missing one argument" << endl;
        exit(1);
    }

    auto start = std::chrono::high_resolution_clock::now();
    auto answer = run(argv[1]);
    auto end = std::chrono::high_resolution_clock::now();

    cout << "_duration:"<< float(std::chrono::duration_cast<std::chrono::microseconds>(end-start).count()) / 1000.0 << "\n";
    
    cout << answer << "\n";
    return 0;
}
