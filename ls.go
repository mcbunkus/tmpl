package main

import (
	"context"
	"fmt"
	"os"
	"sort"
	"strings"
	"text/tabwriter"

	"github.com/BurntSushi/toml"
	"github.com/urfave/cli/v3"
)

func (app *app) ls() *cli.Command {
	return &cli.Command{
		Name:  "ls",
		Usage: "List templates",
		Action: func(ctx context.Context, c *cli.Command) (err error) {
			templates, err := app.getAllTemplates()
			if err != nil {
				return fmt.Errorf("failed to read templates directory: %w", err)
			}

			table := tabwriter.NewWriter(os.Stdout, 0, 0, 4, ' ', 0)
			defer table.Flush()

			fmt.Fprintf(table, "template\targuments\n")
			fmt.Fprintf(table, "--------\t---------\n")
			for _, name := range templates {

				file, err := app.templateReader(name)
				if err != nil {
					return fmt.Errorf("failed to read %q while listing templates: %w", name, err)
				}

				decoder := toml.NewDecoder(file)
				spec := spec{}

				if _, err := decoder.Decode(&spec); err != nil {
					file.Close()
					return fmt.Errorf("failed to unmarshal template: %w", err)
				}

				if err := file.Close(); err != nil {
					return fmt.Errorf("failed to close %q: %w", name, err)
				}

				keys := make([]string, 0, len(spec.Variables))
				for key := range spec.Variables {
					keys = append(keys, key)
				}
				sort.Strings(keys)

				vars := make([]string, len(keys))
				for i, key := range keys {
					vars[i] = fmt.Sprintf("%q: %q", key, spec.Variables[key])
				}

				fmt.Fprintf(table, "%s\t{ %s }\n", name, strings.Join(vars, ", "))
			}

			return nil
		},
	}
}
