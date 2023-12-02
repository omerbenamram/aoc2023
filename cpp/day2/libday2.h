#ifndef LIBDAY2_H
#define LIBDAY2_H

#include <vector>
#include <string>
#include <utility>

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

    bool is_possible() const;
    int power() const;
};

Game from_string(const std::string &s);

int part1(const std::vector<Game> &games);
int part2(const std::vector<Game> &games);

#endif // LIBDAY2_H
