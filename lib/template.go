package lib

// DefaultPermissions is the default permissions for generated templates.
const DefaultPermissions = 0o755

// Template defines
type Template struct {
	Files []File `yaml:"files"`
	Dirs  []Dir  `yaml:"dirs"`
}

func (t *Template) Build(c *Spec) error {
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
