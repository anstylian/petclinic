# Axum Petclinic 

This project is a port of Spring Framework's Petclinic (https://spring-petclinic.github.io/)  to Rust and Axum, which is a simple CRUD webapp where you can keep a list of Pets and Veterinarians.

[screenshot-home]: screenshot.png

![screenshot-home]

## Features 

This project is aimed at showcasing how you could do:

* Cookie based server-side sessions
* Using Redis as a session storage mechanism
* Form based Authentication
* Integration with Tera templates for rendering HTML
* Separate DEV/QA/PROD configurations
* Live reloading of Tera templates in Dev 
* Integration of session data with Tera templates
* Database access using Diesel
* Use nix flakes

## Build

This section should be updated when `flake.nix` is finalized. 

For now only `nix build` is available for building the project.

## Develop

1. `nix develop` you can use it to enter an environment where you can develop for this project.
2. Start redis, `systemctl start redis`.
3. `diesel migration run` to generate the SQLite DB and the `schema.rs`. 
4. You are ready to go.
