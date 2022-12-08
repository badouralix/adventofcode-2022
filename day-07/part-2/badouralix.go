package main

import (
	"fmt"
	"io"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
)

type Filesystem struct {
	Name string
	Size int64
	Type string

	Files  map[string]*Filesystem
	Parent *Filesystem
}

func NewFilesystemFromInput(s string) Filesystem {
	root := Filesystem{Name: "/", Type: "d", Files: make(map[string]*Filesystem)}
	pwd := &root

	for _, line := range strings.Split(s, "\n") {
		if line == "$ cd /" {
			pwd = &root
		} else if line == "$ cd .." {
			pwd = pwd.Parent
		} else if strings.HasPrefix(line, "$ cd ") {
			dirname := line[5:]
			pwd = pwd.Files[dirname]
		} else if line == "$ ls" {
			continue
		} else if strings.HasPrefix(line, "dir ") {
			dirname := line[4:]
			pwd.Files[dirname] = &Filesystem{Name: dirname, Type: "d", Files: make(map[string]*Filesystem), Parent: pwd}
		} else {
			split := strings.Split(line, " ")
			size, _ := strconv.ParseInt(split[0], 10, 64)
			filename := split[1]
			pwd.Files[filename] = &Filesystem{Name: filename, Type: "-", Size: size}
		}
	}

	return root
}

func (fs Filesystem) SmallestDirectoryThatWouldFreeUpEnoughSpace(minSize int64) int64 {
	if fs.Type != "d" || fs.Size < minSize {
		return math.MaxInt64
	}

	size := fs.Size

	for _, file := range fs.Files {
		smallestDirectoryThatWouldFreeUpEnoughSpace := file.SmallestDirectoryThatWouldFreeUpEnoughSpace(minSize)
		if smallestDirectoryThatWouldFreeUpEnoughSpace < size {
			size = smallestDirectoryThatWouldFreeUpEnoughSpace
		}
	}

	return size
}

func (fs *Filesystem) UpdateSize() {
	if fs.Size != 0 {
		return
	}

	size := int64(0)
	for _, file := range fs.Files {
		file.UpdateSize()
		size += file.Size
	}
	fs.Size = size
}

func (fs Filesystem) String() string {
	return fs.string(0)
}

func (fs Filesystem) string(indent int64) string {
	if fs.Type != "d" {
		return fmt.Sprintf("%s- %s (file, size=%d)\n", strings.Repeat(" ", int(indent)), fs.Name, fs.Size)
	}

	var builder strings.Builder
	builder.WriteString(fmt.Sprintf("%s- %s (dir)\n", strings.Repeat(" ", int(indent)), fs.Name))
	for _, file := range fs.Files {
		builder.WriteString(file.string(indent + 2))
	}
	return builder.String()
}

func run(s string) int64 {
	root := NewFilesystemFromInput(s)
	root.UpdateSize()
	return root.SmallestDirectoryThatWouldFreeUpEnoughSpace(30_000_000 + root.Size - 70_000_000)
}

func main() {
	// Uncomment this line to disable garbage collection
	// debug.SetGCPercent(-1)

	var input []byte
	var err error
	if len(os.Args) > 1 {
		// Read input from file for local debugging
		input, err = os.ReadFile(os.Args[1])
		if err != nil {
			panic(err)
		}
		// Remove extra newline
		input = input[:len(input)-1]
	} else {
		// Read input from stdin
		input, err = io.ReadAll(os.Stdin)
		if err != nil {
			panic(err)
		}
	}

	// Start resolution
	start := time.Now()
	result := run(string(input))

	// Print result
	fmt.Printf("_duration:%f\n", time.Since(start).Seconds()*1000)
	fmt.Println(result)
}
