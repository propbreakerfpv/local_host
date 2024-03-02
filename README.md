#local host
a simple way to serve files over http

## Usage

- `-p` or `--port` specify the port to use. default: 1337
- `-r` or `--rout` specify a rout to serve.
- `-f` or `--file` specify a file to serve. every -r hase to have a -f. -r is the url and -f is the path to the file the url will serve. you can have theys arguments in any order you want. ie: -r -r -f -f or -r -f -r -f. but the order
  does mater in the sence that the first -f corisponds to the first -r.
