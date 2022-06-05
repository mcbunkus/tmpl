/*
Copyright © 2022 NAME HERE <EMAIL ADDRESS>

*/
package cmd

import (
	"fmt"
	"io/fs"
	"path/filepath"
	"tmpl/lib"

	"github.com/spf13/cobra"
)

// lsCmd represents the ls command
var lsCmd = &cobra.Command{
	Use:   "ls",
	Short: "List templates in template directory",
	RunE: func(cmd *cobra.Command, args []string) error {
		tmplDir, err := lib.TemplateDir()
		if err != nil {
			return err
		}

		names := []string{}
		filepath.WalkDir(tmplDir, func(path string, d fs.DirEntry, err error) error {
			if d.IsDir() {
				return nil
			}

			if err != nil {
				return err
			}

			names = append(names, d.Name())

			return nil
		})

		for _, name := range names {
			fmt.Println(name)
		}
		return nil
	},
}

func init() {
	rootCmd.AddCommand(lsCmd)

	// Here you will define your flags and configuration settings.

	// Cobra supports Persistent Flags which will work for this command
	// and all subcommands, e.g.:
	// lsCmd.PersistentFlags().String("foo", "", "A help for foo")

	// Cobra supports local flags which will only run when this command
	// is called directly, e.g.:
	// lsCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}
