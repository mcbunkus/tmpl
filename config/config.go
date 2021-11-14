package config

import (
	"log"
	"os"

	"gopkg.in/yaml.v2"
)

// DefaultPermissions is the default permissions for generated templates.
const DefaultPermissions = 0o755

// Template describes an object that can be build from the yaml file.
type Template interface {
	// Build will construct the template. It takes in a pointer to a Config
	// struct for context. For instance, File uses this to read in the variables
	// to Go's text/template package.
	Build(c *Config)
}

// Config describes the structure of the configuration template.
type Config struct {
	Name      string
	Templates []Template
	Variables map[string]interface{}
}

// Build will build all of the templates defined in Templates.
func (c *Config) Build() {
	for _, temp := range c.Templates {
		temp.Build(c)
	}
}

// NewConfig will generate a new Config from the given config file's path.
func NewConfig(fpath string) (conf *Config, err error) {
	data, err := os.ReadFile(fpath)
	if err != nil {
		return
	}

	if err := yaml.Unmarshal(data, conf); err != nil {
		log.Fatalln(err)
	}

	return

}
