package cmd

import (
	"errors"

	"github.com/kawana77b/op/internal/git"
	"github.com/kawana77b/op/internal/path"
	"github.com/kawana77b/op/internal/values"

	"github.com/spf13/cobra"
)

var gitCmd = &cobra.Command{
	Use:     "git",
	Short:   "Open the git remote repository in the current directory or the directory given as argument in a browser",
	Long:    `Open the git remote repository in the current directory or the directory given as argument in a browser`,
	Args:    cobra.MatchAll(cobra.MaximumNArgs(0), validateArg()),
	PreRunE: checkOS,
	RunE:    runGit,
}

func init() {
	rootCmd.AddCommand(gitCmd)
}

func runGit(cmd *cobra.Command, args []string) error {
	dirPath := values.New(args).Get(0)

	url, err := getGitRemoteUrl(dirPath)
	if err != nil {
		return err
	}

	p := path.NewPath(url)
	return p.Open()
}

func getGitRemoteUrl(dir string) (string, error) {
	p := path.NewPath(dir)
	dir = p.Path()
	if err := git.DirExists(dir); err != nil {
		return "", err
	}

	tmpUrl, err := git.GetOriginUrl(dir)
	if err != nil {
		return "", err
	}

	url, err := genRemoteUrlFromTmp(tmpUrl)
	if err != nil {
		return "", err
	}

	return url, err
}

func genRemoteUrlFromTmp(tmpUrl string) (string, error) {
	p := path.NewPath(tmpUrl)
	if p.IsWebAddr() {
		return git.TrimDotGit(p.Path()), nil
	}

	if path := p.Path(); git.IsSSHAddr(path) {
		url := git.SSHToUrls(path)["https"]
		return url, nil
	}

	return "", errors.New("incorrect path")
}
