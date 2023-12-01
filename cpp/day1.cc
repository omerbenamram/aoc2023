#include <iostream>
#include <vector>
#include <regex>
#include <string>
#include "libday1.h"

int main()
{
    std::vector<std::string> input;
    std::string line;
    while (std::getline(std::cin, line))
    {
        input.push_back(line);
    }
    std::cout << "Part1: " << part1(input) << std::endl;
    std::cout << "Part2: " << part2(input) << std::endl;
    return 0;
}
