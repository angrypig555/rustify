#include <iostream>
#include <vector>

void testfunc() {
    std::cout << "this is a test void function" << std::endl;
}

void testfunc2() {
    int baz = 3;
    std::cout << "look at this number: " << baz << std::endl;
}

// our main function
int main() {
// std::vector<std::string> colours = {"red", "green", "blue"}; 
// declare some variables
    int foo = 1;
    int bar = 2;
    bool testbool = true;
    bool testbool2 = false;
    float testfloat = 1.23;
    double testdouble = 1.23;
    testfunc();
// print some text
    std::cout << "Hello World" << std::endl;
// if (bar == 1) { (not yet implemented)
//    std::cout << "bar equals 1" << std::endl;
// }
// return
    return 0;
}
