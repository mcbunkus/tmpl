package config

import (
	"log"
	"os"
)

// Dir consists of its name and nested templates, which could either be
// Files and/or Directories
type Dir struct {
	Name string
	Template
}

// Build will create files and nested directories.
func (d *Dir) Build(c *Config) {
	if err := os.MkdirAll(d.Name, DefaultPermissions); err != nil {
		log.Println(err)
		return
	}

	if err := os.Chdir(d.Name); err != nil {
		log.Println(err)
		return
	}

	for _, file := range d.Files {
		file.Build(c)
	}

	for _, dir := range d.Dirs {
		dir.Build(c)
	}

	os.Chdir("..")
}
