package path

import (
	"errors"
	"os"
	"path/filepath"
	"regexp"
	"strings"

	"github.com/kawana77b/op/internal/env"
	"github.com/kawana77b/op/internal/proc"
)

type Path struct {
	os   string
	path string
}

func NewPath(path string) *Path {
	_, _os := env.GetOS()

	_path := strings.TrimSpace(path)
	if len(_path) == 0 {
		_path, _ = os.Getwd()
	}

	return &Path{
		os:   _os,
		path: _path,
	}
}

func (p *Path) Path() string {
	return p.path
}

func (p *Path) Open() error {
	if p.IsWebAddr() {
		return p.openByBrowser()
	} else {
		return p.openByFiler()
	}
}

// Webアドレスかどうか調べます
func (p *Path) IsWebAddr() bool {
	for _, exp := range []*regexp.Regexp{
		regexp.MustCompile("^http://"),
		regexp.MustCompile("^https://"),
	} {
		if exp.MatchString(p.path) {
			return true
		}
	}

	return false
}

// ファイラでパスを開きます
func (p *Path) openByFiler() error {
	filer := env.GetFiler(p.os)

	dir, err := p.getDirPath()
	if err != nil {
		return err
	}

	return proc.ExecCmd(filer, dir)
}

// ブラウザでパスを開きます
func (p *Path) openByBrowser() error {
	browser := env.GetBrowser(p.os)

	cmd := browser[0]
	args := append(browser[1:], p.path)

	return proc.ExecCmd(cmd, args...)
}

// ディレクトリパスを取得します
func (p *Path) getDirPath() (string, error) {
	path := filepath.Clean(p.path)
	fi, err := os.Stat(path)

	if !os.IsNotExist(err) {
		if fi.IsDir() {
			return path, nil
		} else {
			return filepath.Dir(path), nil
		}
	}

	return "", errors.New("incorrect file path")
}
