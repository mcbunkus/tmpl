package main

// template represents an entry in a template file. It contains the relative path to the resulting file and its content.
type template struct {
	Path string `toml:"path"`
	Body string `toml:"body"`
}

// spec represents a collection of templates, defined in a single toml file. Variables defined at the top are defaults, and entries are defined under [[template]] tags.
type spec struct {
	Variables map[string]string `toml:"variables"`
	Templates []template        `toml:"template"`
}
