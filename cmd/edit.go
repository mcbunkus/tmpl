package cmd

import (
	"errors"
	"os"
	"os/exec"
	"path/filepath"
	"syscall"
	"tmpl/lib"

	"github.com/spf13/cobra"
)

const edDesc = `Edit an existing template in the configured template directory.
An EDITOR environment variable must be set for this to work.`

// editCmd represents the edit command
var editCmd = &cobra.Command{
	Use:     "edit",
	Aliases: []string{"ed"},
	Short:   "Edit an existing template",
	Long:    edDesc,
	RunE: func(cmd *cobra.Command, args []string) error {
		editor := os.Getenv("EDITOR")
		if len(editor) == 0 {
			return errors.New("EDITOR environment variable isn't defined")
		}

		name, err := getTemplateName(args)
		if err != nil {
			return err
		}

		tmplDir, err := lib.TemplateDir()
		if err != nil {
			return err
		}

		path := filepath.Join(tmplDir, name)

		binary, err := exec.LookPath(editor)
		if err != nil {
			return err
		}

		env := os.Environ()
		edArgs := []string{editor, path}

		return syscall.Exec(binary, edArgs, env)

	},
}

func init() {
	rootCmd.AddCommand(editCmd)
}
