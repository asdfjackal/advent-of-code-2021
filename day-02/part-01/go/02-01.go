package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	data, err := ioutil.ReadFile("../input.txt")
	if err != nil {
		fmt.Println(err)
	}
	slicedData := strings.Split(string(data), "\n")
	directions := make([]string, len(slicedData))
	distances := make([]int, len(slicedData))
	for i, v := range slicedData {
		line := strings.Fields(v)
		directions[i] = line[0]
		distances[i], _ = strconv.Atoi(line[1])
	}

	vertical := 0
	horizontal := 0

	for i := 0; i < len(slicedData); i++ {
		if directions[i] == "forward" {
			horizontal += distances[i]
		} else if directions[i] == "up" {
			vertical -= distances[i]
		} else if directions[i] == "down" {
			vertical += distances[i]
		}
	}

	fmt.Println(vertical * horizontal)
}
