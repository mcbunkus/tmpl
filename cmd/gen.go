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
	"log"
	"os"
	"tmpl/config"

	"github.com/spf13/cobra"
)

// genCmd represents the gen command
var genCmd = &cobra.Command{
	Use:   "gen",
	Short: "Generate a blank template",
	Args:  cobra.ExactArgs(1),
	Run: func(cmd *cobra.Command, args []string) {
		if err := os.WriteFile(args[0], []byte(config.BlankConfig), 0o666); err != nil {
			log.Fatalln(err)
		}
	},
}

func init() {
	rootCmd.AddCommand(genCmd)
}
