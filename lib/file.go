package lib

import (
	"fmt"
	"os"
	"strings"
	"text/template"
)

var funcMap = template.FuncMap{
	"yell":  strings.ToUpper,
	"title": strings.Title,
}

// File describes a file with a name and optionally it's content. Go template
// syntax can be used to fill in variables defined in the variables section in
// the config file.
type File struct {
	Name    string
	Content string
}

// Build will create the file and write its contents. Templated variables will
// also be filled in, <no value> will be written in invalid variables.
func (f *File) Build(c *Spec) error {
	textTmpl, err := template.New(f.Name).Funcs(funcMap).Parse(f.Content)
	if err != nil {
		return fmt.Errorf("%s template failed: %s", f.Name, err.Error())
	}

	file, err := os.Create(f.Name)
	if err != nil {
		return fmt.Errorf("failed to create %s: %s", f.Name, err.Error())
	}

	defer file.Close()

	if err := textTmpl.Execute(file, c.Variables); err != nil {
		fmt.Errorf("failed executing %s template: %s", f.Name, err.Error())
	}

	return nil

}
