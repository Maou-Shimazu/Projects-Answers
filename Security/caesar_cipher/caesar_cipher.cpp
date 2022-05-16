#include <iostream>

int main(void) {
        std::string type;
        std::cout << "Would you like to encrypt or decrypt a message?" << std::endl;
        std::cin >> type;
        bool encrypt = type == "encrypt";
        std::string message;
        std::cout << "Enter the message you want " << (encrypt? "encrypted" : "decrypted") << "..." << std::endl;
        std::cin >> message;
        int shift;
        std::cout << "Enter the shift number..." << std::endl;
        std::cin >> shift;
        std::string a_lower = "abcdefghijklmnopqrstuvwxyz", a_upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        std::string output;
        shift %= 26;
        if (!encrypt)
                shift *= -1;
        for (char c: message) {
                int i = a_lower.find(c, 0), j = a_upper.find(c, 0);
                if (i != -1)
                        output += a_lower[(i + shift + 26) % 26];
                else if (j != -1)
                        output += a_upper[(i + shift + 26) % 26];
                else
                        output += c;
        }
  
        std::cout << output << std::endl;
        return 0;
}
