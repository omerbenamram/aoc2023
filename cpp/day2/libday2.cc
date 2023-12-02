#include <vector>
#include <string>
#include <regex>
#include <iostream>

enum class Color
{
    Red,
    Green,
    Blue,
};

struct Game
{
    int id;
    std::vector<std::vector<std::pair<int, Color>>> showings;

    bool is_possible() const
    {
        for (const auto &showing : showings)
        {
            for (const auto &pair : showing)
            {
                switch (pair.second)
                {
                case Color::Red:
                    if (pair.first > 12)
                        return false;
                    break;
                case Color::Green:
                    if (pair.first > 13)
                        return false;
                    break;
                case Color::Blue:
                    if (pair.first > 14)
                        return false;
                    break;
                }
            }
        }
        return true;
    }

    int power() const
    {
        int max_red = 0;
        int max_green = 0;
        int max_blue = 0;

        for (const auto &showing : showings)
        {
            for (const auto &pair : showing)
            {
                switch (pair.second)
                {
                case Color::Red:
                    max_red = std::max(max_red, pair.first);
                    break;
                case Color::Green:
                    max_green = std::max(max_green, pair.first);
                    break;
                case Color::Blue:
                    max_blue = std::max(max_blue, pair.first);
                    break;
                }
            }
        }

        return max_red * max_green * max_blue;
    }
};

Game from_string(const std::string &s)
{
    std::regex re("Game (\\d+): (.+)");
    std::smatch match;
    if (!std::regex_search(s, match, re))
    {
        throw std::runtime_error("Invalid game string");
    }

    Game game;
    game.id = std::stoi(match[1]);

    std::regex re_showing("(\\d+) (\\w+)");
    std::string showings_str = match[2];
    std::sregex_iterator it(showings_str.begin(), showings_str.end(), re_showing);
    std::sregex_iterator end;
    while (it != end)
    {
        int count = std::stoi((*it)[1]);
        std::string color_str = (*it)[2];
        Color color;
        if (color_str == "red")
        {
            color = Color::Red;
        }
        else if (color_str == "green")
        {
            color = Color::Green;
        }
        else if (color_str == "blue")
        {
            color = Color::Blue;
        }
        else
        {
            throw std::runtime_error("Invalid color " + color_str);
        }
        game.showings.push_back({{count, color}});
        ++it;
    }

    return game;
}

int part1(const std::vector<Game> &games)
{
    int sum = 0;
    for (const auto &game : games)
    {
        if (game.is_possible())
        {
            sum += game.id;
        }
    }
    return sum;
}

int part2(const std::vector<Game> &games)
{
    int sum = 0;
    for (const auto &game : games)
    {
        sum += game.power();
    }
    return sum;
}
