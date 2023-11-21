package env

import "runtime"

const (
	ENV_WINDOWS = "windows"
	ENV_MAC     = "darwin"
	ENV_LINUX   = "linux"
)

// OS情報を取得します. 対応OSのみtrueを返し、それ以外はfalseを返します
func GetOS() (ok bool, os string) {
	_os := runtime.GOOS
	if _os == ENV_WINDOWS || _os == ENV_MAC || _os == ENV_LINUX {
		return true, _os
	}

	return false, ""
}

// ファイラとなるコマンドを取得します
func GetFiler(os string) string {
	filers := map[string]string{
		ENV_WINDOWS: "explorer.exe",
		ENV_MAC:     "open",
		ENV_LINUX:   "xdg-open",
	}

	filer, ok := filers[os]
	if !ok {
		return ""
	}

	return filer
}

// ブラウザとなるコマンドおよび引数情報を取得します
func GetBrowser(os string) []string {
	browsers := map[string][]string{
		ENV_WINDOWS: {"rundll32.exe", "url.dll,FileProtocolHandler"},
		ENV_MAC:     {"open"},
		ENV_LINUX:   {"xdg-open"},
	}

	browser, ok := browsers[os]
	if !ok {
		return []string{}
	}

	return browser
}
