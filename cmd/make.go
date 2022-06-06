package cmd

import (
	"fmt"
	"os"
	"path/filepath"
	"tmpl/lib"

	"github.com/spf13/cobra"
)

const mkDesc = `
Make a new template file in the template directory with a default template.
`

// makeCmd represents the make command
var makeCmd = &cobra.Command{
	Use:     "make",
	Aliases: []string{"mk"},
	Short:   "Generate a blank template",
	Long:    mkDesc,
	Args:    cobra.ExactArgs(1),
	RunE: func(cmd *cobra.Command, args []string) error {
		tmplName := args[0]
		tmplDir, err := lib.TemplateDir()
		if err != nil {
			return err
		}

		path := filepath.Join(tmplDir, tmplName)
		if err := os.WriteFile(path, []byte(lib.BlankConfig), 0o666); err != nil {
			return err
		}

		fmt.Printf("generated %s, generate with \"tmpl gen %s\" :)\n", tmplName, tmplName)
		return nil
	},
}

func init() {
	rootCmd.AddCommand(makeCmd)
}
