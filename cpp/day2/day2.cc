#include <iostream>
#include <vector>
#include <regex>
#include <string>
#include "libday2.h"

int main()
{
    std::vector<Game> input;
    std::string line;
    while (std::getline(std::cin, line))
    {
        Game game = from_string(line);
        input.push_back(game);
    }
    std::cout << "Part1: " << part1(input) << std::endl;
    std::cout << "Part2: " << part2(input) << std::endl;
    return 0;
}
