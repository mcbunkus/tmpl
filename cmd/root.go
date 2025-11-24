// Package cmd contains all the subcommand implementations
package cmd

import (
	"fmt"
	"os"

	"github.com/spf13/cobra"
)

const longDesc = `
Generate a project from a given yaml template. For example:

	tmpl generate <template-name> # tmpl gen <template-name> works too

will produce a new project in the current directory. Templates
consist of nested file and directory declarations. An example 
template can be generated using the make command. Run 

	tmpl make <template-name> # or tmpl mk <template-name>

to produce an example yaml template file. You can open it in 
your editor quickly with:

	tmpl edit <template-name>

if you have an EDITOR environment variable set.
`

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:   "tmpl",
	Short: "Generate projects from yaml templates",
	Long:  longDesc,
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}
