package main

import (
	"context"
	"fmt"
	"strings"

	"github.com/BurntSushi/toml"
	"github.com/urfave/cli/v3"
)

func (app *app) new() *cli.Command {
	return &cli.Command{
		Name:  "new",
		Usage: "Create a new template",
		Arguments: []cli.Argument{
			&cli.StringArg{
				Name: "name",
			},
		},
		Action: func(ctx context.Context, c *cli.Command) (err error) {
			name := strings.TrimSpace(c.StringArg("name"))

			if name == "" {
				return fmt.Errorf("a template name is required")
			}

			if app.templateExists(name) {
				return fmt.Errorf("failed to create new template: %q exists", name)
			}

			file, err := app.templateWriter(name)
			if err != nil {
				return fmt.Errorf("failed to open for writing while creating %q: %w", name, err)
			}

			// closing the file can also result in an error, so a defer file.Close() is not optimal
			defer func() {
				if cerr := file.Close(); cerr != nil && err == nil {
					err = fmt.Errorf("failed to close template file: %w", cerr)
				}
			}()

			blankSpec := spec{
				Variables: map[string]string{"key": "value"},
				Templates: []template{
					{Path: "test.txt", Body: "Hello, world!"},
				},
			}
			encoder := toml.NewEncoder(file)
			encoder.Indent = ""

			if err := encoder.Encode(blankSpec); err != nil {
				return fmt.Errorf("failed to create new template: %w", err)
			}

			return nil
		},
	}
}
