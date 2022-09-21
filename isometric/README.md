# Isometric

## Running tests

Tests use offscreen rendering using osmesa. For this to work you will need
* `libosmesa` to be installed (e.g. `sudo apt-get install -y libosmesa-dev` on Ubuntu)
* `MESA_GL_VERSION_OVERRIDE` environment variable set to `3.3` or above