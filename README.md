# GOALS

- Define a standard for building CLAP plugins that:
  - targets WASM + WASI architecture
  - can run within a web browser
  - can run within a WASM JIT engine, like wasmer
- Define a standard way to distribute web CLAP plugins

## Why?

A WCLAP (meaning WEB-CLAP) can:
- run on any OS and CPU archicture
  - build once, run everywhere
  - long term compatibility
  - good performance as the code can be optimized for the native CPU and use SIMD
- use web technologies for the GUI
- better user experience for installing and discovering plugins:
  - standard and unified way to distribute and install
  - Official plugin registry, with the possibility for the user to add
    unofficial plugin registries

## How?

There are a set of constraints when targeting WCLAP:
- the plugin may run on an OS that may not exist yet at the time of building the plugin
  - the host has to help the plugin to determine where to store files, like activation,
    user preset, caches, indexes, ...
- we will probably need to be able to sandbox plugins:
  - restrict filesystem, network, hardware, ... access

The plugin will be a bundle.

We'll have an official root plugin registry, hosted by the CLAP project.

We'll provide a standard and opensource adapter for bridging wasm plugin to existing native hosts.

## The bridge

It is unrealistic to expect every hosts to add support to wasm and undesirable because:
- it is very hard to add the required dependencies to the build system
- we want a consistent experience across all hosts
- we want a universal solution

We will build an opensource bridge that the user will have to install and update.

```
---------------       ---------------       ----------------
| Native Host | <---> | WASM Bridge | <---> | WASM Plugins |
---------------       ---------------       ----------------
```

The bridge will:
- provide web-ui support for the wasm plugins
- provide wasi runtime via wasmer (or similar)
- provide an application for:
  - browsing and installing/uninistalling wclap plugins
  - managing the plugin permissions
- provide filesystem helper for the plugin to locate:
  - the plugin bundle (to load the factory content, ...)
  - the user preset directory
  - the cache directory
  - the "var" directory (to store indexes, activation files, preferences, ...)
  - this could be done via a virtual filesystem mapping (within wasmer) or via a getter.

The bridge will likely be implemented in rust, because cargo will make everything easier.

### JSON registry/repository format

The bridge will fetch an initial list of repositories, and the user will be able to add their own registries/repositories to that list, by providing the URL of JSON resources.  The specific JSON format will be developed as part of the open-source bridge plugin.

While this JSON format may be adopted by other WCLAP hosts (for easy browsing/installation of WCLAP plugins), it is not an explicit part of the WCLAP standard, as it's a package-manager/distribution issue.

## GUI

The GUI is optional, so 3 scenarios are planned:
- headless
- web gui
  - we can ship a web browser engine like tauri does, and spawn a window for the plugin with a webview, then connect the plugin to the webview
- native gui
  - once wasi provides wgpu, mouse and keyboard using input and windowing system support

## The Bundle file format

The bundle is a directory that contains a mandatory `module.wasm` file,
and an arbitrary set of files. For example:
```
my-plugin.wclap/
|`- module.wasm
|`- factory-wavetables/...
|`- factory-presets/...
|`- gui/
|    `- images/...
|    `- html/...
|    `- js/...
 `- ...
```

### Fetching from the web

When serving WCLAP bundles over the web, their URL should resolve to an archive using `.tar.gz` file format.  This allows straightforward discovery and pre-loading of all resources.

Hosts running in the web browser might use [Service Workers](https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API) to transparently fetch and unpack this bundle and return its contents using fictitious URLs (e.g. `my-plugin.wclap.tar.gz/module.wasm`).
