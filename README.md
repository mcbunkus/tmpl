# tmpl

This is an overengineered tool for myself to generate projects.

Here's the blurb from the help message:

```
Generate a project from a given yaml template. For example:

        tmpl template.yaml

will produce a new project in the current directory. Templates
consist of nested file and directory declarations. An example 
template can be generated using the gen command. Run 

        tmpl gen example.yaml

to produce an example yaml template file.

Usage:
  tmpl [flags]
  tmpl [command]

Available Commands:
  completion  generate the autocompletion script for the specified shell
  gen         Generate a blank template
  help        Help about any command

Flags:
  -h, --help   help for tmpl

Use "tmpl [command] --help" for more information about a command.
```

This is an example template file generated with the `gen` command:

``` yaml
name: <your-name-here>

# You can specify template variables which can written to file contents.
# It uses Go style template syntax.
variables:
  project: test-project

# An example cmake project
template:
  files:
    - name: CMakeLists.txt
      content: |
        cmake_minimum_required(VERSION 3.1...3.21)

        # Fill in the project variable defined in variables
        project({{ .project }} VERSION 1.0 LANGUAGES CXX)

        # Find packages go here.

        # You should usually split this into folders, but this is a simple example

        # This is a "default" library, and will match the *** variable setting.
        # Other common choices are STATIC, SHARED, and MODULE
        # Including header files here helps IDEs but is not required.
        # Output libname matches target name, with the usual extensions on your system
        add_library(MyLibExample simple_lib.cpp simple_lib.hpp)

        # Link each target with other targets or add options, etc.

        # Adding something we can run - Output name matches target name
        add_executable(MyExample simple_example.cpp)

        # Make sure you link your targets with this command. It can also link libraries and
        # even flags, so linking a target that does not exist will not give a configure-time error.
        target_link_libraries(MyExample PRIVATE MyLibExample)

  # Here's a directory containing some C code.
  dirs:
    - name: src
      files:
        - name: main.c
          content: |
            #include <stdio.h>

            int main() {
                printf("Hello, world!\n");
                return 0;
            }
```
