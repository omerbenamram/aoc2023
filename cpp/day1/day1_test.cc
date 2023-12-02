#include "libday1.h"
#include "gtest/gtest.h"

TEST(Day1Test, Part2)
{
    std::vector<std::string> input = {
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
        "eighthree", // overlapping
    };
    EXPECT_EQ(part2(input), 281 + 83);
}