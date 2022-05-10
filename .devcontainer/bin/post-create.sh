#!/usr/bin/env bash

cargo check
sudo chown -R vscode:vscode /usr/local/cargo/

exit 0