# Configuration file for the Sphinx documentation builder.

project = '__ZAPPY_PROJECT_NAME__'
copyright = '__ZAPPY_YEAR__, __ZAPPY_USER__'
author = '__ZAPPY_USER__'

extensions = ["breathe"]
html_extra_path = ["_build/xml"]

breathe_projects = {
    "adapter": "_build/xml"
}
breathe_default_project = "__ZAPPY_PROJECT_NAME_SNAKE__"

html_theme = "sphinx_rtd_theme"

import subprocess
subprocess.call('doxygen Doxyfile', shell=True)
