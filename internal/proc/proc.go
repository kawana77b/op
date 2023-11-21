package proc

import "os/exec"

// コマンドを実行します
func ExecCmd(cmd string, args ...string) error {
	err := exec.Command(cmd, args...).Start()
	if err != nil {
		return err
	}

	return nil
}
