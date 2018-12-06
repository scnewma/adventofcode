package main

import "testing"

func TestFabric(t *testing.T) {
	fabric := newFabric(10, 10)
	fabric.AddClaim(Claim{ID: 1, LeftStart: 3, TopStart: 2, Width: 5, Height: 4})
	fabric.AddClaim(Claim{ID: 2, LeftStart: 0, TopStart: 0, Width: 5, Height: 4})
	fabric.AddClaim(Claim{ID: 3, LeftStart: 3, TopStart: 2, Width: 2, Height: 2})
	expected := 4
	actual := fabric.Overlap()

	if actual != expected {
		t.Errorf("expected overlap to be %d, got: %d", expected, actual)
	}
}

func TestValid(t *testing.T) {
	fabricTests := []struct {
		claim Claim
		valid bool
	}{
		{Claim{ID: 1, LeftStart: 1, TopStart: 3, Width: 4, Height: 4}, false},
		{Claim{ID: 2, LeftStart: 3, TopStart: 1, Width: 4, Height: 4}, false},
		{Claim{ID: 3, LeftStart: 5, TopStart: 5, Width: 2, Height: 2}, true},
	}
	fabric := newFabric(10, 10)

	for _, tt := range fabricTests {
		fabric.AddClaim(tt.claim)
	}

	for _, tt := range fabricTests {
		t.Run(tt.claim.String(), func(t *testing.T) {
			got := fabric.Valid(tt.claim)
			if got != tt.valid {
				t.Errorf("expected %v, got %v", got, tt.valid)
			}
		})
	}
}
