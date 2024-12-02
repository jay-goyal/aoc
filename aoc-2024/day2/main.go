package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func isSafeReport(report []int32) bool {
	var prev_val int32 = -1
	var direc int32 = 0
	is_valid := true
	for _, val := range report {
		if prev_val == -1 {
			prev_val = val
		} else if direc == 0 {
			diff := val - prev_val
			if diff > 0 && diff < 4 {
				direc = 1
				prev_val = val
			} else if diff < 0 && diff > -4 {
				direc = -1
				prev_val = val
			} else {
				is_valid = false
				break
			}
		} else {
			diff := direc * (val - prev_val)
			if diff > 0 && diff < 4 {
				prev_val = val
			} else {
				is_valid = false
				break
			}
		}
	}
	return is_valid
}

func main() {
	file, err := os.Open("day2/input.txt")
	if err != nil {
		fmt.Println("Error opening file: ", err.Error())
		return
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	var reports [][]int32 = [][]int32{}
	var idx int32 = -1
	var ans1, ans2 uint32 = 0, 0

	for scanner.Scan() {
		parts := strings.Fields(scanner.Text())
		reports = append(reports, []int32{})
		idx++
		for _, s := range parts {
			num1, _ := strconv.Atoi(s)
			reports[idx] = append(reports[idx], int32(num1))
		}
	}

	err = scanner.Err()
	if err != nil {
		fmt.Println("Error reading file: ", err.Error())
	}

	for _, report := range reports {
		if isSafeReport(report) {
			ans1++
			ans2++
		} else {
			for i := range report {
				modified := append([]int32{}, report[:i]...)
				modified = append(modified, report[i+1:]...)
				if isSafeReport(modified) {
					ans2++
					break
				}
			}
		}

	}

	fmt.Printf("Ans for part 1: %v\n", ans1)
	fmt.Printf("Ans for part 2: %v\n", ans2)
}
