package main

import (
	"testing"
)

func TestNoCycle(t *testing.T) {
	inputStr := `RL

	AAA = (BBB, CCC)
	BBB = (DDD, EEE)
	CCC = (ZZZ, GGG)
	DDD = (DDD, DDD)
	EEE = (EEE, EEE)
	GGG = (GGG, GGG)
	ZZZ = (ZZZ, ZZZ)`

	input, _ := input(inputStr)
	result, _ := part1(input)
	if result != 2 {
		t.Errorf("Expected 2, but got %d", result)
	}
}

func TestCycle(t *testing.T) {
	inputStr := `LLR

	AAA = (BBB, BBB)
	BBB = (AAA, ZZZ)
	ZZZ = (ZZZ, ZZZ)`

	input, _ := input(inputStr)
	result, _ := part1(input)
	if result != 6 {
		t.Errorf("Expected 6, but got %d", result)
	}
}

func TestPart2(t *testing.T) {
	inputStr := `LR

	11A = (11B, XXX)
	11B = (XXX, 11Z)
	11Z = (11B, XXX)
	22A = (22B, XXX)
	22B = (22C, 22C)
	22C = (22Z, 22Z)
	22Z = (22B, 22B)
	XXX = (XXX, XXX)`

	input, _ := input(inputStr)
	result, _ := part2(input)
	if result != 6 {
		t.Errorf("Expected 6, but got %d", result)
	}
}
