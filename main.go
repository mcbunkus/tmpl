package main

import (
	"context"
	"log"
	"os"
	"path/filepath"

	"github.com/urfave/cli/v3"
)

func main() {
	userConfigDir, err := os.UserConfigDir()
	if err != nil {
		log.Fatalf("there was an issue getting your config directory: %s", err.Error())
	}

	templatesDir := filepath.Join(userConfigDir, "tmpl", "templates")
	if err := os.MkdirAll(templatesDir, 0o755); err != nil {
		log.Fatalf("there was an issue creating your templates directory: %s", err.Error())
	}

	app := app{templatesDir}

	cmd := &cli.Command{
		Commands: []*cli.Command{
			app.ls(),
			app.gen(),
			app.new(),
		},
	}

	if err := cmd.Run(context.Background(), os.Args); err != nil {
		log.Fatalln(err.Error())
	}
}
