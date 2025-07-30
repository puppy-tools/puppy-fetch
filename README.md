[![puppy-tools](https://img.shields.io/badge/puppy--tools-%3C/%3E-6495ed.svg?logo=data:image/svg%2bxml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48IS0tISBGb250IEF3ZXNvbWUgRnJlZSA2LjcuMiBieSBAZm9udGF3ZXNvbWUgLSBodHRwczovL2ZvbnRhd2Vzb21lLmNvbSBMaWNlbnNlIC0gaHR0cHM6Ly9mb250YXdlc29tZS5jb20vbGljZW5zZS9mcmVlIChJY29uczogQ0MgQlkgNC4wLCBGb250czogU0lMIE9GTCAxLjEsIENvZGU6IE1JVCBMaWNlbnNlKSBDb3B5cmlnaHQgMjAyNCBGb250aWNvbnMsIEluYy4gLS0+PHBhdGggZD0iTTIyNi41IDkyLjljMTQuMyA0Mi45LS4zIDg2LjItMzIuNiA5Ni44cy03MC4xLTE1LjYtODQuNC01OC41LjMtODYuMiAzMi42LTk2LjggNzAuMSAxNS42IDg0LjQgNTguNXpNMTAwLjQgMTk4LjZjMTguOSAzMi40IDE0LjMgNzAuMS0xMC4yIDg0LjFzLTU5LjctLjktNzguNS0zMy4zUy0yLjcgMTc5LjMgMjEuOCAxNjUuM3M1OS43LjkgNzguNSAzMy4zek02OS4yIDQwMS4yQzEyMS42IDI1OS45IDIxNC43IDIyNCAyNTYgMjI0czEzNC40IDM1LjkgMTg2LjggMTc3LjJjMy42IDkuNyA1LjIgMjAuMSA1LjIgMzAuNWwwIDEuNmMwIDI1LjgtMjAuOSA0Ni43LTQ2LjcgNDYuNy0xMS41IDAtMjIuOS0xLjQtMzQtNC4ybC04OC0yMmMtMTUuMy0zLjgtMzEuMy0zLjgtNDYuNiAwbC04OCAyMmMtMTEuMSAyLjgtMjIuNSA0LjItMzQgNC4yQzg0LjkgNDgwIDY0IDQ1OS4xIDY0IDQzMy4zbDAtMS42YzAtMTAuNCAxLjYtMjAuOCA1LjItMzAuNXpNNDIxLjggMjgyLjdjLTI0LjUtMTQtMjkuMS01MS43LTEwLjItODQuMXM1NC00Ny4zIDc4LjUtMzMuMyAyOS4xIDUxLjcgMTAuMiA4NC4xLTU0IDQ3LjMtNzguNSAzMy4zek0zMTAuMSAxODkuN2MtMzIuMy0xMC42LTQ2LjktNTMuOS0zMi42LTk2LjhzNTIuMS02OS4xIDg0LjQtNTguNSA0Ni45IDUzLjkgMzIuNiA5Ni44LTUyLjEgNjkuMS04NC40IDU4LjV6IiBmaWxsPSIjNjQ5NWVkIi8+PC9zdmc+)](#)
[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](#)
[![Commits](https://img.shields.io/github/commit-activity/t/puppy-tools/puppy-fetch)](#)
[![Workflow](https://img.shields.io/github/actions/workflow/status/puppy-tools/puppy-fetch/rust.yml)](#)
[![Stars](https://img.shields.io/github/stars/puppy-tools/puppy-fetch)](#)

# puppy-fetch :feet: 
A git reference intersection tool in Rust.  
Originally made as a build supplement tool for the **[puppy-tools website](https://puppy-tools.github.io)**.


## Notice
> [!WARNING]
> This repository may be unstable for the time being.

## Installation

> [!NOTE]
> Assumes **[git](https://git-scm.com/downloads)** and **[cargo](https://rustup.rs/)** are installed.

> [!WARNING]
> Ensure `.cargo/bin` is apart of your `PATH` environment variable, or else you will not be able to run this tool.

1. Clone the Repository
```
git clone https://github.com/puppy-tools/puppy-fetch
```
2. Install w/ `cargo`
```
cargo install --path puppy-fetch
```

> [!NOTE]
> Running `puppy-fetch` will print a help message auto-generated from `clap` that should hopefully supply you with the remaining information you need to use this tool.

## Roadmap

- [ ] Search
  - [X] Filter references based on regex
  - [X] Filter blobs based on regex
  - [X] Filter references based on type ( branches, remotes, tags, notes )
  - [X] Include directories
  - [ ] Exclude directories
  - [ ] Dry searching

- [ ] Remote
  - [X] Authorizing with a ssh-agent
  - [X] Authorizing with a ssh-key
  - [X] Authorizing with an auth-token   
  - [ ] API mode
  - [ ] Global credential configuration

- [ ] UX
  - [ ] Directory cleaning on SIGTERM or error
  - [ ] Better documented `clap` arguments
  - [ ] Interactive mode (TUI)

- [ ] Repository
  - [ ] Official binary releases
  - [X] Github workflows
