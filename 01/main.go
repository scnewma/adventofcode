package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
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

	freqSeen := make(map[int]bool, 100)
	currFreq := 0
	freqSeen[0] = true
	for {
		for _, f := range frequencies {
			currFreq += f

			_, ok := freqSeen[currFreq]
			if ok { // frequency has already been seen
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
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		freq, err := strconv.Atoi(scanner.Text())
		if err != nil {
			return nil, fmt.Errorf("parsing frequency: %v", err)
		}

		frequencies = append(frequencies, freq)
	}

	if err := scanner.Err(); err != nil {
		return nil, fmt.Errorf("reading frequencies: %v", err)
	}

	return frequencies, nil
}
