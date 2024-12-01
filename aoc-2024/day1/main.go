package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	file, err := os.Open("day1/input.txt")
	if err != nil {
		fmt.Println("Error opening file: ", err.Error())
		return
	}

	defer file.Close()

	scanner := bufio.NewScanner(file)
	arr1, arr2 := []int{}, []int{}
	var ans1, ans2 uint32 = 0, 0

	for scanner.Scan() {
		parts := strings.Fields(scanner.Text())
		if len(parts) == 2 {
			num1, _ := strconv.Atoi(parts[0])
			num2, _ := strconv.Atoi(parts[1])
			arr1 = append(arr1, num1)
			arr2 = append(arr2, num2)
		}
	}

	err = scanner.Err()
	if err != nil {
		fmt.Println("Error reading file: ", err.Error())
	}

	slices.Sort(arr1)
	slices.Sort(arr2)

	for i := range arr1 {
		ans1 += uint32(math.Abs(float64(arr1[i] - arr2[i])))
	}

	for i := range arr1 {
		for j := range arr2 {
			if arr1[i] == arr2[j] {
				ans2 += uint32(arr1[i])
			}
		}
	}

	fmt.Printf("Ans for part 1: %v\n", ans1)
	fmt.Printf("Ans for part 2: %v\n", ans2)
}
