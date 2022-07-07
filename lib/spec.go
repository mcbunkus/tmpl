package lib

import (
	"bytes"
	"io"

	"gopkg.in/yaml.v3"
)

// Spec describes the structure of the project template.
type Spec struct {
	Variables map[string]string `yaml:"variables"`
	Template  Template          `yaml:"template"`
}

// Build will build all of the templates defined in Templates.
func (c *Spec) Build() error {
	return c.Template.Build(c)
}

// NewSpec will generate a new Config from the given config file's path.
func NewSpec(reader io.Reader) (conf *Spec, err error) {
	conf = new(Spec)
	buffer := new(bytes.Buffer)
	if _, err = buffer.ReadFrom(reader); err != nil {
		return
	}

	if err = yaml.Unmarshal(buffer.Bytes(), conf); err != nil {
		return
	}

	return

}
