package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Color int

const (
	Red Color = iota
	Green
	Blue
)

type Game struct {
	id       int
	showings [][]Pair
}

type Pair struct {
	count int
	color Color
}

func (g *Game) isPossible() bool {
	for _, showing := range g.showings {
		for _, pair := range showing {
			switch pair.color {
			case Red:
				if pair.count > 12 {
					return false
				}
			case Green:
				if pair.count > 13 {
					return false
				}
			case Blue:
				if pair.count > 14 {
					return false
				}
			}
		}
	}
	return true
}

func (g *Game) power() int {
	maxRed, maxGreen, maxBlue := 0, 0, 0
	for _, showing := range g.showings {
		for _, pair := range showing {
			switch pair.color {
			case Red:
				if pair.count > maxRed {
					maxRed = pair.count
				}
			case Green:
				if pair.count > maxGreen {
					maxGreen = pair.count
				}
			case Blue:
				if pair.count > maxBlue {
					maxBlue = pair.count
				}
			}
		}
	}
	return maxRed * maxGreen * maxBlue
}

var reGame = regexp.MustCompile(`Game (\d+): (.+)`)
var reShowings = regexp.MustCompile(`(\d+) (\w+)`)

func parseGame(s string) (*Game, error) {
	matches := reGame.FindStringSubmatch(s)
	if matches == nil {
		return nil, fmt.Errorf("invalid game string")
	}

	id, err := strconv.Atoi(matches[1])
	if err != nil {
		return nil, err
	}

	showingsStr := matches[2]
	showings := make([][]Pair, 0)
	for _, showingStr := range strings.Split(showingsStr, "; ") {
		showing := make([]Pair, 0)
		for _, match := range reShowings.FindAllStringSubmatch(showingStr, -1) {
			count, err := strconv.Atoi(match[1])
			if err != nil {
				return nil, err
			}

			var color Color
			switch match[2] {
			case "red":
				color = Red
			case "green":
				color = Green
			case "blue":
				color = Blue
			default:
				return nil, fmt.Errorf("invalid color: %s", match[2])
			}

			showing = append(showing, Pair{count, color})
		}
		showings = append(showings, showing)
	}

	return &Game{id, showings}, nil
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	games := make([]*Game, 0)
	for scanner.Scan() {
		game, err := parseGame(scanner.Text())
		if err != nil {
			fmt.Println(err)
			return
		}
		games = append(games, game)
	}

	part1, part2 := 0, 0
	for _, game := range games {
		if game.isPossible() {
			part1 += game.id
		}
		part2 += game.power()
	}

	fmt.Printf("Part1: %d\n", part1)
	fmt.Printf("Part2: %d\n", part2)
}
