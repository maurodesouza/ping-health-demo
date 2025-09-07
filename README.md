<div align="center" id="top">
  <img src="https://skillicons.dev/icons?i=nodejs" height="50" alt="nodejs logo"  />
  <img width="15" />
  <img src="https://skillicons.dev/icons?i=ts" height="50" alt="typescript logo"  />
  <img width="15" />
  <img src="https://skillicons.dev/icons?i=go" height="50" alt="go logo"  />
  <img width="15" />
  <img src="https://skillicons.dev/icons?i=rust" height="50" alt="rust logo"  />
  <img width="15" />
  <img src="https://coolify.io/docs/coolify-logo-transparent.png" height="50" alt="coolify Logo" />
  <img width="15" />
  <img src="https://skillicons.dev/icons?i=docker" height="50" alt="docker logo"  />
</div>

<h1 align="center">Ping Health Demo</h1>

<p align="center">
  <img alt="Github top language" src="https://img.shields.io/github/languages/top/maurodesouza/ping-health-demo?color=56BEB8">

  <img alt="Github language count" src="https://img.shields.io/github/languages/count/maurodesouza/ping-health-demo?color=56BEB8">

  <img alt="Repository size" src="https://img.shields.io/github/repo-size/maurodesouza/ping-health-demo?color=56BEB8">

  <img alt="License" src="https://img.shields.io/github/license/maurodesouza/ping-health-demo?color=56BEB8">

</p>

<p align="center">
  <a href="#dart-about">About</a> &#xa0; | &#xa0;
  <a href="#sparkles-features">Features</a> &#xa0; | &#xa0;
  <a href="#white_check_mark-requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#checkered_flag-starting">Starting</a> &#xa0; | &#xa0;
  <a href="#memo-license">License</a> &#xa0; | &#xa0;
  <a href="https://github.com/maurodesouza" target="_blank">Author</a>
</p>

<br>

## :dart: About ##

Project created to practice deploying a multi-language project using Docker + Coolify

The goal was to set up a development environment with 3 different apps (Node + Typescript, Go and Rust) and a database (Postgres), and deploy them into a VPS using Docker + Coolify

## :sparkles: Features ##

Each service has two basic endpoints:

- **/ping**: Creates a ping record with the service where it was called.
  (Example: if you call `node_application/ping`, it will create a record with service: `node`. If you call with `rust`, service: `rust`).
  If the record already exists, it will increase the amount and then return it.

- **/health**: Returns **200** if everything is fine, or **503** if not.

## :white_check_mark: Requirements ##

Before starting :checkered_flag:, you need to have [Git](https://git-scm.com) and 🐳 [Docker](https://www.docker.com/) installed.
To test the API, I used the [Rest Client](https://github.com/Huachao/vscode-restclient) extension for [Visual Studio Code](https://code.visualstudio.com), so you can call the endpoints using the `root/request.http` file!

## :checkered_flag: Starting ##

```bash
# Clone this project
$ git clone https://github.com/maurodesouza/ping-health-demo

# Access
$ cd ping-health-demo

# Create .env.development based on .env.example
$ cat .env.example > .env.development

# Run the project
$ docker-compose -f compose.yml --env-file .env.development up

# The servers will initialize at:

node: http://localhost:7070
go: http://localhost:8080
rust: http://localhost:9090

prisma-studio: http://localhost:5555
postgres: http://localhost:5432

```

## :memo: License ##

This project is under license from MIT. For more details, see the [LICENSE](LICENSE.md) file.


Made with :heart: by <a href="https://github.com/maurodesouza" target="_blank">Mauro de Souza</a>

&#xa0;

<a href="#top">Back to top</a>
