package main

import (
	"fmt"
	"testing"
)

func TestReduce(t *testing.T) {
	input := "dabAcCaCBAcCcaDA"

	output := reduce(input)

	if output != "dabCBAcaDA" {
		t.Errorf("expected: dabCBAcaDA, got: %s\n", output)
	}
}

func TestPolar(t *testing.T) {
	tests := []struct {
		a, b rune
		want bool
	}{
		{'a', 'A', true},
		{'A', 'a', true},
		{'a', 'a', false},
		{'A', 'A', false},
		{'a', 'B', false},
	}

	for _, tt := range tests {
		t.Run(fmt.Sprintf("polar(%s, %s)", string(tt.a), string(tt.b)), func(t *testing.T) {
			got := polar(tt.a, tt.b)
			if got != tt.want {
				t.Errorf("expected polar(%s, %s) to be %t, got %t", string(tt.a), string(tt.b), tt.want, got)
			}
		})
	}
}
