Usage
=====

Once you installed nflux using your preferred linux distribution, the systemd service will fail by default because you need to edit the ``nflux.toml`` configuration file.

.. code-block:: bash

    sudo nvim /etc/nflux/nflux.toml

Here the most important is edit the ``interface`` parameter and the procotols you want to monitor.

After saving the configuration file, you can now start your ``systemd service``:

.. code-block:: bash

    sudo systemctl start nflux

Let's check the log file:

.. code-block:: bash

    sudo tail -f /var/log/nflux/nflux.log

Congratulations! 🎉 You have successfully started the nflux service and can now monitor your network traffic.

You will track all the incoming connections on the specified network interface.

Now the most interesting part is to visualize the data, so refer to the `monitoring page <https://nflux.containerscrew.com/monitoring.html>`_.