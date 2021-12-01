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
	slicedData := strings.Fields(string(data))
	parsedData := make([]int, len(slicedData))
	for i, v := range slicedData {
		parsedData[i], _ = strconv.Atoi(v)
	}

	var increasingCount int = 0

	var lastSum = parsedData[0] + parsedData[1] + parsedData[2]
	for i := 1; i < len(parsedData)-2; i++ {
		sum := parsedData[i] + parsedData[i+1] + parsedData[i+2]
		if sum > lastSum {
			increasingCount++
		}
		lastSum = sum
	}

	fmt.Println(increasingCount)
}
