/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

*/
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

// edCmd represents the ed command
var edCmd = &cobra.Command{
	Use:   "ed",
	Short: "Edit an existing template in the template directory",
	Long:  "An EDITOR environment variable must be set for this to work",
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
	rootCmd.AddCommand(edCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// edCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// edCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
