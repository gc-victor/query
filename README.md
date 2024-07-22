# Project Switch CLI

The Project Switch CLI is a command-line tool designed to facilitate switching between projects. Built with Rust, it offers a simple yet powerful interface for easily adding, listing, removing, and navigating your projects.

## Install

### Use The Installer Scripts

macOS and Linux (not NixOS, Alpine, or Asahi):

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/gc-victor/project-swithc/releases/latest/download/query-installer.sh | sh
```

Windows PowerShell:

```powershell
irm https://github.com/gc-victor/project-swithc/releases/latest/download/query-installer.ps1 | iex
```

## Features

- **Add Projects**: Easily add your current working directory as a new project. If a project name is not specified, the CLI will use the current directory name.
- **List Projects**: Display a list of all added projects. You can view the raw list, including project paths, or a simplified list showing only project names.
- **Remove Projects**: Remove projects from your list by specifying the project name.
- **Go To Project**: Quickly switch to a project's directory by specifying the project name.

## Getting Started

To start with the Project Switch CLI, install this repository as explained in the [Install](#install) section.

After installing, you can run the CLI directly from the command line. Here are some example commands:

```sh
# Add the current directory as a new project, using the folder name as a project name
project-switch add
```

```sh
# Add the current directory as a new project, using the defined name as a project name
project-switch add my-project
```

```sh
# List all projects
project-switch list
```

```sh
# List all projects with their paths
project-switch list --raw true
```

```sh
# Remove a project by name
project-switch remove my-project
```

```sh
# Go to a project by name
project-switch go my-project
```

We do recommend adding the following alias to your shell configuration file (e.g., `.bashrc,` `.zshrc,` etc.):

```sh
alias pswa='project-switch add'
alias pswl='project-switch list'
alias pswlr='project-switch list --raw true'
alias pswr='project-switch remove'
alias pswg='project-switch go'
```

## Configuration

The Project Switch CLI automatically creates a configuration directory and a database file to store your projects. The config files are created in your system's default configuration directory under `project-switch.` The location of this directory varies depending on your operating system:

- Linux: `~/.config/project-switch`
- MacOS: `~/Library/Application Support/project-switch`
- Windows: `C:\Users\<username>\AppData\Roaming\project-switch`

## Acknowledgements

This project was inspired by https://github.com/Angelmmiguel/pm.

## Contribution

Contributions are welcome! If you have ideas for new features or have found a bug, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
