package lib

import (
	"os"
	"os/user"
	"path/filepath"
)

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

func TemplateDir() (string, error) {
	workDir, err := WorkDirectory()
	if err != nil {
		return "", err
	}
	templateDir := filepath.Join(workDir, "templates")
	err = os.MkdirAll(templateDir, 0o755)
	return templateDir, err
}
