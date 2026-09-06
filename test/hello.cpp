#include <iostream>
#include <vector>

void testfunc() {
    std::cout << "this is a test void function" << std::endl;
}

int testfunc2() {
    int baz = 3;
    std::cout << "look at this number: " << baz << std::endl;
}

// our main function
int main() {
    //std::vector<std::string> colours = {"red", "green", "blue"}; 
    // declare some variables
    int foo = 1;
    int bar = 2;
    testfunc();
    // print some text
    std::cout << "Hello World" << std::endl;
    if (bar == 1) {
        std::cout << "bar equals 1" << std::endl;
    }
    // return
    return 0;
}
