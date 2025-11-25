package main

import (
	"io"
	"os"
	"path/filepath"
)

type app struct {
	templatesDir string
}

func (app *app) templateReader(name string) (io.ReadCloser, error) {
	templatePath := filepath.Join(app.templatesDir, name)
	return os.Open(templatePath)
}

func (app *app) templateWriter(name string) (io.WriteCloser, error) {
	templatePath := filepath.Join(app.templatesDir, name)
	return os.Create(templatePath)
}

func (app *app) templateExists(name string) bool {
	templatePath := filepath.Join(app.templatesDir, name)
	_, err := os.Stat(templatePath)
	return err == nil
}

func (app *app) getAllTemplates() ([]string, error) {
	dirEntries, err := os.ReadDir(app.templatesDir)
	if err != nil {
		return nil, err
	}

	entries := make([]string, 0, len(dirEntries))

	for _, entry := range dirEntries {
		if !entry.IsDir() {
			entries = append(entries, entry.Name())
		}
	}

	return entries, nil
}
