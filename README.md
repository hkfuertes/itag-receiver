## iTag python script to detect button pushes
Simple script to read iTag button pushes. Originally was [done](https://gist.github.com/bricewge/a38bd4223e407967f6ab78842c3df07e) with `bluepy`, but one of the future ideas is to add this script to `buildroot` and create a minimal image for Raspberry Pi Zero W.

![](image.jpg)

```
Service UUID: 0000ffe0-0000-1000-8000-00805f9b34fb
Charasteristic UUID: 0000ffe1-0000-1000-8000-00805f9b34fb
```

### [TODO] Integration with Home Assistant
The idea is to use the tag service inside Home Assistant:
```python
# ...
requests.post(host + "/api/events/tag_scanned", json = tag_id, headers = {"Authorization": "Bearer " + hassPasswd, "Content-Type": "application/json"})
# ...
```

### Other TODOs:
- Rust implementation:
  - ~POC working~
  - CTRL-C Exit
  - Threads
  - Enigo (simulate key press)
- Buildroot image for `rpi0w`

### References
- https://github.com/ukBaz/python-bluezero
- https://gist.github.com/bricewge/a38bd4223e407967f6ab78842c3df07e