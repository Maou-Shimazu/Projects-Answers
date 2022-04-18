#include <random>
#include <iostream>
#include <string>
using namespace std;

int randint(int min, int max) { // ty stackoverflow uwu https://stackoverflow.com/a/5009307 :3
    int randnum = min + (int)((double)rand() / (RAND_MAX + 1) * (max - min + 1));
    return randnum;
}

int main()
{
    string times_as_str;
    cout << "How many times would you like to flip the coin?: ";
    getline(cin, times_as_str);
    for (int i = 0; i < stoi(times_as_str); i++) {
        switch (randint(0, 1)) {
        case 0:
            cout << "heads!" << endl;
            break;
        case 1:
            cout << "tails!" << endl;
            break;
        }
    }
}
