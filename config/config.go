package config

import (
	"log"
	"os"

	"gopkg.in/yaml.v3"
)

// DefaultPermissions is the default permissions for generated templates.
const DefaultPermissions = 0o755

type Template struct {
	Files []File
	Dirs  []Dir
}

// Config describes the structure of the configuration template.
type Config struct {
	Name      string
	Template  Template
	Variables map[string]interface{}
}

// Build will build all of the templates defined in Templates.
func (c *Config) Build() {
	c.Template.Build(c)
}

// NewConfig will generate a new Config from the given config file's path.
func NewConfig(fpath string) (conf *Config, err error) {
	conf = new(Config)
	data, err := os.ReadFile(fpath)
	if err != nil {
		return
	}

	if err := yaml.Unmarshal(data, conf); err != nil {
		log.Fatalln(err)
	}

	return conf, nil

}

func (t *Template) Build(c *Config) {
	for _, f := range t.Files {
		f.Build(c)
	}

	for _, d := range t.Dirs {
		d.Build(c)
	}

}
