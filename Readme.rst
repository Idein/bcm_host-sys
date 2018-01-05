
bcm_host_sys
================================================================

This library provides a thin Rust interface to `libbcm_host`_ via the Foreign Function Interface(FFI) .


Installation
----------------------------------------------------------------

Add it to your `Cargo.toml` file

.. code-block:: none

    [dependencies]
    bcm_host_sys = "*"

Then, building with the environment variable :envvar:`CLANG_INCLUDE_DIR` .

.. code-block:: shell

    pi@raspberry$ CLANG_INCLUDE_DIR=/path/to/clang/include cargo build


For example:

.. code-block:: shell

    pi@raspberry$ CLANG_INCLUDE_DIR=/usr/lib/llvm-3.9/lib/clang/3.9.1/include cargo build


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

.. envvar:: CLANG_INCLUDE_DIR

   PATH to directory contains header files of clang.
   e.g. /usr/lib/llvm-3.9/lib/clang/3.9.1/include


Link
----------------------------------------------------------------

.. _libbcm_host: https://github.com/raspberrypi/firmware/

