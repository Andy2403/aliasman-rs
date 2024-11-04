# ALIASMAN-RS
Estado actual:

[![CI](https://github.com/Andy2403/aliasman-rs/actions/workflows/main.yml/badge.svg)](https://github.com/Andy2403/aliasman-rs/actions/workflows/main.yml)

Licensias:

![Crates.io License](https://img.shields.io/crates/l/aliasman)
![GitHub License](https://img.shields.io/github/license/Andy2403/aliasman-rs)

🌟 Gracias a todos

![Crates.io Total Downloads](https://img.shields.io/crates/d/aliasman)
![Crates.io Downloads (latest version)](https://img.shields.io/crates/dv/aliasman)

![Dynamic TOML Badge](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FAndy2403%2Faliasman-rs%2Fmain%2FCargo.toml&query=%24.package.version&style=flat&label=In%20Dev%20Version)
[![Crates.io](https://img.shields.io/crates/v/aliasman.svg)](https://crates.io/crates/aliasman)

## Estado en GitHub:

![GitHub User's stars](https://img.shields.io/github/stars/Andy2403?style=flat)
![GitHub watchers](https://img.shields.io/github/watchers/Andy2403/aliasman-rs?style=flat)
[![Lines of code](https://tokei.rs/b1/github/Andy2403/aliasman-rs?category=code)](https://github.com/Andy2403/aliasman-rs)
![Crates.io Size](https://img.shields.io/crates/size/aliasman)
![GitHub repo size](https://img.shields.io/github/repo-size/Andy2403/aliasman-rs)
![GitHub commit activity](https://img.shields.io/github/commit-activity/t/Andy2403/aliasman-rs)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues-pr/Andy2403/aliasman-rs)
![Crates.io Dependents](https://img.shields.io/crates/dependents/aliasman)
![GitHub Created At](https://img.shields.io/github/created-at/Andy2403/aliasman-rs)
![GitHub contributors](https://img.shields.io/github/contributors/Andy2403/aliasman-rs)
![GitHub last commit (branch)](https://img.shields.io/github/last-commit/Andy2403/aliasman-rs/main)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Andy2403/aliasman-rs/main.yml)
[![Docs.rs](https://img.shields.io/docsrs/aliasman)](https://docs.rs/crate/aliasman)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/Andy2403/aliasman-rs/total)
![GitHub Release](https://img.shields.io/github/v/release/Andy2403/aliasman-rs?style=flat)
![GitHub Tag](https://img.shields.io/github/v/tag/Andy2403/aliasman-rs)

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

[CONTRIBUTING.md]: https://github.com/Andy2403/aliasman-rs/blob/master/CONTRIBUTING.md
