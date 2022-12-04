#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <set>
#include <string>
#include <vector>

char intersect(std::vector<std::string> &sets) {
  std::string init = sets[0];
  for (int i = 0; i < sets.size() - 1; i++) {
    std::string next_intersection;
    std::string compare = sets[i + 1];
    std::sort(init.begin(), init.end());
    std::sort(compare.begin(), compare.end());
    std::set_intersection(init.begin(), init.end(), compare.begin(),
                          compare.end(), std::back_inserter(next_intersection));
    init = next_intersection;
  }
  return init[0];
}

std::vector<std::string> get_duplicate_sack(std::string &str) {
  int len = str.length();
  int halfway = len / 2;
  std::string first = str.substr(0, halfway);
  std::string second = str.substr(halfway, len);
  std::vector<std::string> v;
  v.push_back(first);
  v.push_back(second);
  return v;
}

int get_prio(char &c) {
  int code = int(c);
  if (code >= 97 && code <= 122) {
    return code - 96;
  } else if (code >= 65 && code <= 90) {
    return code - 38;
  }
  std::cout << "Invalid char code " << code;
  std::exit(1);
}

int main() {
  std::ifstream file("input.txt");
  std::string str;
  int sum = 0;
  int counter = 0;
  int badges_sum = 0;
  std::vector<std::string> group;
  while (std::getline(file, str)) {
    std::vector<std::string> s = get_duplicate_sack(str);
    char c = intersect(s);
    sum += get_prio(c);

    // pt 2
    group.push_back(str);
    if (counter % 3 == 2) {
      char d = intersect(group);
      badges_sum += get_prio(d);
      group.clear();
    }

    counter++;
  }
  std::cout << sum << '\n';
  std::cout << badges_sum << '\n';
  std::cout << "I hate cpp";
  std::exit(0);
}
