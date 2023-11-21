package git

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"regexp"
	"strings"

	"github.com/kawana77b/op/internal/util"
)

// 指定ディレクトリに.gitディレクトリがあるか見る。ない場合error
func DirExists(dir string) error {
	git := filepath.Join(dir, ".git")
	if !util.DirExists(git) {
		return errors.New("the .git directory does not exist")
	}

	return nil
}

// git config --get remote.origin.urlを実行し、その出力を返す
func GetOriginUrl(dir string) (string, error) {
	wd, _ := os.Getwd()
	defer func() {
		os.Chdir(wd)
	}()

	os.Chdir(dir)

	git, err := exec.LookPath("git")
	if err != nil {
		return "", errors.New("git is not installed")
	}

	args := []string{"config", "--get", "remote.origin.url"}
	buf, err := exec.Command(git, args...).Output()
	if err != nil {
		return "", errors.New("remote repository URL is not set")
	}

	url := string(buf)
	if len(url) == 0 {
		return "", errors.New("remote repository URL is not set")
	}

	return url, nil
}

// git@...のアドレスかどうか見る
func IsSSHAddr(addr string) bool {
	return regexp.MustCompile("^git@.+:.+.git$").MatchString(addr)
}

// .gitを取り除く
func TrimDotGit(url string) string {
	return regexp.MustCompile(".git$").ReplaceAllString(url, "")
}

// git sshアドレスからurl候補を生成
func SSHToUrls(ssh string) map[string]string {
	sshexp := regexp.MustCompile("^git@")

	domain := strings.Replace(sshexp.ReplaceAllString(ssh, ""), ":", "/", 1)
	domain = TrimDotGit(domain)

	return map[string]string{
		"https": "https://" + domain,
		"http":  "http://" + domain,
	}
}
