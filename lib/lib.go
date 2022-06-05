package lib

import (
	"fmt"
	"os"
	"os/user"
	"path/filepath"
)

// WorkDirectory returns the path to the working directory,
// by default in $HOME/.config/tmpl.
func WorkDirectory() (string, error) {
	user, err := user.Current()
	if err != nil {
		return "", err
	}

	path := filepath.Join(user.HomeDir, ".config", "tmpl")

	if _, err := os.Stat(path); os.IsNotExist(err) {
		if err := os.MkdirAll(path, 0o755); err != nil {
			return "", err
		}
	} else if err != nil {
		return "", err
	}
	return path, nil
}

// TemplateDir returns the path to the template directory.
func TemplateDir() (string, error) {
	workDir, err := WorkDirectory()
	if err != nil {
		return "", err
	}
	templateDir := filepath.Join(workDir, "templates")
	err = os.MkdirAll(templateDir, 0o755)
	return templateDir, err
}

// wrapError appends a new error to the end of an existing one,
// so the user will get a list of everything that went wrong,
// instead of getting them one at a time.
func wrapError(err, newError error) error {
	if newError == nil {
		return err
	}
	return fmt.Errorf("%w\n%s", err, newError)
}
