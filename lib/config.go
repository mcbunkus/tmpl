package lib

import (
	"bytes"
	"io"

	"gopkg.in/yaml.v3"
)

// DefaultPermissions is the default permissions for generated templates.
const DefaultPermissions = 0o755

type Template struct {
	Files []File
	Dirs  []Dir
}

// TemplateConfig describes the structure of the configuration template.
type TemplateConfig struct {
	Name      string
	Template  Template
	Variables map[string]any
}

// Build will build all of the templates defined in Templates.
func (c *TemplateConfig) Build() {
	c.Template.Build(c)
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

func (t *Template) Build(c *TemplateConfig) {
	for _, f := range t.Files {
		f.Build(c)
	}

	for _, d := range t.Dirs {
		d.Build(c)
	}

}
