package lib

import (
	"fmt"
	"io/fs"
	"os"
	"os/user"
	"path/filepath"
	"strings"

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

	path := filepath.Join(user.HomeDir, ".tmpl")

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

type BadKeyValueArg struct {
	Arg string
}

func (b BadKeyValueArg) Error() string {
	return fmt.Sprintf("could not parse %s", b.Arg)
}

// KeyValueMap takes a slice of strings with the format "key=value" and converts
// them into a map.
func KeyValueMap(args []string) (map[string]string, error) {
	kvMap := make(map[string]string)
	for _, arg := range args {
		split := strings.Split(arg, "=")
		if len(split) != 2 {
			return kvMap, BadKeyValueArg{arg}
		}
		key, value := split[0], split[1]
		kvMap[key] = value
	}
	return kvMap, nil
}

// MergeMaps takes an oldMap and a newMap, and merges the newMap into the old one.
// This will overwrite keys in the old map, and this is intended, so users can
// overwrite variables in the config if they want to
func MergeMaps(oldMap, newMap map[string]string) map[string]string {
	for k, v := range newMap {
		oldMap[k] = v
	}
	return oldMap
}
