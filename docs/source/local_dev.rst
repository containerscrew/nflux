===============
Local Development
===============

This guide explains how to set up and run the project locally for development purposes.

Prerequisites
=============

- Cargo installed

...THIS DOCUMENTATION IS STILL UNDER CONSTRUCTION...

Setup Instructions
==================

1. **Clone the repository:**

    .. code-block:: bash

        git clone https://github.com/containerscrew/nflux.git
        cd nflux

> Missing some more steps here...

2. **Create a new tag:**

I personally use [semantic versioning](https://semver.org/) for tags and [cocogitto](https://docs.cocogitto.io/). To create a new tag, use the following commands:

    .. code-block:: bash

        cog commit fix -a "whatever fix I did"
        cog bump --version 1.0.1
        git push origin main --tags
        cog changelog > CHANGELOG.md
        cog commit chore -a "update changelog for 1.0.1"