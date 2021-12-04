package main

import (
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

func main() {
	rawData, err := ioutil.ReadFile("../input.txt")
	if err != nil {
		fmt.Println(err)
	}
	slicedData := strings.Split(string(rawData), "\n")
	numInputs := len(slicedData)
	allBits := make([]int, len(slicedData[0]))
	for _, data := range slicedData {
		listOfChars := strings.Split(data, "")
		for i, char := range listOfChars {
			bit, _ := strconv.Atoi(char)
			allBits[i] += bit
		}
	}

	var gamma uint = 0
	var mask uint = 0
	for _, count := range allBits {
		gamma = gamma << 1
		mask = mask << 1
		if count > (numInputs / 2) {
			gamma++
		}
		mask++
	}

	var epsilon uint = gamma ^ mask

	fmt.Println(gamma * epsilon)
}
