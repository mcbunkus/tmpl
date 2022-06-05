package lib

const BlankConfig = `
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

`
