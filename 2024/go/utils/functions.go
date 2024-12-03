package utils

import (
	"bufio"
	"fmt"
	"io"
)

func Gcd(a, b int) int {
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

func Lcm(a, b int) int {
	return a * b / (Gcd(a, b))
}

func Check(e error, format string, a ...any) {
	if e != nil {
		message := fmt.Sprintf(format, a...)
		panic(fmt.Errorf("%s: %s", message, e))
	}
}

func ReadLines(r io.Reader) []string {
	result := []string{}
	sc := bufio.NewScanner(r)
	for sc.Scan() {
		line := sc.Text()
		result = append(result, line)
	}

	return result
}
