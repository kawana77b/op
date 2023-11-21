package git_test

import (
	"fmt"
	"strings"
	"testing"

	"github.com/kawana77b/op/internal/git"
)

func Test_IsGitSSHAddr(t *testing.T) {
	ssh := "git@github.com:ruby/ruby.git"

	if !git.IsSSHAddr(ssh) {
		t.Errorf("not git ssh address?: %s\n", ssh)
	}
}

func Test_GitSSHToUrl(t *testing.T) {
	ssh := "git@github.com:ruby/ruby.git"
	urls := git.SSHToUrls(ssh)
	for _, v := range urls {
		fmt.Printf("generated url: %s\n", v)

		if !strings.Contains(v, "http") || strings.Contains(v, ".git") {
			t.Errorf("不正なURLです %s\n", v)
		}
	}
}
