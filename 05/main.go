package main

import (
	"fmt"
	"log"
	"os"
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

	fmt.Println(len(reduce(polymer)))
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
