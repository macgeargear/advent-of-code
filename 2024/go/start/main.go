package main

import (
	"fmt"
	"os"
	"path"
	"text/template"
	"time"

	"github.com/macgeargear/advent-of-code-go/utils"

	"github.com/spf13/pflag"
)

func main() {
	now := time.Now()
	day := pflag.IntP("day", "d", now.Day(), "Advent of Code Day")

	pflag.Parse()

	dirname := fmt.Sprintf("day%02d", *day)

	if err := os.MkdirAll(dirname, 0755); err != nil {
		fmt.Fprintf(os.Stderr, "Error creating destination directory %s: %s\n", dirname, err)
		os.Exit(1)
	}

	tpls := template.Must(template.ParseFS(os.DirFS("daily-template"), "*.tmpl"))

	part1_filename := "part1.go"
	part2_filename := "part2.go"
	part1_test_filename := "part1_test.go"
	part2_test_filename := "part2_test.go"

	// create part1
	fp1, err := os.Create(path.Join(dirname, part1_filename))
	utils.Check(err, "unable to create file %s", part1_filename)
	defer fp1.Close()

	err = tpls.ExecuteTemplate(fp1, "part1.tmpl", *day)
	utils.Check(err, "unable to execute template part1.tmpl")

	// create part2
	fp2, err := os.Create(path.Join(dirname, part2_filename))
	utils.Check(err, "unable to create file %s", part2_filename)
	defer fp1.Close()

	err = tpls.ExecuteTemplate(fp2, "part2.tmpl", *day)
	utils.Check(err, "unable to execute template part2.tmpl")

	// create part1_test
	fpt1, err := os.Create(path.Join(dirname, part1_test_filename))
	utils.Check(err, "unable to create file %s", part1_test_filename)
	defer fpt1.Close()

	err = tpls.ExecuteTemplate(fpt1, "part1_test.tmpl", *day)
	utils.Check(err, "unable to execute template part_test1.tmpl")

	// create part2_test
	fpt2, err := os.Create(path.Join(dirname, part2_test_filename))
	utils.Check(err, "unable to create file %s", part2_test_filename)
	defer fpt1.Close()

	err = tpls.ExecuteTemplate(fpt2, "part2_test.tmpl", *day)
	utils.Check(err, "unable to execute template part_test2.tmpl")
}
