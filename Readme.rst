
bcm_host_sys
================================================================

This library provides a thin Rust interface to `libbcm_host`_ via the Foreign Function Interface(FFI) .


Installation
----------------------------------------------------------------

Add it to your `Cargo.toml` file

.. code-block:: none

    [dependencies]
    bcm_host_sys = "*"

Then, building with the environment variable :envvar:`LIBCLANG_INCLUDE_PATH` .

.. code-block:: shell

    pi@raspberry$ LIBCLANG_INCLUDE_PATH=/path/to/clang/include cargo build

For example:

.. code-block:: shell

    pi@raspberry$ LIBCLANG_INCLUDE_PATH=/usr/include/clang/3.9.1/include cargo build


Requirements
----------------------------------------------------------------

- pkg-config
- libclang
- `libbcm_host`_


Building on the Raspberry Pi, below packages are required:

- pkg-config
- libraspberrypi-dev
- libclang-<ver>-dev (ver=3.9 is confirmed)


Environment Variables
----------------------------------------------------------------

.. envvar:: LIBCLANG_INCLUDE_PATH

   PATH to directory contains C header files.
   e.g. /usr/include/clang/3.9.1/include


Build
----------------------------------------------------------------

Cross Build
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block:: shell

    $ apt-get install libclang-3.9-dev
    $ curl -O http://archive.raspberrypi.org/debian/pool/main/r/raspberrypi-firmware/libraspberrypi-dev_1.20171029-1_armhf.deb
    $ dpkg -x libraspberrypi-dev_1.20171029-1_armhf.deb /
    $ export PKG_CONFIG_PATH=/opt/vc/lib/pkgconfig:$PKG_CONFIG_PATH
    $ export PKG_CONFIG_ALLOW_CROSS=1
    $ export LIBCLANG_INCLUDE_PATH=/usr/include/clang/3.9.1/include
    $ cargo build --target=arm-unknown-linux-gnueabihf


Link
----------------------------------------------------------------

.. _libbcm_host: https://github.com/raspberrypi/firmware/

