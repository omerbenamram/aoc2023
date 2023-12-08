#include <iostream>
#include <vector>
#include <regex>
#include <string>
#include "libday8.h"

int main()
{
    std::string stdin_input;
    std::string line;
    while (std::getline(std::cin, line))
    {
        stdin_input += line + "\n";
    }
    Input inputData = input(stdin_input);
    std::cout << "Part1: " << part1(inputData) << std::endl;
    std::cout << "Part2: " << part2(inputData) << std::endl;
    return 0;
}
