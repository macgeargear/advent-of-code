package main

import (
	"fmt"
	"io"
	"os"
	"strconv"
	"time"

	day01 "github.com/macgeargear/advent-of-code-go/day01"
	"github.com/macgeargear/advent-of-code-go/utils"
	"github.com/spf13/pflag"
)

var RunAll bool
var DaySelected int
var PartSelected int

type aocFunc func(io.Reader) any

type aocResult struct {
	Result      string
	TimeElapsed time.Duration
}

type aocRunnerInput struct {
	Name     string
	Func     aocFunc
	Filename string
	Day      int
	Part     int
}

var days = []aocRunnerInput{
	{
		Name:     "Day 1 part 1",
		Func:     day01.SolvePart1,
		Filename: "day01/input1.txt",
		Day:      1,
		Part:     1,
	},
	{
		Name:     "Day 1 part 2",
		Func:     day01.SolvePart2,
		Filename: "day01/input2.txt",
		Day:      1,
		Part:     2,
	},
}

func init() {
	pflag.BoolVarP(&RunAll, "all", "a", false, "run all days")
	pflag.IntVarP(&DaySelected, "day", "d", 0, "run specific day")
	pflag.IntVarP(&PartSelected, "part", "p", 1, "run specific part")
}

func runPart(partFunc aocFunc, filename string) aocResult {
	f, err := os.Open(filename)
	utils.Check(err, "unable to open file %s", filename)
	defer f.Close()

	start := time.Now()
	r := partFunc(f)
	duration := time.Since(start)

	res := aocResult{TimeElapsed: duration}

	switch v := r.(type) {
	case int:
		res.Result = strconv.Itoa(v)
	case int64:
		res.Result = strconv.FormatInt(v, 10)
	case uint64:
		res.Result = strconv.FormatUint(v, 10)
	case string:
		res.Result = v
	case fmt.Stringer:
		res.Result = v.String()
	default:
		res.Result = "unknown return value"
	}

	return res
}

func runDay(day int, part int) {
	found := false

	for _, v := range days {
		if v.Day == day && v.Part == part {
			fmt.Printf("Day %d part %d\n", day, part)
			r := runPart(v.Func, v.Filename)
			fmt.Println(r.Result)
			fmt.Printf("Time elapsed: %s\n", r.TimeElapsed)
			found = true
			break
		}
	}

	if !found {
		fmt.Printf("Did not find a solution for day %d part %d\n", day, part)
	}
}

func runCurrent() {
	fmt.Println("Day 1 Part 1")
	r := runPart(day01.SolvePart1, "inputs/day01.txt")
	fmt.Println(r.Result)
	fmt.Printf("Time elapsed: %s\n", r.TimeElapsed)
}

func main() {
	pflag.Parse()

	if RunAll {
		// runAll()
		return
	}

	runDay(DaySelected, PartSelected)
}
