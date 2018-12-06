package main

import (
	"fmt"
	"log"
	"os"
)

func main() {
	frequencies, err := parseFrequencies("frequencies.txt")
	if err != nil {
		log.Fatalf("unable to parse frequencies: %v", err)
	}

	// Part 1 - Resulting frequency

	final := 0
	for _, f := range frequencies {
		final += f
	}

	fmt.Printf("The resulting frequency is: %d\n", final)

	// Part 2 - First duplicate frequency found

	freqSeen := map[int]bool{0: true}
	currFreq := 0
	for {
		for _, f := range frequencies {
			currFreq += f

			if _, ok := freqSeen[currFreq]; ok {
				fmt.Printf("First duplicate frequency is: %d\n", currFreq)
				return
			}

			freqSeen[currFreq] = true
		}
	}
}

func parseFrequencies(filename string) ([]int, error) {
	f, err := os.Open(filename)
	if err != nil {
		return nil, err
	}

	var frequencies []int
	for {
		var freq int
		if _, err := fmt.Fscanf(f, "%d", &freq); err != nil {
			break
		}

		frequencies = append(frequencies, freq)
	}

	return frequencies, nil
}
