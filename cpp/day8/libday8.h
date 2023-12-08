#ifndef LIBDAY8_H
#define LIBDAY8_H

#include <map>
#include <vector>
#include <string>

using Instructions = std::string;
using Graph = std::map<std::string, std::vector<std::string>>;
using Input = std::pair<Instructions, Graph>;

Input input(const std::string &s);
int64_t path_len_from(const Graph &graph, const std::string &instructions, const std::string &node);
std::vector<int64_t> prime_factorization(int i);
int64_t part1(const Input &input);
int64_t part2(const Input &input);

#endif // LIBDAY8_H
