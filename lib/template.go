package lib

import (
	"fmt"
	"os"
	"path/filepath"
	"text/template"
)

// DefaultPermissions is the default permissions for generated templates.
const DefaultPermissions = 0o755

// Template defines
type Template struct {
	Name    string `toml:"name"`
	Content string `toml:"content"`
}

func (t *Template) Build(c *Spec) error {
	textTmpl, err := template.New(t.Name).Parse(t.Content)
	if err != nil {
		return fmt.Errorf("%s template failed: %s", t.Name, err.Error())
	}

	// first things first, try to create the full path to the file
	dir := filepath.Dir(t.Name)

	if err := os.MkdirAll(dir, 0o755); err != nil {
		return fmt.Errorf("%s template failed: %s", t.Name, err.Error())
	}

	file, err := os.Create(t.Name)
	if err != nil {
		return fmt.Errorf("failed to create %s: %s", t.Name, err.Error())
	}

	if err := textTmpl.Execute(file, c.Variables); err != nil {
		return fmt.Errorf("failed executing %s template: %s", t.Name, err.Error())
	}

	return file.Close()
}
