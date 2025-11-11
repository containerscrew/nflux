===============
Local Development
===============

This guide explains how to set up and run the project locally for development purposes.

Prerequisites
=============

- Python 3.8 or higher
- Git
- Virtualenv (optional but recommended)

Setup Instructions
==================

1. **Clone the repository:**

    .. code-block:: bash

        git clone https://github.com/yourusername/nflux.git
        cd nflux

2. **Create and activate a virtual environment:**

    .. code-block:: bash

        python3 -m venv venv
        source venv/bin/activate

3. **Install dependencies:**

    .. code-block:: bash

        pip install -r requirements.txt

Running the Application
=======================

To start the application locally:

.. code-block:: bash

    python main.py

Testing
=======

To run tests:

.. code-block:: bash

    pytest

Troubleshooting
===============

- Ensure all dependencies are installed.
- Check your Python version.
- Refer to the project's README for additional help.

Contribution
============

See :doc:`contributing` for guidelines on contributing to the project.