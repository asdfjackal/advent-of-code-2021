package main

import (
	"fmt"
	"io/ioutil"
	"math"
	"strconv"
	"strings"
)

func main() {
	rawData, err := ioutil.ReadFile("../input.txt")
	if err != nil {
		fmt.Println(err)
	}
	slicedData := strings.Split(string(rawData), "\n")
	numBits := len(slicedData[0])

	oxygenSlice := slicedData

	for i := 0; i < numBits; i++ {
		oxygenBits := 0
		for _, data := range oxygenSlice {
			listOfChars := strings.Split(data, "")
			bit, _ := strconv.Atoi(listOfChars[i])
			oxygenBits += bit
		}
		var newSlice []string
		var target string
		if oxygenBits >= int(math.Ceil(float64(len(oxygenSlice))/2.0)) {
			target = "1"
		} else {
			target = "0"
		}
		for _, data := range oxygenSlice {
			listOfChars := strings.Split(data, "")
			if listOfChars[i] == target {
				newSlice = append(newSlice, data)
			}
		}
		oxygenSlice = newSlice
		if len(oxygenSlice) == 1 {
			break
		}
	}

	oxygen, _ := strconv.ParseInt(oxygenSlice[0], 2, 32)

	co2Slice := slicedData

	for i := 0; i < numBits; i++ {
		co2Bits := 0
		for _, data := range co2Slice {
			listOfChars := strings.Split(data, "")
			bit, _ := strconv.Atoi(listOfChars[i])
			co2Bits += bit
		}
		var newSlice []string
		var target string
		if co2Bits >= int(math.Ceil(float64(len(co2Slice))/2.0)) {
			target = "0"
		} else {
			target = "1"
		}
		for _, data := range co2Slice {
			listOfChars := strings.Split(data, "")
			if listOfChars[i] == target {
				newSlice = append(newSlice, data)
			}
		}
		co2Slice = newSlice
		if len(co2Slice) == 1 {
			break
		}
	}

	co2, _ := strconv.ParseInt(co2Slice[0], 2, 32)

	fmt.Println(oxygen * co2)
}
