package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
	"unicode"
)

func part1(calibrationValues []string) (int, error) {
	var sum int

	for _, line := range calibrationValues {
		digits := strings.Map(func(r rune) rune {
			if unicode.IsDigit(r) {
				return r
			}
			return -1
		}, line)

		if len(digits) == 0 {
			return 0, fmt.Errorf("expected at least one digit")
		}

		code1 := string(digits[0])
		code2 := string(digits[len(digits)-1])

		num, err := strconv.Atoi(code1 + code2)
		if err != nil {
			return 0, err
		}

		sum += num
	}

	return sum, nil
}

func toDigit(s string) (int, error) {
	c := s[0]

	switch c {
	case '0', '1', '2', '3', '4', '5', '6', '7', '8', '9':
		return strconv.Atoi(s)
	default:
		switch s {
		case "one":
			return 1, nil
		case "two":
			return 2, nil
		case "three":
			return 3, nil
		case "four":
			return 4, nil
		case "five":
			return 5, nil
		case "six":
			return 6, nil
		case "seven":
			return 7, nil
		case "eight":
			return 8, nil
		case "nine":
			return 9, nil
		default:
			return 0, fmt.Errorf("string is not a number")
		}
	}
}

func part2(calibrationValues []string) (int, error) {
	var re = regexp.MustCompile(`(\d|one|two|three|four|five|six|seven|eight|nine)`)
	var sum int

	for _, line := range calibrationValues {
		var start int
		var matches []string

		for start < len(line) {
			search := line[start:]
			loc := re.FindStringIndex(search)

			if loc == nil {
				break
			}

			match := search[loc[0]:loc[1]]
			matches = append(matches, match)
			start += max(1, loc[0])
		}

		if len(matches) == 0 {
			return 0, fmt.Errorf("expected at least one digit")
		}

		code1, err := toDigit(matches[0])
		if err != nil {
			return 0, err
		}

		code2, err := toDigit(matches[len(matches)-1])
		if err != nil {
			return 0, err
		}

		num, err := strconv.Atoi(fmt.Sprintf("%d%d", code1, code2))
		if err != nil {
			return 0, err
		}

		sum += num
	}

	return sum, nil
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	var calibrationValues []string

	for scanner.Scan() {
		calibrationValues = append(calibrationValues, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}

	part1Result, err := part1(calibrationValues)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Part1: %d\n", part1Result)

	part2Result, err := part2(calibrationValues)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Printf("Part2: %d\n", part2Result)
}
