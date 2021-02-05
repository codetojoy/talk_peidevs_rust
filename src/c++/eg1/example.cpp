
#include <iostream>
#include <vector>
#include <string>

int main() {
    std::vector<std::string> v;
    v.push_back("abc");

    std::string &x = v[0];

    v.push_back("def");

    std::cout << x;
    std::cout << "Hello World!\n";
    return 0;
}
