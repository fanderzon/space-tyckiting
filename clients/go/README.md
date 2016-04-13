# Space Tyckiting Go client

![logo](logo.png)

*Author: Henri Koski*

## Synopsis

```sh
# Fetch depencies
cd clients/go/client
go get

cd clients/go/dummy
go get

# Run
go run dummy/dummy.go

# Help
go run dummy/dummy.go -help
```

## Prerequisites

Download and install [Go](https://golang.org/doc/install).

## How-to start with new AI?

There is a dummy AI available in `ai/` folder that can be used as a template.
 1. Copy `dummy` folder to `your_bot` folder
 2. Rename `your_bot/dummy.go` to `your_bot/your_bot.go`
 3. Implement behaviour in the `move()` function.
 4. Run your custom AI with `go run your_bot/your_bot.go`


## Testing

Create test file and run `go test`

## Editors

Vim plugins:
* [vim-go](https://github.com/fatih/vim-go)

Sublime plugins:
* [GoSublime](https://github.com/DisposaBoy/GoSublime)
* [sublime-build](https://github.com/golang/sublime-build)

Visual Studio plugins:
* [vscode-go](https://github.com/Microsoft/vscode-go)

## Learning materials

Golang tutorials:
* [Tour of go](https://tour.golang.org)
* [Go by Example](https://gobyexample.com/)
* [Learn Go](https://github.com/golang/go/wiki/Learn)
