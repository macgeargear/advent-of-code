package day01

import (
	"strings"
	"testing"
)

var testInput1 = `TEST INPUT GOES HERE`

func TestSolvePart1(t *testing.T) {
	tests := []struct {
		input  string
		answer int
	}{
		{testInput, 0},
	}

	for _, test := range tests {
		r := strings.NewReader(test.input)

		result := SolvePart1(r)

		if result != test.answer {
			t.Errorf("Expected %d, got %d", test.answer, result)
		}
	}
}
