package config

import (
	"log"
	"os"
)

// Directory consists of its name and nested templates, which could either be
// Files and/or Directories
type Directory struct {
	Name      string
	Templates []Template
}

// Build will create files and nested directories.
func (d Directory) Build(c *Config) {
	if err := os.MkdirAll(d.Name, DefaultPermissions); err != nil {
		log.Println(err)
		return
	}

	if err := os.Chdir(d.Name); err != nil {
		log.Println(err)
		return
	}

	for _, dir := range d.Templates {
		dir.Build(c)
	}

	os.Chdir("..")
}
