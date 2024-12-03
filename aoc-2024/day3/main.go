package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	file, err := os.Open("day3/input.txt")
	if err != nil {
		fmt.Println("Error opening file: ", err.Error())
		return
	}

	defer file.Close()

	var ans1, ans2 uint32 = 0, 0
	scanner := bufio.NewScanner(file)
	re := regexp.MustCompile(`(do\(\)|don't\(\)|mul\((\d{1,3}),(\d{1,3})\))`)
	var input string
	to_match := true
	for scanner.Scan() {
		input += scanner.Text()
	}

	err = scanner.Err()
	if err != nil {
		fmt.Println("Error reading file: ", err.Error())
	}

	matches := re.FindAllStringSubmatch(input, -1)

	for _, match := range matches {
		if match[1] == "do()" {
			to_match = true
		} else if match[1] == "don't()" {
			to_match = false
		} else if len(match) == 4 {
			x, x_err := strconv.Atoi(match[2])
			y, y_err := strconv.Atoi(match[3])

			if x_err != nil || y_err != nil {
				fmt.Println("Error Parsing Arguments")
				return
			}

			var prod uint32 = uint32(x) * uint32(y)

			ans1 += prod
			if to_match {
				ans2 += prod
			}
		}
	}

	fmt.Printf("Ans for part 1: %v\n", ans1)
	fmt.Printf("Ans for part 2: %v\n", ans2)
}
