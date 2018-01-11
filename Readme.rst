
bcm_host_sys
================================================================

This library provides a thin Rust interface to `libbcm_host`_ via the Foreign Function Interface(FFI) .


Installation
----------------------------------------------------------------

Add it to your `Cargo.toml` file

.. code-block:: none

    [dependencies]
    bcm_host_sys = "*"

Then, building with the environment variable :envvar:`C_INCLUDE_DIR` .

.. code-block:: shell

    pi@raspberry$ C_INCLUDE_DIR=/path/to/c/include cargo build


For example:

.. code-block:: shell

    pi@raspberry$ C_INCLUDE_DIR=/usr/lib/llvm-3.9/lib/clang/3.9.1/include cargo build


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

.. envvar:: C_INCLUDE_DIR

   PATH to directory contains C header files.
   e.g. /usr/lib/llvm-3.9/lib/clang/3.9.1/include

Build
----------------------------------------------------------------

Cross Build
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block:: shell

    $ curl -O
    $ dpkg -x /
    $ export PKG_CONFIG_PATH=/opt/vc/lib/pkgconfig:$PKG_CONFIG_PATH
    $ export PKG_CONFIG_ALLOW_CROSS
    $ export C_INCLUDE_DIR=/x-tools/sysroot/usr/include
    $ cargo build --target=arm-unknown-linux-gnueabihf

Link
----------------------------------------------------------------

.. _libbcm_host: https://github.com/raspberrypi/firmware/

