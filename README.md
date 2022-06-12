# what-to-code-cli
Get a random idea from https://what-to-code.com/
![preview.png](https://i.imgur.com/AoDhkDJ.png)

## Installation
### Releases
Get a release from [github releases](https://github.com/Guaxinim5573/what-to-code-cli/releases).
```
tar xvf what-to-code*.tar.gz
cd what-to-code-cli
sudo ./install
```
### From source
```bash
git clone https://github.com/Guaxinim5573/what-to-code-cli
cd what-to-code-cli
sudo make # Same as `make build && make install`
```
##### Avaible make targets
```bash
make build        # Builds with --release
make build-debug  # Builds without --release
make install      # Installs what-to-code (needs root permissions)
make uninstall    # Uninstall what-to-code (needs root permissions)
```
# License
This program uses [LGPL-2.1](https://github.com/Guaxinim5573/what-to-code-cli/blob/main/LICENSE) license.
