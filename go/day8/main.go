package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"strings"
)

type Instructions string
type Graph map[string][]string
type Input struct {
	instructions Instructions
	graph        Graph
}

func input(s string) (Input, error) {
	lines := strings.Split(s, "\n")
	instructions := Instructions(lines[0])
	lines = lines[2:] // skip newline

	re := regexp.MustCompile(`(?P<node>\w{3}) = \((?P<left>\w{3}),\s(?P<right>\w{3})\)`)
	graph := make(Graph)

	for _, line := range lines {
		match := re.FindStringSubmatch(line)
		node := match[1]
		left := match[2]
		right := match[3]

		graph[node] = append(graph[node], left, right)
	}

	return Input{instructions, graph}, nil
}

func pathLenFrom(graph Graph, instructions Instructions, node string) (int, error) {
	steps := 0
	at := node

	instIndex := 0
	instLen := len(instructions)
	for {
		inst := instructions[instIndex%instLen]
		switch inst {
		case 'L':
			at = graph[at][0]
		case 'R':
			at = graph[at][1]
		default:
			return 0, fmt.Errorf("unexpected instruction")
		}
		steps++
		if strings.HasSuffix(at, "Z") {
			break
		}
		instIndex++
	}

	return steps, nil
}

func primeFactorization(i int) []int {
	var factors []int

	if i%2 == 0 {
		factors = append(factors, 2)
	}

	if i%3 == 0 {
		factors = append(factors, 3)
	}

	for p := 3; p < i; {
		if i%p == 0 {
			factors = append(factors, p)
		}

		p += 2
	}

	return factors
}

func part1(input Input) (int, error) {
	return pathLenFrom(input.graph, input.instructions, "AAA")
}

func part2(input Input) (int64, error) {
	var at []string
	for node := range input.graph {
		if strings.HasSuffix(node, "A") {
			at = append(at, node)
		}
	}

	primeFactors := make(map[int]struct{})

	for _, node := range at {
		pathLen, _ := pathLenFrom(input.graph, input.instructions, node)

		for _, n := range primeFactorization(pathLen) {
			primeFactors[n] = struct{}{}
		}
	}

	product := int64(1)
	for factor := range primeFactors {
		fmt.Println(factor)
		product *= int64(factor)
	}

	return product, nil
}

func main() {
	bytes, _ := io.ReadAll(os.Stdin)
	stdinInput := string(bytes)
	input, _ := input(stdinInput)
	result1, _ := part1(input)
	fmt.Printf("Part1: %d\n", result1)
	result2, _ := part2(input)
	fmt.Printf("Part2: %d\n", result2)
}
