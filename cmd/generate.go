package cmd

import (
	"fmt"
	"os"
	"path/filepath"
	"tmpl/lib"

	"github.com/spf13/cobra"
)

const genDesc = `
Generate a project from a template file. A template can be specified as an argument, 
or it can be selected interactively if no argument is given.
`

// generateCmd represents the generate command
var generateCmd = &cobra.Command{
	Use:     "generate",
	Aliases: []string{"gen"},
	Short:   "Generate a project",
	Long:    genDesc,
	RunE: func(cmd *cobra.Command, args []string) (err error) {
		userArgs := make(map[string]string)
		if len(args) > 1 {
			userArgs, err = lib.KeyValueMap(args[1:])
			if err != nil {
				return err
			}
		}

		name, err := getTemplateName(args)
		if err != nil {
			return
		}

		tmplDir, err := lib.TemplateDir()
		if err != nil {
			return
		}

		fname := filepath.Join(tmplDir, name)

		if _, err := os.Stat(fname); os.IsNotExist(err) {
			name := filepath.Base(fname)
			return fmt.Errorf("%s doesn't exist, generate it with \"tmpl gen %s\"", name, name)
		}

		file, err := os.Open(fname)
		if err != nil {
			return
		}

		conf, err := lib.NewSpec(file)
		if err != nil {
			return
		}

		if len(userArgs) != 0 {
			conf.Variables = lib.MergeMaps(conf.Variables, userArgs)
		}

		return conf.Build()
	},
}

func getTemplateName(args []string) (name string, err error) {
	if len(args) == 0 {
		name, err = lib.PromptForTemplate()
		if err != nil {
			return
		}
	} else {
		name = args[0]
	}
	return
}

func init() {
	rootCmd.AddCommand(generateCmd)
}
