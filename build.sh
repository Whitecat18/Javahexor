#!/bin/bash

if [ -f /etc/os-release ]; then
    source /etc/os-release

    if [[ "$ID" == "arch" ]]; then
        sudo pacman -Syu --noconfirm
        # Installing Sources !
        sudo pacman -Syu
        cargo install bore-cli
        curl -O https://install.tunnelmole.com/t357g/install && sudo bash install
        sudo curl https://get.telebit.io/ | bash
        sudo pacman -S gtk3
        cargo build

    elif [[ "$ID" =~ debian|ubuntu|mint|kali ]]; then
        sudo apt update #sudo apt upgrade -y
        npm install --save tunnelmole
        cargo install bore-cli
        sudo curl https://get.telebit.io/ | bash
        curl -O https://install.tunnelmole.com/t357g/install && sudo bash install
	    sudo apt-get install libgtk-3-dev nodejs npm
        cargo build
    else
        echo "Unsupported distribution"
        exit 1
    fi
else
    echo "Unable to determine distribution."
    exit 1
fi
