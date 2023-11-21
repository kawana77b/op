package cmd

import (
	"errors"
	"fmt"
	"os"

	"github.com/kawana77b/op/internal/env"
	"github.com/kawana77b/op/internal/path"
	"github.com/kawana77b/op/internal/values"

	"github.com/spf13/cobra"
)

const (
	APP_VERSION string = "v0.0.1"
)

var rootCmd = &cobra.Command{
	Use:     "op",
	Version: APP_VERSION,
	Short:   "",
	Long: `Open the file path or web address in the prescribed file explorer or browser

- This command works only on Windows, Mac, and Linux.
- If no arguments are given, it opens the current directory with the specified filer.
- If a file path is given, the directory will be opened with the specified filer.
`,
	Args:    cobra.MatchAll(cobra.MaximumNArgs(1), validateArg()),
	PreRunE: checkOS,
	RunE:    runRoot,
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.CompletionOptions.DisableDefaultCmd = true
}

func validateArg() cobra.PositionalArgs {
	var maxLen uint = 250
	return func(cmd *cobra.Command, args []string) error {
		if len(args) > 0 && len(args[0]) > int(maxLen) {
			msg := fmt.Sprintf("argument must be less than or equal to %d characters", maxLen)
			return errors.New(msg)
		}

		return nil
	}
}

func checkOS(cmd *cobra.Command, args []string) error {
	if ok, _ := env.GetOS(); !ok {
		return errors.New("OS is not supported")
	}

	return nil
}

func runRoot(cmd *cobra.Command, args []string) error {
	arg := values.New(args).Get(0)
	p := path.NewPath(arg)
	return p.Open()
}
