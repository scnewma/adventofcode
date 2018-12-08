package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"strings"
	"unicode"
)

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	var polymer string
	if _, err := fmt.Fscanf(f, "%s", &polymer); err != nil {
		log.Fatal(err)
	}

	// Part 1
	fmt.Println(len(reduce(polymer)))

	// Part 2
	fmt.Println(len(shortest(polymer)))
}

func shortest(polymer string) string {
	alpha := "abcdefghijklmnopqrstuvwxyz"
	resultCh := make(chan string, len(alpha))
	for _, r := range alpha {
		go func(r rune) {
			p := removeAll(polymer, string(r))
			p = removeAll(p, string(unicode.ToUpper(r)))
			resultCh <- reduce(p)
		}(r)
	}
	min := ""
	minlen := math.MaxInt64
	for i := 0; i < len(alpha); i++ {
		p := <-resultCh

		plen := len(p)
		if plen < minlen {
			minlen = plen
			min = p
		}
	}

	return min
}

func removeAll(polymer string, r string) string {
	return strings.Replace(polymer, r, "", -1)
}

func reduce(polymer string) string {
	for i := 1; i < len(polymer); i++ {
		if polar(rune(polymer[i-1]), rune(polymer[i])) {
			return reduce(polymer[:i-1] + polymer[i+1:])
		}
	}
	return polymer
}

func polar(a, b rune) bool {
	return unicode.ToUpper(a) == unicode.ToUpper(b) && ((unicode.IsUpper(a) && unicode.IsLower(b)) || (unicode.IsLower(a) && unicode.IsUpper(b)))
}
