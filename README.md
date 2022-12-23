## iTag python script to detect button pushes
Simple script to read iTag button pushes. Originally was [done](https://gist.github.com/bricewge/a38bd4223e407967f6ab78842c3df07e) with `bluepy`, but one of the future ideas is to add this script to `buildroot` and create a minimal image for Raspberry Pi Zero W. As `bluepy` its not by default part of `buildroot` but `bleak` is, the whole script was migrated to `bleak`.

![](image.jpg)
```
Service UUID: 0000ffe0-0000-1000-8000-00805f9b34fb
Charasteristic UUID: 0000ffe1-0000-1000-8000-00805f9b34fb
```

### Execute & Install
```bash
# python -m venv venv
# source venv/bin/activate
pip install -r requirements.txt
python standalone.py
```

### TODO:
- ~~CTRL-C Exit~~
- Argument parser (?)
- Buildroot image for `rpi0w`

### References
- https://gist.github.com/bricewge/a38bd4223e407967f6ab78842c3df07e