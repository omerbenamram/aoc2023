package main

import (
	"strings"
	"testing"
)

func TestGame(t *testing.T) {
	input := `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
	Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
	Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
	Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
	Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green`

	games := make([]*Game, 0)
	for _, line := range strings.Split(input, "\n") {
		game, err := parseGame(strings.TrimSpace(line))
		if err != nil {
			t.Fatal(err)
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

	if part1 != 8 {
		t.Errorf("Part1 = %d, want %d", part1, 8)
	}
	if part2 != 2286 {
		t.Errorf("Part2 = %d, want %d", part2, 2286)
	}
}
