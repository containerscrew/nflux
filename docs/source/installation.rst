Installation
============

Debian/Ubuntu based
-------------------

.. code-block:: bash

    wget https://github.com/containerscrew/nflux/releases/download/1.0.1/nflux_1.0.1-1_amd64.deb -O /tmp/nflux.deb
    sudo dpkg -i /tmp/nflux.deb
    rm /tmp/nflux.deb

Or if using ARM based system:

.. code-block:: bash

    wget https://github.com/containerscrew/nflux/releases/download/1.0.1/nflux_1.0.1-1_arm64.deb -O /tmp/nflux.deb
    sudo dpkg -i /tmp/nflux.deb
    rm /tmp/nflux.deb


Arch Linux
----------

.. code-block:: bash

    wget https://github.com/containerscrew/nflux/releases/download/1.0.1/nflux-1.0.1-1-x86_64.pkg.tar.zst -O /tmp/nflux.pkg.tar.zst
    sudo pacman -U /tmp/nflux.pkg.tar.zst
    rm /tmp/nflux.pkg.tar.zst

.. note::

    Change the version number and architecture as needed. Visit the `releases page <https://github.com/containerscrew/nflux/releases/>`_ to find the latest version and available architectures.

Local Installation from Source
--------------------------------

Pending to be documented.


Now that you have installed nflux, refer to the `usage page <https://nflux.containerscrew.com/usage.html>`_ to get started.