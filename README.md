# Simple HTTP Server

Simple HTTP sever to serve a directory. This server can be used as fast replacement of command
`python -m SimpleHTTPServer`. Root server directory can be set using `--path` argument.

## Usage

Start HTTP server in current directory listening on localhost:8080:

```bash
./simple-http-server
```

Optional arguments:

* `-a` (`--address`) ADDR: Address to listen on, default value - localhost;
* `-p` (`--port`) PORT: Port to listen on, default value - 8080;
* `-d` (`--path`) PATH: Path to server content directory, default value - current directory;
* `-h` (`--help`): Show help and exit.
