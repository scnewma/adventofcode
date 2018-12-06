package main

import (
	"fmt"
	"log"
	"os"
)

func main() {
	f, err := os.Open("boxids.txt")
	if err != nil {
		log.Fatalf("reading boxids.txt: %v", err)
	}

	var boxIDs []string
	var boxID string
	for {
		if _, err := fmt.Fscanf(f, "%s", &boxID); err != nil {
			break
		}

		boxIDs = append(boxIDs, boxID)
	}

	fmt.Printf("The checksum for the box ids is: %d\n", checksum(boxIDs))
	fabricBoxes := correctBoxes(boxIDs)
	fmt.Printf("The common letters of the correct box ids are: %s\n", commonLetters(fabricBoxes[0], fabricBoxes[1]))
}

func checksum(boxIDs []string) int {
	dups, trips := 0, 0
	for _, id := range boxIDs {
		runeCount := make(map[rune]int)
		for _, r := range id {
			runeCount[r] = runeCount[r] + 1
		}

		dups += min(len(runesFreq(runeCount, 2)), 1)
		trips += min(len(runesFreq(runeCount, 3)), 1)
	}
	return dups * trips
}

func commonLetters(boxA, boxB string) string {
	var common string
	for i := 0; i < len(boxA); i++ {
		if boxA[i] == boxB[i] {
			common += string(boxA[i])
		}
	}
	return common
}

func correctBoxes(boxIDs []string) []string {
	var correct []string
	for i := 0; i < len(boxIDs); i++ {
		for j := i + 1; j < len(boxIDs); j++ {
			boxA, boxB := boxIDs[i], boxIDs[j]

			if neighbors(boxA, boxB) {
				correct = appendUnique(correct, boxA, boxB)
			}
		}
	}

	return correct
}

func neighbors(boxA, boxB string) bool {
	diffCount := 0
	for i := 0; i < len(boxA); i++ {
		if boxA[i] != boxB[i] {
			diffCount++

			if diffCount > 1 {
				return false
			}
		}
	}

	return true
}

// returns the runes in the map that have the given frequency
func runesFreq(runeCount map[rune]int, freq int) []rune {
	var runes []rune
	for r, c := range runeCount {
		if c == freq {
			runes = append(runes, r)
		}
	}
	return runes
}

func min(i, j int) int {
	if i < j {
		return i
	}

	return j
}

func appendUnique(slice []string, elements ...string) []string {
	for _, element := range elements {
		slice = appendUniqueOne(slice, element)
	}
	return slice
}

func appendUniqueOne(slice []string, element string) []string {
	for _, sliceElement := range slice {
		if sliceElement == element {
			return slice
		}
	}
	return append(slice, element)
}
