
#include <iostream>
#include <vector>
#include <string>

using namespace std;

class Cat {
  public:
    string name;
    int age;

    Cat(string s, int n) {
        name = s;
        age = n;
    }
};

ostream& operator<<(ostream &strm, const Cat &cat) {
  return strm << "name: " << cat.name << " age: " << cat.age;
}

int main() {
    vector<Cat> cats;
    cats.push_back(Cat("Mozart", 3));

    Cat &mozart = cats[0];

    cats.push_back(Cat("Bach", 5));

    cout << mozart;
    cout << "Hello World!\n";
    return 0;
}
