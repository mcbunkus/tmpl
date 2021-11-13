package main

import (
	"fmt"
	"log"
	"os"

	"gopkg.in/yaml.v2"
)

const DefaultPermissions = 0o755

type File struct {
	Name    string
	Content string `yaml:",flow"`
}

type Directory struct {
	Name        string
	Files       []File
	Directories []Directory
}

type Template struct {
	Files       []File
	Directories []Directory
}

type TemplateConfig struct {
	Name     string
	Template Template
}

func (t Template) Build() {
	for _, file := range t.Files {
		file.Build()
	}

	for _, dir := range t.Directories {
		dir.Build()
	}
}

func (d Directory) Build() {
	if err := os.MkdirAll(d.Name, DefaultPermissions); err != nil {
		log.Println(err)
		return
	}

	if err := os.Chdir(d.Name); err != nil {
		log.Println(err)
		return
	}

	for _, file := range d.Files {
		file.Build()
	}

	for _, dir := range d.Directories {
		dir.Build()
	}

	os.Chdir("..")
}

func (f File) Build() {
	if err := os.WriteFile(f.Name, []byte(f.Content), DefaultPermissions); err != nil {
		log.Println(err)
	}
}

func main() {

	data, err := os.ReadFile("test.yaml")
	if err != nil {
		log.Fatalln(err)
	}

	var tmpl TemplateConfig
	if err := yaml.Unmarshal(data, &tmpl); err != nil {
		log.Fatalln(err)
	}

	tmpl.Template.Build()

	fmt.Println(tmpl)
}
