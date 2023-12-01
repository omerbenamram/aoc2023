#include <iostream>
#include <vector>
#include <regex>
#include <string>

int to_digit(std::string s)
{
    char c = s[0];
    if (isdigit(c))
    {
        return std::stoi(s);
    }
    else
    {
        if (s == "one")
            return 1;
        if (s == "two")
            return 2;
        if (s == "three")
            return 3;
        if (s == "four")
            return 4;
        if (s == "five")
            return 5;
        if (s == "six")
            return 6;
        if (s == "seven")
            return 7;
        if (s == "eight")
            return 8;
        if (s == "nine")
            return 9;
        throw std::invalid_argument("String is not a number");
    }
}

int part1(std::vector<std::string> calibration_values)
{
    int sum = 0;
    for (std::string line : calibration_values)
    {
        std::string chars = "";
        for (char c : line)
        {
            if (isdigit(c))
                chars += c;
        }
        if (chars.empty())
            throw std::invalid_argument("Expected at least one digit");
        char code1 = chars[0];
        char code2 = chars[chars.size() - 1];
        sum += std::stoi(std::string() + code1 + code2);
    }
    return sum;
}

int part2(std::vector<std::string> calibration_values)
{
    std::regex re("(\\d|one|two|three|four|five|six|seven|eight|nine)");
    int sum = 0;
    for (std::string line : calibration_values)
    {
        std::smatch match;
        std::string::const_iterator start = line.cbegin();
        std::vector<std::string> matches;

        while (start != line.end() && std::regex_search(start, line.cend(), match, re))
        {
            matches.push_back(match[0]);
            if (match.suffix().first != line.end())
            {
                start = match.prefix().first + 1;
            }
            else
            {
                break;
            }
        }

        if (matches.empty())
        {
            throw std::invalid_argument("No match found in line: " + line);
        }
        std::string code1 = matches.front();
        std::string code2 = matches.back();
        sum += std::stoi(std::to_string(to_digit(code1)) + std::to_string(to_digit(code2)));
    }
    return sum;
}
