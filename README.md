# ALIASMAN-RS
Estado actual:

[![CI](https://github.com/Andy2403/aliasman-rs/actions/workflows/main.yml/badge.svg)](https://github.com/Andy2403/aliasman-rs/actions/workflows/main.yml)

Licensias:

![Crates.io License](https://img.shields.io/crates/l/ohmyalias)
![GitHub License](https://img.shields.io/github/license/Andy2403/aliasman-rs)

🌟 Gracias a todos

![Crates.io Total Downloads](https://img.shields.io/crates/d/ohmyalias)
![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/ohmyalias)

![Dynamic TOML Badge](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FAndy2403%2Faliasman-rs%2Fmain%2FCargo.toml&query=%24.package.version&style=flat&label=In%20Dev%20Version)
[![Crates.io](https://img.shields.io/crates/v/ohmyalias.svg)](https://crates.io/crates/ohmyalias)

## Estado en GitHub:

![GitHub User's stars](https://img.shields.io/github/stars/Andy2403?style=flat)
![GitHub commit activity](https://img.shields.io/github/commit-activity/t/Andy2403/aliasman-rs)

[![Lines of code](https://tokei.rs/b1/github/Andy2403/aliasman-rs?category=code)](https://github.com/Andy2403/aliasman-rs)
![Crates.io Size](https://img.shields.io/crates/size/ohmyalias)
![GitHub repo size](https://img.shields.io/github/repo-size/Andy2403/aliasman-rs)

![Crates.io Dependents](https://img.shields.io/crates/dependents/ohmyalias)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Andy2403/aliasman-rs/main.yml)

## Que hay de nuevo en la 1.2?
En esta última versión he puesto el foco en:
  - Mejorar la experiencia de usuario
  - Agregar nuevas características

Maneja tus alias de forma fácil y rapida.

## Cómo usarse

El funcionamiento de AliasMan se basa en estos comandos:

1. ➕ Add: para agregar un alias no existente
```shell
aliasman add hello echo "Hello World"
```
2. 🔄 Remove o Rm: para quitar un alias previamente creado
```shell
aliasman remove hello
aliasman rm hello
```
3. 📝 Replace o Edit: para cambiar el comando a ejecutar de un alias
```shell
aliasman replace hello echo "Hello!"
aliasman edit hello echo "Hello!"
```
4. 📑 List: muestra un listado de todos los alias
```shell
aliasman list
```
## Ahora ya no se tendrá que actualizar el paquete para añadir soporte a otras terminales
5. ⭐ Update: descarga el archivo de configuración desde github
```shell
aliasman update
```

## Soporte de terminales

#### De momento aliasman esta testeado en:
1. bash | sh
2. zsh
3. fish

### Estamos trabajando activamente para añadir más terminales

## Contribuir
¿Encontraste un problema o tienes alguna sugerencia?
Siéntete libre de abrir una issue o si necesita
más información lea [CONTRIBUTING.md].

## Cargo Install
Para instalar este paquete, si estas en linux (Ubuntu) necesitarás tener estos paquetes
o los equivalentes en tu distro: pkg-config libssl-dev libudev-dev

De presentarse algún problema, por favor informelo mediante una issue siguiendo siempre el [CONTRIBUTING.md].

[CONTRIBUTING.md]: https://github.com/Andy2403/aliasman-rs/blob/master/CONTRIBUTING.md
