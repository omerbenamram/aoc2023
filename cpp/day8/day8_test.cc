#include "libday8.h"
#include "gtest/gtest.h"

TEST(Day8Test, NoCycle)
{
    std::string input_str = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)\n";
    Input inputData = input(input_str);
    EXPECT_EQ(part1(inputData), 2);
}

TEST(Day8Test, Cycle)
{
    std::string input_str = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)\n";
    Input inputData = input(input_str);
    EXPECT_EQ(part1(inputData), 6);
}

TEST(Day8Test, Part2)
{
    std::string input_str = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)\n";
    Input inputData = input(input_str);
    EXPECT_EQ(part2(inputData), 6);
}
