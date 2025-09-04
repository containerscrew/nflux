#!/usr/bin/env sh

set -e

# Global vars
INSTALLATION_PATH="/usr/local/bin/"
BINARY_NAME="nflux"

# Welcome message
echo "Welcome to the $BINARY_NAME installation script! 🚀."
echo "Author: github.com/containerscrew"

happyexit(){
  echo ""
  echo "$BINARY_NAME successfully installed! 🎉"
  echo ""
  echo "Now run: $ $BINARY_NAME --help"
  echo ""
  echo "Or directly: $ sudo $BINARY_NAME"
  echo ""
  exit 0
}

# Check OS
OS=$(uname -s)
arch=$(uname -m)
cli_arch=""
case $OS in
  Linux)
    case $arch in
      x86_64)
        cli_arch=amd64
        ;;
      armv8*)
        cli_arch=arm64
        ;;
      aarch64*)
        cli_arch=arm64
        ;;
      amd64|arm64)
        cli_arch=$arch
        ;;
      *)
        echo "There is no $BINARY_NAME $OS support for $arch"
        exit 1
        ;;
    esac
    ;;
  *)
    echo "There is no $BINARY_NAME $OS support for $arch"
    exit 1
    ;;
esac
OS=$(echo "$OS" | tr '[:upper:]' '[:lower:]')

download_release() {
  LATEST_VERSION=$(curl -s https://api.github.com/repos/containerscrew/$BINARY_NAME/releases/latest | jq -r ".name")
  if [ -z "$1" ]; then VERSION=$LATEST_VERSION; else VERSION=$1; fi

  printf "\033[0;32m[info] - Downloading version: ${VERSION}/$BINARY_NAME-${OS}-${cli_arch}-${VERSION}.zip \033[0m\n"
  curl -L --fail --remote-name-all https://github.com/containerscrew/$BINARY_NAME/releases/download/"${VERSION}"/$BINARY_NAME-${OS}-${cli_arch}-${VERSION}.zip -o /tmp/$BINARY_NAME.zip
  rm -rf /tmp/$BINARY_NAME
  unzip -o /tmp/$BINARY_NAME.zip -d /tmp/
}

install_binary(){
  if [ "$(id -u)" = 0 ]; then
      cp /tmp/$BINARY_NAME $INSTALLATION_PATH
      chmod +x $INSTALLATION_PATH/$BINARY_NAME
  else
      sudo cp /tmp/$BINARY_NAME $INSTALLATION_PATH
      sudo chmod +x $INSTALLATION_PATH/$BINARY_NAME
  fi
  rm -rf /tmp/$BINARY_NAME*
  happyexit
}

# Function to display help text
usage() {
    echo "Usage: $0 [-v] [-h]"
    echo "Options:"
    echo "  -v           Select which version do you want to install."
    echo "  -h           Display the help message"
}

# Parse options using getopts
while getopts "v:h" option; do
    case "${option}" in
        v)  # Install specific version
            version=${OPTARG}
            download_release "$version"
            install_binary
            ;;
        h)  # Help option
            usage
            exit 0
            ;;
        \?) # Invalid option
            echo "Invalid option: -${OPTARG}"
            usage
            exit 1
            ;;
    esac
done

# If no flags, by default install latest version
if [ $# -eq 0 ]; then
    download_release
    install_binary
fi
