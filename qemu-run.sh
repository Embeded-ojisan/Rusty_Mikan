#!/bin/sh
## kvmへのアクセスができない場合は以下
## sudo chmod 777 /dev/kvm
cp target/x86_64-unknown-uefi/debug/boot.efi esp/efi/boot/bootx64.efi
qemu-system-x86_64 -enable-kvm                                      \
    -drive if=pflash,format=raw,readonly=on,file=bios/OVMF_CODE.fd  \
    -drive if=pflash,format=raw,readonly=on,file=bios/OVMF_VARS.fd  \
    -drive format=raw,file=fat:rw:esp