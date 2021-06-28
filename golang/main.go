// this is a fun implementation with the attempt of rewriting bytes of a string
package main

import (
	"fmt"
	"reflect"
	"unsafe"
)

func main() {
	// The method doesn't work if we create strings right away
	targets := [][]byte{
		[]byte(" On  my   home world"),
		[]byte("On my home world"),
		[]byte("  On  my        home world    "),
		[]byte("On  my        home world  "),
		[]byte("    "),
		[]byte(" "),
		// []byte(""),  // doesn't work
		[]byte("  4"),
		[]byte("4  "),
		[]byte("42"),
	}

	for i := 0; i < len(targets); i++ {
		fmt.Printf("Case %v:\n", i+1)
		// ?? probably copies a slice of bytes to a string with copying .Cap attribute
		// ?? because we can try to read its pointer as []byte and change the values inside the string
		inputString := string(targets[i])

		fmt.Printf("Input: |%v|\n", inputString)
		RemoveExtraSpaces(&inputString)
		fmt.Printf("Output: |%v|\n\n", inputString)
	}
}

func RemoveExtraSpaces(input *string) {
	var space byte = ' '
	fromIndex := 0           // copy from this index
	toIndex := 0             // copy to this index
	isPreviousSpace := false // set true when a single space is written
	originalLength := len(*input)

	target := (*[0x7fff0000]byte)( // unsafe usage a string as a slice of bytes
		unsafe.Pointer((*reflect.StringHeader)(unsafe.Pointer(input)).Data))[:len(*input):len(*input)]

	// step 1: skipping spaces at the begining
	for ; fromIndex < originalLength && target[fromIndex] == space; fromIndex++ {
	}

	// step 2: processing all chars after possible spaces
	for ; fromIndex < originalLength; fromIndex++ {
		if target[fromIndex] != space { // copies 'fromIndex' to 'toIndex'
			target[toIndex] = target[fromIndex]
			toIndex++
			isPreviousSpace = false
		} else {
			if !isPreviousSpace { //puts one space if the previous is not a space
				target[toIndex] = space
				toIndex++
				isPreviousSpace = true //stops appearing more than 1 space
			}
		}
	}
	// step 3: cutting the string inplace
	slicePtr := (*reflect.SliceHeader)(unsafe.Pointer(input))
	if toIndex > 1 && isPreviousSpace {
		slicePtr.Len = toIndex - 1
	} else {
		slicePtr.Len = toIndex
	}
}
