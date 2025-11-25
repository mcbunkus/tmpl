package main

import (
	"context"

	"github.com/urfave/cli/v3"
)

func (app *app) gen() *cli.Command {
	return &cli.Command{
		Name:  "gen",
		Usage: "Generate a template",
		Arguments: []cli.Argument{
			&cli.StringArg{
				Name: "template",
			},
		},
		Action: func(ctx context.Context, c *cli.Command) error {
			return nil
		},
	}
}
