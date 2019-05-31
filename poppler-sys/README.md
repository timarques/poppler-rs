# poppler-sys

Low level bindings to [poppler](https://gitlab.freedesktop.org/poppler/poppler).

## Bindings for poppler-0.77.0

To regenerate the bindings:

``` bash
export POPPLER_TAG="0.77.0"

# get and checkout poppler
echo ">checking requirements.." \
&& clang --version \
&& echo ">getting and updating poppler.." \
&& git submodule update --init \
&& echo ">checking out to poppler-$POPPLER_TAG.." \
&& cd poppler \
&& git checkout poppler-$POPPLER_TAG \
&& cd .. \
&& echo ">finished." \
|| >&2 echo ">some error occurred"

# build, so templated source files generate final source files
# (commands based on `poppler/INSTALL`)
# (takes a while..)
mkdir build \
&& cd build \
&& cmake .. \
&& cd glib \
&& make \
&& cd .. \
&& cd .. \
&& echo ">sucessfully generated templated files."
|| >&2 echo ">some error occurred"
# ps. actually, only few pre-processing commands should be
# required to be executed

TODO remove
cargo install bindgen --vers 0.30.0
bindgen --builtins --no-doc-comments librdkafka/src/rdkafka.h -o src/bindings/{platform}.rs
```

## Version

The rdkafka-sys version number is in the format `X.Y.Z-P`, where `X.Y.Z`
corresponds to the librdkafka version, and `P` indicates the version of the
rust bindings.

## Build

By default a submodule with the librdkafka sources pinned to a specific commit will
be used to compile and statically link the library.

The `dynamic_linking` feature can be used to link rdkafka to a locally installed
version of librdkafka: if the feature is enabled, the build script will use `pkg-config`
to check the version of the library installed in the system, and it will configure the
compiler to use dynamic linking.

The build process is defined in [`build.rs`].

[`build.rs`]: https://github.com/fede1024/rust-rdkafka/blob/master/rdkafka-sys/build.rs

## Updating

To upgrade change the git submodule in `librdkafka`, check if new errors
need to be added to `helpers::primive_to_rd_kafka_resp_err_t` and update
the version in `Cargo.toml`.







