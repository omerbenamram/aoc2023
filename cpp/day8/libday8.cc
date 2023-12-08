#include <map>
#include <vector>
#include <string>
#include <regex>
#include <numeric>
#include <iostream>
#include <set>
#include <cstdint>

#define println(x) std::cout << x << std::endl;

using Instructions = std::string;
using Graph = std::map<std::string, std::vector<std::string>>;
using Input = std::pair<Instructions, Graph>;

Input input(const std::string &s)
{
    std::istringstream lines(s);
    std::string instructions;
    std::getline(lines, instructions);
    std::string dummy;
    std::getline(lines, dummy);

    std::regex re(R"((\w{3}) = \((\w{3}),\s(\w{3})\))");

    Graph graph;

    for (std::string line; std::getline(lines, line);)
    {
        std::smatch match;
        if (std::regex_search(line, match, re))
        {
            std::string node = match[1];
            std::string left = match[2];
            std::string right = match[3];

            graph[node].push_back(left);
            graph[node].push_back(right);
        }
    }

    return std::make_pair(instructions, graph);
}

int64_t path_len_from(const Graph &graph, const std::string &instructions, const std::string &node)
{
    int64_t steps = 0;
    std::string at = node;
    int inst_len = instructions.length();

    for (int i = 0;; i++)
    {
        char inst = instructions[i % inst_len];
        switch (inst)
        {
        case 'L':
            at = graph.at(at)[0];
            break;
        case 'R':
            at = graph.at(at)[1];
            break;
        default:
            throw std::runtime_error("Unexpected instruction");
        }
        steps++;
        if (at.back() == 'Z')
        {
            break;
        }
    }

    return steps;
}

std::vector<int64_t> prime_factorization(int64_t i)
{
    std::vector<int64_t> factors;
    for (int64_t p = 2; p < i; p++)
    {
        if (i % p == 0)
        {
            factors.push_back(p);
        }
    }

    return factors;
}

int64_t part1(const Input &input)
{
    return path_len_from(input.second, input.first, "AAA");
}

int64_t part2(const Input &input)
{
    const std::string &instructions = input.first;
    const Graph &graph = input.second;

    std::vector<std::string> at;
    for (const auto &node : graph)
    {
        if (node.first.back() == 'A')
        {
            at.push_back(node.first);
        }
    }

    std::set<int64_t> prime_factors;
    for (const auto &node : at)
    {
        int64_t path_len = path_len_from(graph, instructions, node);
        std::vector<int64_t> factors = prime_factorization(path_len);
        prime_factors.insert(factors.begin(), factors.end());
    }

    return std::accumulate(prime_factors.begin(), prime_factors.end(), int64_t(1), std::multiplies<int64_t>());
}
