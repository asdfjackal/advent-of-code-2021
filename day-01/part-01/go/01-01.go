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

	var lastDepth = parsedData[0]
	for i := 1; i < len(parsedData); i++ {
		if parsedData[i] > lastDepth {
			increasingCount++
		}
		lastDepth = parsedData[i]
	}

	fmt.Println(increasingCount)
}
