package client

import (
	"testing"
)

var b = Bot{
	BotId:    1,
	Name:     "bot",
	TeamId:   1,
	Hp:       1,
	Alive:    true,
	Position: Position{0, 0},
}

func TestGetValidMoves(t *testing.T) {
	a := b.GetValidMoves(2)
	if len(a) != 19 {
		t.Fatalf("Invalid amount of valid moves %d, should be 19.\n", len(a))
	}
}

func TestGetValidCannons(t *testing.T) {
	a := b.GetValidCannons(5)
	if len(a) != 91 {
		t.Fatalf("Invalid amount of valid cannon positons %d, should be 91.\n", len(a))
	}
}

func TestGetValidRadars(t *testing.T) {
	a := b.GetValidRadars(10)
	if len(a) != 331 {
		t.Fatalf("Invalid amount of valid radar positons %d, should be 331.\n", len(a))
	}
}
