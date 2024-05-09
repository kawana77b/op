package main

import "github.com/kawana77b/op/cmd"

var version string = "0.0.1"

func main() {
	cmd.Version = version
	cmd.Execute()
}
