
# Tacit-Py

```
tacit
(ta-sit) adjective
understood without being expressed directly
```

`tacit-py` aims to be a one-stop execution environment for python scripts.
Specific goals are:

 - Zero dependencies beyond the `tacit-py` binary and OS bindings
 - Cross-platform support for 64-bit windows and 64-bit linux on x86 processors
 - Capability to automatically resolve `import <x>` packages using `pip` into a `script-site-packages` folder at either:
    - `os.path.join(os.dirname(__file__), 'script-site-packages')` if running a `.py` script
    - `$CWD/repl-site-packages` if run as an interactive REPL prompt
    - FOR PACKAGES WHICH DO NOT SHARE PACKAGE AND MODULE NAMES:
        - Add a comment like `#TACIT-PY: resolve_module('PIL', 'Pillow')` before a line like `from PIL import Image`
 - Stand-alone windows + linux graphical terminal, with specific enhancements:
    - For windows, avoid hanging when writing process stdout/stderr. This is done by not using 26-year-old terminal graphics and buffering logic.
    - Display common images and graphs in the graphical terminal when `print()`-ed or typed in the REPL
        - replace logic of `matplotlib.pyplot.show()` with rendering to buffer under repl
        - `PIL.Image` will render to display if `print()`-ed or resolved in REPL
 - Semi-interactive mode; when requested, will drop to an interactive REPL after running a python script.
    - To enable, one of the first `40` lines in your `.py` script must contain `#TACIT-PY: semi_interactive=True`

The high-level goals are to enable execution of python scripts on machines which may not have a
python runtime or packages installed in an environment to run the script.

# Building

You will need a copy of `python` (any version >3.9 or so) and `cargo` available.

```bash
python build.py
```


# Distro-specific woes

Arch Linux does not ship the older version of `libcrypt.so.1` which `pyoxidizer` assumes exists for it's copy of `python3.10`.

```bash
sudo pacman -S libxcrypt-compat
```



