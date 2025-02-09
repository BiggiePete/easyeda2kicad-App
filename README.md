# EZ_LCSC2KiCAD

## Building easyEDA2KiCAD

```
cd into the dir
python -m pip install -r requirements.txt
python ./setup.py build
python -m pip install pyinstaller
pyinstaller --onefile ./build/lib/easyeda2kicad/__main__.py
```
