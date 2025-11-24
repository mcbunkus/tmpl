package lib

import (
	"io"

	"github.com/BurntSushi/toml"
)

// Spec describes the structure of the project template.
type Spec struct {
	Variables map[string]string `toml:"variables"`
	Templates []Template        `toml:"template"`
}

// Build will build all of the templates defined in Templates.
func (c *Spec) Build() error {
	for _, template := range c.Templates {
		if err := template.Build(c); err != nil {
			return err
		}
	}
	return nil
}

// NewSpec will generate a new Config from the given config file's path.
func NewSpec(reader io.Reader) (*Spec, error) {
	conf := new(Spec)
	decoder := toml.NewDecoder(reader)

	if _, err := decoder.Decode(conf); err != nil {
		return nil, err
	}

	return conf, nil
}
