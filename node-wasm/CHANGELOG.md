# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.1](https://github.com/zvolin/lumina/compare/lumina-node-wasm-v0.3.0...lumina-node-wasm-v0.3.1) - 2024-09-26

### Other

- *(node-wasm)* clarify edge case when polling worker on startup ([#390](https://github.com/zvolin/lumina/pull/390))

## [0.3.0](https://github.com/eigerco/lumina/compare/lumina-node-wasm-v0.2.0...lumina-node-wasm-v0.3.0) - 2024-09-24

### Added

- *(node-wasm)* [**breaking**] Align JS api to use camelCase ([#383](https://github.com/eigerco/lumina/pull/383))
- *(node-wasm)* [**breaking**] Webpack compatibility ([#377](https://github.com/eigerco/lumina/pull/377))
- *(node)* [**breaking**] Implement graceful shutdown ([#343](https://github.com/eigerco/lumina/pull/343))

### Fixed

- *(node-wasm)* wait for worker to start when creating client ([#389](https://github.com/eigerco/lumina/pull/389))

### Other

- *(ci)* Automatic release to npmjs ([#378](https://github.com/eigerco/lumina/pull/378))

## [0.2.0](https://github.com/eigerco/lumina/compare/lumina-node-wasm-v0.1.1...lumina-node-wasm-v0.2.0) - 2024-08-13

### Added
- *(node-wasm)* [**breaking**] Add websocket support ([#341](https://github.com/eigerco/lumina/pull/341))
- feat!(node): make syncer batch sizes configurable ([#327](https://github.com/eigerco/lumina/pull/327))
- add support for dnsaddr resolving in browser ([#319](https://github.com/eigerco/lumina/pull/319))
- Add requesting storage persistence for more quota ([#318](https://github.com/eigerco/lumina/pull/318))
- *(node)* Generate syncing related events ([#312](https://github.com/eigerco/lumina/pull/312))
- *(wasm)* Run Lumina in a Shared Worker ([#265](https://github.com/eigerco/lumina/pull/265))
- *(node)* [**breaking**] Make HeaderSession operate on a single header range again ([#303](https://github.com/eigerco/lumina/pull/303))
- *(node/syncer)* [**breaking**] Implement backwards header sync ([#279](https://github.com/eigerco/lumina/pull/279))
- *(node-wasm)* Add human readable message on node events ([#294](https://github.com/eigerco/lumina/pull/294))
- *(node-wasm)* Implement easier way for handling JS errors. ([#284](https://github.com/eigerco/lumina/pull/284))
- *(node)* Generate events for data sampling that can be used by front-end ([#276](https://github.com/eigerco/lumina/pull/276))
- *(node/daser)* [**breaking**] Implement backward sampling and sampling window ([#269](https://github.com/eigerco/lumina/pull/269))

### Fixed
- *(node-wasm)* Fix high memory consumption ([#356](https://github.com/eigerco/lumina/pull/356))
- *(node)* [**breaking**] Do not skip header-sub reports when store writes are slow ([#333](https://github.com/eigerco/lumina/pull/333))
- *(node)* Patch unreleased libp2p version to include syncing bug fixes ([#290](https://github.com/eigerco/lumina/pull/290))
- *(node-wasm)* require serving and providing worker script ([#313](https://github.com/eigerco/lumina/pull/313))
- new lints coming with 1.78 and 1.80-nightly ([#275](https://github.com/eigerco/lumina/pull/275))

### Other
- *(node)* [**breaking**] Hide internal components ([#342](https://github.com/eigerco/lumina/pull/342))
- remove genesis hash from node configuration ([#316](https://github.com/eigerco/lumina/pull/316))
- [**breaking**] Upgrade dependencies but exclude the ones that are patched by risc0 ([#292](https://github.com/eigerco/lumina/pull/292))

## [0.1.1](https://github.com/eigerco/lumina/compare/lumina-node-wasm-v0.1.0...lumina-node-wasm-v0.1.1) - 2024-04-18

### Added
- Expose get_sampling_metadata in node and node-wasm ([#234](https://github.com/eigerco/lumina/pull/234))
- *(blockstore)* add IndexedDb blockstore ([#221](https://github.com/eigerco/lumina/pull/221))
- feat!(node): use generic blockstore in node ([#218](https://github.com/eigerco/lumina/pull/218))

### Fixed
- *(ci)* Fix release for lumina (cli) ([#190](https://github.com/eigerco/lumina/pull/190))

## [0.1.0](https://github.com/eigerco/lumina/releases/tag/lumina-node-wasm-v0.1.0) - 2024-01-12

### Added
- Return bootstrap peers as iterator and filter relevant ones ([#147](https://github.com/eigerco/lumina/pull/147))
- *(node)* Implement running node in browser ([#112](https://github.com/eigerco/lumina/pull/112))

### Other
- add missing metadata to the toml files ([#170](https://github.com/eigerco/lumina/pull/170))
- document public api ([#161](https://github.com/eigerco/lumina/pull/161))
- rename the node implementation to Lumina ([#156](https://github.com/eigerco/lumina/pull/156))
- switch browser logging to tracing-web ([#150](https://github.com/eigerco/lumina/pull/150))
