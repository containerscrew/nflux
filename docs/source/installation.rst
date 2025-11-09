Installation Guide
===================


Arch Linux
----------

.. code-block:: bash

    git clone https://github.com/containerscrew/nflux.git
    cd nflux/packaging
    makepkg -si # or makepkg -si --cleanbuild for a clean build
    # Clean
    rm -rf pkg src *.pkg.tar* && cargo clean
