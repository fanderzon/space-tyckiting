# Space Tyckiting Python client

![logo](logo.png)

*Author: Axel Eirola*

## Synopsis

```sh
# Setup
virtualenv venv
source venv/bin/activate
pip install -r requirements.txt

# Run
python ./cli.py
```

## Prerequisites

Python 2.7 or 3.x, should be preinstalled on most systems. If not, follow instuctions at https://wiki.python.org/moin/BeginnersGuide/Download.

You'll also want [virtualenv](https://pypi.python.org/pypi/virtualenv) in order to isolate project dependencies.

```sh
sudo easy_install install virtualenv
```

## How-to start with new AI?

There is a dummy AIs available in `tyckiting_client/ai/` folder that can be used as a template.
 1. Copy `dummy.py` to `your_ai.py`
 2. Implement behaviour in the `move()` method.
 3. Run your custom AI with `python ./cli --ai your_ai`



## Testing

Add your tests in a new file under `tyckiting_client/ai/tests` and run `nosetests`.
If you want to watch for file changes and rerun tests on save, run `nosetests --with-watch`.


## Editors

Any text editor with syntax highlighting is sufficient. Additionally, you might want some static code analysis, such as [`flake8`](https://pypi.python.org/pypi/flake8), either as a commandline tool or editor plugin ([Sublime](https://github.com/SublimeLinter/SublimeLinter-flake8), [Atom](https://atom.io/packages/linter-flake8), [vim](https://github.com/nvie/vim-flake8), [Emacs](http://flycheck.readthedocs.org/en/latest/guide/languages.html#python))

If you prefer IDE style editors, then [PyCharm](https://www.jetbrains.com/pycharm/) is a solid option.


## Learning materials

*Links are to Python 3, Python 2 equivalents available*

 * [Instant Python](http://hetland.org/writing/instant-python.html) - Minimal crash course
 * [Official tutorial](https://docs.python.org/3/tutorial/index.html) - Really verbose tutorial
 * [Language reference](https://docs.python.org/3/reference/index.html) - Details on language
 * [Standard library reference](https://docs.python.org/3/library/index.html) - Details on included facilities
 * [PyPI](https://pypi.python.org/pypi) - Python package repository, for stuff not in the standard library
 * [DevDocs](http://devdocs.io/python) - Quick, searchable reference documentation
 * [Code Like a Pythonista: Idiomatic Python](http://python.net/~goodger/projects/pycon/2007/idiomatic/handout.html) - Advanced conventions
