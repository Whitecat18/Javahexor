#!/bin/bash

if [ -f /etc/os-release ]; then
    source /etc/os-release

    if [[ "$ID" == "arch" ]]; then
        sudo pacman -Syu --noconfirm
        # Installing Sources !
        sudo pacman -Syu
        sudo pacman -S rust-all
        cargo install bore-cli
        curl -O https://install.tunnelmole.com/t357g/install && sudo bash install
        sudo curl https://get.telebit.io/ | bash
        sudo pacman -S gtk3
        cargo build

    elif [[ "$ID" =~ debian|ubuntu|mint|kali ]]; then
        sudo apt update #sudo apt upgrade -y
        sudo apt install rust-all
        cargo install bore-cli
        sudo cp $HOME/.cargo/bin/bore /usr/bin/
        sudo curl https://get.telebit.io/ | bash 
        curl -O https://install.tunnelmole.com/t357g/install && sudo bash install
        sudo apt-get install libgtk-3-dev nodejs npm
	    npm install --save tunnelmole
        cargo build
    else
        echo "Unsupported distribution"
        exit 1
    fi
else
    echo "Unable to determine distribution."
    exit 1
fi
