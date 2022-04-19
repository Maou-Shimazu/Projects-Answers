#include <iostream>

int main(){
    for(unsigned int i = 0; i<=100; ++i){ 
        if(i % 3 == 0) std::cout << "fizz "; 
        else if(i % 5 == 0) std::cout << "buzz "; 
        else std::cout << i << " ";
    }
}
