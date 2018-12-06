package main

import (
	"reflect"
	"testing"
)

func TestChecksum(t *testing.T) {
	expectedCS := 12
	cs := checksum([]string{"abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"})

	if cs != expectedCS {
		t.Errorf("Expected checksum to be %d, got %d", expectedCS, cs)
	}
}

func TestCorrectBoxes(t *testing.T) {
	expected := []string{"fghij", "fguij"}
	actual := correctBoxes([]string{
		"abcde",
		"fghij",
		"klmno",
		"pqrst",
		"fguij",
		"axcye",
		"wvxyz",
	})

	if !reflect.DeepEqual(expected, actual) {
		t.Errorf("Expected correct box ids to be %+q, got %+q", expected, actual)
	}
}

func TestNeighbors(t *testing.T) {
	if !neighbors("fghij", "fguij") {
		t.Error("expected to be neighbors")
	}
}

func TestNotNeighbors(t *testing.T) {
	if neighbors("abcde", "axcye") {
		t.Error("expected not to be neighbors")
	}
}

func TestCommonLetters(t *testing.T) {
	if common := commonLetters("abcda", "abcea"); common != "abca" {
		t.Error("expected common letters to be abca, got:", common)
	}
}
