package main

import "testing"

func TestPart1(t *testing.T) {
	input := []string{
		"1abc2",
		"pqr3stu8vwx",
		"a1b2c3d4e5f",
		"treb7uchet",
	}

	result, err := part1(input)
	if err != nil {
		t.Fatal(err)
	}

	if result != 142 {
		t.Errorf("Expected 142, got %d", result)
	}
}

func TestPart2(t *testing.T) {
	input := []string{
		"two1nine",
		"eightwothree",
		"abcone2threexyz",
		"xtwone3four",
		"4nineeightseven2",
		"zoneight234",
		"7pqrstsixteen",
		"eighthree",
	}

	result, err := part2(input)
	if err != nil {
		t.Fatal(err)
	}

	if result != (364) {
		t.Errorf("Expected 364, got %d", result)
	}
}
