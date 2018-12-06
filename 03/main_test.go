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
