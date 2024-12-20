#!/bin/bash

# Check if /etc/os-release exists
if [ -f /etc/os-release ]; then
    source /etc/os-release

    case "$ID" in
        arch)
            echo "Detected Arch Linux. Installing dependencies..."
            sudo pacman -Syu --noconfirm
            sudo pacman -S --noconfirm rust gtk3 base-devel
            cargo install bore-cli
            curl -O https://install.tunnelmole.com/t357g/install && sudo bash install
            sudo curl -s https://get.telebit.io/ | bash
            cargo build
            ;;
        
        debian|ubuntu|mint|kali)
            echo "Detected Debian-based distribution. Installing dependencies..."
            sudo apt update && sudo apt upgrade -y
            sudo apt install -y curl build-essential libgtk-3-dev rustc cargo nodejs npm
            cargo install bore-cli
            sudo cp "$HOME/.cargo/bin/bore" /usr/bin/
            sudo curl -s https://get.telebit.io/ | bash
            curl -O https://install.tunnelmole.com/t357g/install && sudo bash install
            npm install --save tunnelmole
            cargo build
            ;;
        
        *)
            echo "Unsupported distribution: $ID"
            exit 1
            ;;
    esac
else
    echo "Unable to determine distribution: /etc/os-release not found."
    exit 1
fi
