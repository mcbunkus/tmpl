/*
Copyright © 2021 NAME HERE <EMAIL ADDRESS>

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/
package cmd

import (
	"fmt"
	"log"
	"os"
	"tmpl/config"

	"github.com/spf13/cobra"
)

var (
	createBlank bool
)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:        "tmpl",
	Short:      "Generate projects from yaml templates",
	Args:       cobra.ExactArgs(1),
	ArgAliases: []string{"template-file"},
	Run: func(cmd *cobra.Command, args []string) {
		conf, err := config.NewConfig(args[0])
		if err != nil {
			log.Fatalln(err)
		}
		conf.Build()
	},
}

// Execute adds all child commands to the root command and sets flags appropriately.
// This is called by main.main(). It only needs to happen once to the rootCmd.
func Execute() {
	if err := rootCmd.Execute(); err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
}

func init() {
	rootCmd.Flags().BoolVarP(&createBlank, "create-blank", "c", false, "Create a blank template file")
}
