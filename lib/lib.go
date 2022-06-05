package lib

import (
	"fmt"
	"io/fs"
	"os"
	"os/user"
	"path/filepath"

	"github.com/AlecAivazis/survey/v2"
)

// PromptForTemplate is a helper for interactively querying the user for what template they want to use.
func PromptForTemplate() (string, error) {
	templates, err := GetTemplates()
	if err != nil {
		return "", err
	}
	template := ""
	prompt := &survey.Input{Message: "What template do you want to use?", Suggest: func(toComplete string) []string { return templates }}
	err = survey.AskOne(prompt, &template)
	return template, err
}

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

func GetTemplates() ([]string, error) {
	names := []string{}

	tmplDir, err := TemplateDir()
	if err != nil {
		return names, err
	}

	err = filepath.WalkDir(tmplDir, func(path string, d fs.DirEntry, err error) error {
		if d.IsDir() {
			return nil
		}

		if err != nil {
			return err
		}

		names = append(names, d.Name())

		return nil
	})

	return names, err
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
