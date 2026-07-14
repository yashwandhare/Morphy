#!/usr/bin/env bash
set -e

# --- Configuration ---
REPO="yashwandhare/Morphy"
BIN_NAME="morphy"
INSTALL_DIR="$HOME/.local/bin"

cat << "EOF"
███╗   ███╗  ██████╗  ██████╗  ██████╗  ██╗  ██╗ ██╗   ██╗
████╗ ████║ ██╔═══██╗ ██╔══██╗ ██╔══██╗ ██║  ██║ ╚██╗ ██╔╝
██╔████╔██║ ██║   ██║ ██████╔╝ ██████╔╝ ███████║  ╚████╔╝ 
██║╚██╔╝██║ ██║   ██║ ██╔══██╗ ██╔═══╝  ██╔══██║   ╚██╔╝  
██║ ╚═╝ ██║ ╚██████╔╝ ██║  ██║ ██║      ██║  ██║    ██║   
╚═╝     ╚═╝  ╚═════╝  ╚═╝  ╚═╝ ╚═╝      ╚═╝  ╚═╝    ╚═╝   
==========================================================
EOF

echo " Installing Morphy..."

# --- Detect OS and Architecture ---
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux)
        OS_TAG="linux"
        ;;
    Darwin)
        OS_TAG="macos"
        ;;
    *)
        echo "[ERROR] Unsupported OS: $OS"
        exit 1
        ;;
esac

case "$ARCH" in
    x86_64)
        ARCH_TAG="amd64"
        ;;
    arm64 | aarch64)
        ARCH_TAG="arm64"
        ;;
    *)
        echo "[ERROR] Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

ASSET_NAME="${BIN_NAME}-${OS_TAG}-${ARCH_TAG}.tar.gz"
DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${ASSET_NAME}"

# --- Download and Extract ---
mkdir -p "$INSTALL_DIR"
TMP_DIR=$(mktemp -d)

echo " [1/3] Downloading ${ASSET_NAME}..."
if ! curl -sSLf "$DOWNLOAD_URL" -o "$TMP_DIR/$ASSET_NAME"; then
    echo "[ERROR] Failed to download from $DOWNLOAD_URL"
    echo "Make sure you have created a GitHub Release with the correct asset name."
    rm -rf "$TMP_DIR"
    exit 1
fi

echo " [2/3] Extracting..."
tar -xzf "$TMP_DIR/$ASSET_NAME" -C "$TMP_DIR"
mv "$TMP_DIR/$BIN_NAME" "$INSTALL_DIR/$BIN_NAME"
chmod +x "$INSTALL_DIR/$BIN_NAME"

rm -rf "$TMP_DIR"

echo " [3/3] Finishing setup..."
# --- Add to PATH if needed ---
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo " [WARNING] $INSTALL_DIR is not in your PATH."
    echo " Add this line to your ~/.bashrc or ~/.zshrc:"
    echo " export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

echo ""
echo " [SUCCESS] Morphy successfully installed to $INSTALL_DIR/$BIN_NAME"
echo ""

# --- Post-Installation Dependency Check ---
MISSING_FFMPEG=0
MISSING_WEASYPRINT=0

if ! command -v ffmpeg &> /dev/null; then
    MISSING_FFMPEG=1
fi

if ! command -v weasyprint &> /dev/null; then
    MISSING_WEASYPRINT=1
fi

if [ $MISSING_FFMPEG -eq 1 ] || [ $MISSING_WEASYPRINT -eq 1 ]; then
    echo "----------------------------------------------------------"
    echo " [INFO] Optional Dependencies Missing"
    
    if [ $MISSING_FFMPEG -eq 1 ]; then
        echo " - FFmpeg: Required for Video to GIF conversion."
    fi
    if [ $MISSING_WEASYPRINT -eq 1 ]; then
        echo " - WeasyPrint: Required for Markdown to PDF conversion."
    fi

    echo ""
    echo " Installation Guide for your OS ($OS):"

    if [ "$OS" = "Darwin" ]; then
        # macOS instructions
        echo " Using Homebrew (macOS):"
        if [ $MISSING_FFMPEG -eq 1 ]; then
            echo "   brew install ffmpeg"
        fi
        if [ $MISSING_WEASYPRINT -eq 1 ]; then
            echo "   brew install weasyprint"
        fi
    elif [ "$OS" = "Linux" ]; then
        # Linux instructions (trying to be generic or cover apt/dnf)
        if command -v apt-get &> /dev/null; then
            echo " Using APT (Ubuntu/Debian):"
            if [ $MISSING_FFMPEG -eq 1 ]; then
                echo "   sudo apt update && sudo apt install -y ffmpeg"
            fi
            if [ $MISSING_WEASYPRINT -eq 1 ]; then
                echo "   sudo apt update && sudo apt install -y weasyprint"
                echo "   (Or via pip: pip3 install weasyprint)"
            fi
        elif command -v dnf &> /dev/null; then
            echo " Using DNF (Fedora/RHEL):"
            if [ $MISSING_FFMPEG -eq 1 ]; then
                echo "   sudo dnf install -y ffmpeg"
            fi
            if [ $MISSING_WEASYPRINT -eq 1 ]; then
                echo "   sudo dnf install -y weasyprint"
                echo "   (Or via pip: pip3 install weasyprint)"
            fi
        elif command -v pacman &> /dev/null; then
            echo " Using Pacman (Arch Linux):"
            if [ $MISSING_FFMPEG -eq 1 ]; then
                echo "   sudo pacman -S ffmpeg"
            fi
            if [ $MISSING_WEASYPRINT -eq 1 ]; then
                echo "   sudo pacman -S weasyprint"
            fi
        else
            echo " Please use your system's package manager to install the missing tools."
            if [ $MISSING_WEASYPRINT -eq 1 ]; then
                echo " Weasyprint can also be installed via pip: pip3 install weasyprint"
            fi
        fi
    fi
    echo "----------------------------------------------------------"
fi

echo " You can now run Morphy by typing 'morphy' in your terminal!"
