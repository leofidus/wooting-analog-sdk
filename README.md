[![Build Status](https://travis-ci.com/simon-wh/Analog-SDK.svg?token=cXSit4fro9HcRkW9M9xk&branch=master)](https://travis-ci.com/simon-wh/Analog-SDK)

# Analog SDK

This repo contains all the core components of the Analog SDK.

NOTE: Use the `WOOTING_ANALOG_SDK_PLUGINS_PATH` environment variable to tell the SDK where to search for plugins.

Have a look at the [SDK usage](SDK_USAGE.md) for a guide on how to use the SDK and the [Plugin introduction](PLUGINS.md) for information on creating plugins.

## Components
* `wooting-analog-sdk`: The core Analog SDK which handles loading of plugins. This is installed systemwide and is updated separately
* `wooting-analog-sdk-common`: This library contains all common Analog SDK code, this is used by plugins and the SDK itself
* `wooting-analog-sdk-wrapper`: This is the SDK wrapper which is what Applications should use to communicate with the SDK. The linked dll should be shipped with the application using it.
* `wooting-analog-sdk-test`: This is a C# test application which can be used to test the SDK through the wrapper.

## Building 
### Build Dependencies
* [rust](https://www.rust-lang.org/)
* [cargo-make](https://github.com/sagiegurari/cargo-make)
* [cbindgen](https://github.com/eqrion/cbindgen) (Should be installed automatically if necessary)
* [dotnet-core](https://dotnet.microsoft.com/download) If you want to use `analog-sdk-test`


### How to Build
Everything can be built using this command. All the outputs will be under `target/debug`
```
cargo make
```

The current build process is setup to verify the existing generated headers in the test phase. If you decide to make changes which effect these outputs, you can update the headers by running:
```
cargo make gen-headers
```


To run the test application:
```
cargo make test-app
```

### Outputs
All build outputs can be found under `target/debug`, with generated headers coming under the `includes` and `includes-cpp` directories.

Currently the headers have to be manually generated and kept in the repo. When intentional changes are made, the testing phase verifies that the pre-generated headers match what would be generated now to ensure that accidental changes aren't made to the output of the header generation.

#### Outputted Headers
* `wooting-analog-sdk-wrapper.h`: This is the header which includes everything that you need to use the SDK. (This uses `wooting-analog-sdk-common.h` which defines all relevant enums & structs)
* `wooting-analog-sdk-common.h`: This defines all common enums, headers & structs which are needed by plugins & SDK users
* `wooting-analog-sdk-common-plugin.h`: This includes `wooting-analog-sdk-common.h` & additional functions which are obtained from statically linking to the analog-sdk-common library. (FOR USE WITH PLUGINS)
* `plugin.h`: This is the header which plugins should use to define all functions that need to be exported for a plugin to work

## Related Repositories

* [wooting-analog-sdk-plugin](https://github.com/simon-wh/wooting-analog-sdk-plugin): This is Wooting's Plugin which is written in Rust and serves as a good reference implementation
* [analog-sdk-plugin-examples](https://github.com/simon-wh/analog-sdk-plugin-examples): This repo contains all plugin examples that have been collected
* [wooting-analog-sdk-wrappers](https://github.com/simon-wh/analog-sdk-wrappers): Official language wrappers for the Wooting Analog SDK



## TODO

- [ ] Analog SDK Self-updater
- [ ] Example Application using the SDK
- [ ] Push `wooting-analog-sdk-common` to crates.io
