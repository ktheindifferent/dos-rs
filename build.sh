#!/bin/bash
RUSTFLAGS="-C opt-level=z" cargo build --release --target dos.json

sudo partx -av freedos.img
read -p "What is the loop number? " loop_number 
sudo mount "/dev/loop${loop_number}p1" /mnt
sudo cp ./target/dos/release/dos.com /mnt/
sudo umount /mnt
sudo partx -dv "/dev/loop${loop_number}"
qemu-system-i386 freedos.img -boot c