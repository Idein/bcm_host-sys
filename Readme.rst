
bcm_host_sys
================================================================

This library provides a thin Rust interface to `libbcm_host`_ via the Foreign Function Interface(FFI) .


Installation
----------------------------------------------------------------

Add it to your `Cargo.toml` file

.. code-block:: none

    [dependencies]
    bcm_host_sys = "*"

Then, building with environment variables listed below:

* :envvar:`VC_INCLUDE_DIR`
* :envvar:`CLANG_INCLUDE_DIR`

.. code-block:: shell

    pi$ VC_INCLUDE_DIR=/path/to/vc/opt/include
    pi$ CLANG_INCLUDE_DIR=/usr/lib/llvm-3.9/lib/clang/3.9.1/include
    pi$ cargo build


Environment Variables
----------------------------------------------------------------

.. envvar:: VC_INCLUDE_DIR

   PATH to directory contains header files of libbcm_host.so.
   If the value is not specified, /opt/vc/include is used.


.. envvar:: CLANG_INCLUDE_DIR

   PATH to directory contains header files of clang.
   e.g. /usr/lib/llvm-3.9/lib/clang/3.9.1/include


Link
----------------------------------------------------------------

.. _libbcm_host: https://github.com/raspberrypi/firmware/

