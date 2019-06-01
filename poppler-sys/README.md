# poppler-sys

Low level bindings to [poppler](https://gitlab.freedesktop.org/poppler/poppler).

## Features

glib-api
qt5-api
cpp-api

## Bindings for poppler-0.77.0

To regenerate the bindings:

``` bash
export POPPLER_TAG="0.77.0"

# 1. get and checkout poppler
# (at poppler-sys/)
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

# 2. build process so final source files are generated
# (commands based on poppler/INSTALL)
# (at poppler-sys/)
mkdir poppler/build \
&& cd poppler/build \
&& cmake .. \
&& echo ">sucessfully generated part of final source files."
&& cd ../.. \
|| >&2 echo ">some error occurred"

# depending on the apis that you want, you'll have to
# generate it for glib, qt5 or directly for cpp.

# 2.1 for glib api..
# (at poppler-sys/)
cd poppler/build/glib \
&& make \
&& cd ../../.. \
&& echo ">sucessfully generated final glib source files." \
|| >&2 echo ">some error occurred"
# ps. actually, only few pre-processing commands should be
# required to be executed

# 2.2 for qt5 api..
# (at poppler-sys/)
cd poppler/build/qt5 \
&& make \
&& cd ../../.. \
&& echo ">sucessfully generated final qt5 source files." \
|| >&2 echo ">some error occurred"
# ps. actually, only few pre-processing commands should be
# required to be executed

# 2.3 for cpp api..
# (at poppler-sys/)
cd poppler/build/cpp \
&& make \
&& cd ../../.. \
&& echo ">sucessfully generated final cpp source files." \
|| >&2 echo ">some error occurred"
# ps. actually, only few pre-processing commands should be
# required to be executed
```

