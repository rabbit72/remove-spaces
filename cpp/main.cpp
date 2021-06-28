#include <iostream>

using namespace std;

void RemoveExtraSpaces(string &target) {
    const char space = ' ';
    int from_index = 0; // copy from this index
    int to_index = 0;  // copy to this index
    bool is_previous_space = false;  // set true when a single space is written

    int original_length = target.length();

    // step 1: skipping spaces at the begining
    for (; from_index < original_length && target[from_index] == space; from_index++) {}
 
    // step 2: processing all chars after possible spaces
    for (; from_index < original_length; from_index++) {
        if (target[from_index] != space) {  // copies 'from_index' to 'to_index'
            target[to_index] = target[from_index];
            to_index++;
            is_previous_space = false;
        } else {
            if (!is_previous_space) {  //puts one space if the previous is not a space
                target[to_index] = space;
                to_index++;
                is_previous_space = true;  //stops appearing more than 1 space
            }
        }
    }
    // step 3: cutting the string inplace
    if (to_index > 1 && is_previous_space) {
        target.erase(target.begin() + to_index - 1, target.end());
    } else {
        target.erase(target.begin() + to_index, target.end());
    }
}


int main() {
    string targets[] = {
        " On  my   home world",
        "On my home world",
        "  On  my        home world    ",
        "On  my        home world  ",
        "    ",
        " ",
        "",
        "  4",
        "4  ",
        "42"
    };

    for (int i = 0; i < 10; i++) {
        printf("Case %i:\n", i + 1);
        printf("Input: |%s|\n", targets[i].c_str());
        RemoveExtraSpaces(targets[i]);
        printf("Output: |%s|\n\n", targets[i].c_str());
    }
    return 0;
}
