package lib

import (
	"os"
)

// Dir consists of its name and nested templates, which could either be
// Files and/or Directories
type Dir struct {
	Name string `yaml:"name"`

	// the yaml parser doesn't like embedded structs :(
	Files []File `yaml:"files"`
	Dirs  []Dir  `yaml:"dirs"`
}

// Build will create files and nested directories.
func (d *Dir) Build(c *Spec) error {

	var err error = nil
	if err := os.MkdirAll(d.Name, DefaultPermissions); err != nil {
		return err
	}

	if err := os.Chdir(d.Name); err != nil {
		return err
	}

	for _, file := range d.Files {
		if ferr := file.Build(c); ferr != nil {
			err = wrapError(err, ferr)
		}
	}

	for _, dir := range d.Dirs {
		if derr := dir.Build(c); derr != nil {
			err = wrapError(err, derr)
		}
	}

	err = wrapError(err, os.Chdir(".."))
	return err
}
