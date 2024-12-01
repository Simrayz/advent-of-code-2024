package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	// Read file and prepare number lists
	filePath := os.Args[1]
	lines := read_file(filePath)
	left, right := build_lists_from_lines(lines)

	// Run calculations
	distance_score := get_distance_score(left, right)
	similarity_score := get_similarity_score(left, right)
	fmt.Println("Distance score  : ",distance_score)
	fmt.Println("Similarity score: ",similarity_score)
}

func read_file(path string) []string {
	readFile, err := os.Open(path);

	if err != nil {
		fmt.Println(err)
	}

	fileScanner := bufio.NewScanner(readFile)
    fileScanner.Split(bufio.ScanLines)
    var fileLines []string

	for fileScanner.Scan() {
		fileLines = append(fileLines, fileScanner.Text())
	}

	readFile.Close()

	return fileLines
}

func build_lists_from_lines(lines []string) ([]int, []int) {
	var left_numbers []int
	var right_numbers []int

	for i := 0; i < len(lines); i++ {
		var numbers = strings.Fields(lines[i])
		left_num, err_left := strconv.Atoi(numbers[0])
		right_num, err_right := strconv.Atoi(numbers[1])

		if err_left != nil || err_right != nil {
			panic("Invalid input file")
		}

		
		left_numbers = append(left_numbers, left_num)
		right_numbers = append(right_numbers, right_num)


		slices.Sort(left_numbers)
		slices.Sort(right_numbers)
		
	}
	return left_numbers, right_numbers
}

func get_distance_score(sorted_left []int, sorted_right []int) int {
	var distance_score int
	for i := 0; i < len(sorted_left); i++ {
		distance_score += abs(sorted_left[i] - sorted_right[i])
	}
	return distance_score
}

func get_similarity_score(sorted_left []int, sorted_right []int) int {
	var similarity_score int
	var frequency_map = get_frequency_map(sorted_right)

	for i := 0; i < len(sorted_left); i++ {
		if _, ok := frequency_map[sorted_left[i]]; ok {
			similarity_score += sorted_left[i] * frequency_map[sorted_left[i]]
		}
	}
	
	return similarity_score
}

func get_frequency_map(numbers []int) map[int]int {
	frequency_map := make(map[int]int)
	for i := 0; i < len(numbers); i++ {
		if _, ok := frequency_map[numbers[i]]; ok {
			frequency_map[numbers[i]]++
		} else {
			frequency_map[numbers[i]] = 1
		}
	}
	return frequency_map
}

func abs(a int) int {
    if a >= 0 {
        return a
    }
    return -a
}