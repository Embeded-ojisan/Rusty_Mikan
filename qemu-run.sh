#!/bin/sh

cp ./target/x86_64-unknown-uefi/debug/boot.efi mnt/EFI/BOOT/BOOTx64.EFI
qemu-system-x86_64 --bios bios/OVMF.fd -drive format=raw,file=fat:rw:mnt