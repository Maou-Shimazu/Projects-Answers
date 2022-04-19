#include <iostream>
int main() {
    std::string str;
    std::cout << "Enter a word: ";
    std::cin >> str;
    for (int i = 0, len = str.length(), j = len - 1; i < j; i++, j--) {
        if (str[i] != str[j]) {
            std::cout << "Is not a palindrome";
            return 0;
        }
    }
    std::cout << "Is a palindrome";
}
