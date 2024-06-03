#!/usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PARENT_DIR="$(dirname "$SCRIPT_DIR")"

echo "#[BEGIN SKAR_CONFIG]" >> "$ZDOTDIR/.zshrc"
echo "#[BEGIN SKAR_CONFIG]" >> ~/.bashrc

echo "alias '?'='$PARENT_DIR/target/release/skar shell complete'" >> "$ZDOTDIR/.zshrc"
echo "alias '?'='$PARENT_DIR/target/release/skar shell complete'" >> ~/.bashrc

echo "alias '??'='$PARENT_DIR/target/release/skar shell explain'" >> "$ZDOTDIR/.zshrc"
echo "alias '??'='$PARENT_DIR/target/release/skar shell explain'" >> ~/.bashrc

echo "alias '?!'='$PARENT_DIR/target/release/skar shell generate'" >> "$ZDOTDIR/.zshrc"
echo "alias '?!'='$PARENT_DIR/target/release/skar shell generate'" >> ~/.bashrc

echo "#[END SKAR_CONFIG]" >> "$ZDOTDIR/.zshrc"
echo "#[END SKAR_CONFIG]" >> ~/.bashrc
