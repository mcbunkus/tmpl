package config

import (
	"log"
	"os"
	"text/template"
)

// File describes a file with a name and optionally it's content. Go template
// syntax can be used to fill in variables defined in the variables section in
// the config file.
type File struct {
	Name    string
	Content string
}

// Build will create the file and write its contents. Templated variables will
// also be filled in, <no value> will be written in invalid variables.
func (f File) Build(c *Config) {
	textTmpl, err := template.New(f.Name).Parse(f.Content)
	if err != nil {
		log.Printf("Failed to fill in template for %s:\n\n\t%s\n\n", f.Name, err.Error())
		return
	}

	file, err := os.Create(f.Name)
	if err != nil {
		log.Printf("Failed to create %s:\n\n\t%s\n\n", f.Name, err.Error())
		return
	}

	defer file.Close()

	if err := textTmpl.Execute(file, c.Variables); err != nil {
		log.Printf("Failed executing template for %s:\n\n\t%s\n\n", f.Name, err.Error())
	}

}
