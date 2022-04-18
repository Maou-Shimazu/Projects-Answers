#include <iostream>
#include <cstring>
using namespace std;

int main() {
    cout << "Enter the string you want reversed..." << endl;
    string str;
    cin >> str;
    for (int i = str.length(); i > 0; i--) {
      cout << str[i - 1] << endl;
    }
    return 0;
}
