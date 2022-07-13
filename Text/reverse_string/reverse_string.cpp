#include <iostream>
#include <algorithm>
#include <string>
using std::cout; using std::string; using std::cin;

int main() {
    cout << "Enter the string you want reversed: ";
    string str; cin >> str;
    std::reverse(str.begin(), str.end());
    cout << str;
    return 0;
}