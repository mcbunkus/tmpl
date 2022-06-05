package lib

import (
	"bytes"
	"io"

	"gopkg.in/yaml.v3"
)

// DefaultPermissions is the default permissions for generated templates.
const DefaultPermissions = 0o755

type Template struct {
	Files []File `yaml:"files"`
	Dirs  []Dir  `yaml:"dirs"`
}

// TemplateConfig describes the structure of the configuration template.
type TemplateConfig struct {
	Variables map[string]any `yaml:"variables"`
	Template  Template       `yaml:"template"`
}

// Build will build all of the templates defined in Templates.
func (c *TemplateConfig) Build() error {
	return c.Template.Build(c)
}

// NewTemplateConfig will generate a new Config from the given config file's path.
func NewTemplateConfig(reader io.Reader) (conf *TemplateConfig, err error) {
	conf = new(TemplateConfig)
	buffer := new(bytes.Buffer)
	if _, err = buffer.ReadFrom(reader); err != nil {
		return
	}

	if err = yaml.Unmarshal(buffer.Bytes(), conf); err != nil {
		return
	}

	return

}

func (t *Template) Build(c *TemplateConfig) error {
	var err error = nil
	for _, f := range t.Files {
		if ferr := f.Build(c); ferr != nil {
			err = wrapError(err, ferr)
		}
	}

	for _, d := range t.Dirs {
		if derr := d.Build(c); derr != nil {
			err = wrapError(err, derr)
		}
	}

	return err
}
