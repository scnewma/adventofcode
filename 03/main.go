package main

import (
	"fmt"
	"log"
	"os"
)

type Claim struct {
	ID        int
	LeftStart int
	TopStart  int
	Width     int
	Height    int
}

func (c Claim) String() string {
	return fmt.Sprintf("#%d @ %d,%d: %dx%d", c.ID, c.LeftStart, c.TopStart, c.Width, c.Height)
}

func newFabric(width, height int) Fabric {
	f := make([][]int, height)
	for i := range f {
		f[i] = make([]int, width)
	}
	return f
}

type Fabric [][]int

func (f Fabric) AddClaim(c Claim) {
	for i := c.TopStart; i < c.TopStart+c.Height; i++ {
		for j := c.LeftStart; j < c.LeftStart+c.Width; j++ {
			f[i][j] = f[i][j] + 1
		}
	}
}

func (f Fabric) Overlap() int {
	overlap := 0
	for i := 0; i < len(f); i++ {
		for j := 0; j < len(f[i]); j++ {
			if f[i][j] > 1 {
				overlap++
			}
		}
	}
	return overlap
}

func (f Fabric) Print() {
	for _, row := range f {
		fmt.Println(row)
	}
}

func main() {
	f, err := os.Open("claims.txt")
	if err != nil {
		log.Fatal("opening claims.txt:", err)
	}

	fabric := newFabric(1000, 1000)
	var id, left, top, width, height int
	for {
		if _, err := fmt.Fscanf(f, "#%d @ %d,%d: %dx%d", &id, &left, &top, &width, &height); err != nil {
			break
		}

		fabric.AddClaim(Claim{
			ID:        id,
			LeftStart: left,
			TopStart:  top,
			Width:     width,
			Height:    height,
		})
	}

	fmt.Printf("Overlap: %d\n", fabric.Overlap())
}
