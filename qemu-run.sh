#!/bin/sh
## kvmへのアクセスができない場合は以下
## sudo chmod 777 /dev/kvm
cd ./boot && ./xbuild.sh && cd ../ || exit
cd ./kernel && cargo build && cd ../ || exit

mkdir -p esp/bios/
mkdir -p esp/efi/boot/

cp /usr/share/OVMF/OVMF_CODE.fd esp/bios/
cp /usr/share/OVMF/OVMF_VARS.fd esp/bios/

cp target/x86_64-unknown-uefi/debug/boot.efi esp/efi/boot/bootx64.efi

cp target/x86_64-unknown-none-rusty_mikan/debug/kernel.elf esp/kernel.elf

sudo chmod 777 /dev/kvm

qemu-system-x86_64 -enable-kvm      \
    -drive if=pflash,format=raw,readonly=on,file=esp/bios/OVMF_CODE.fd  \
    -drive if=pflash,format=raw,readonly=on,file=esp/bios/OVMF_VARS.fd  \
    -drive format=raw,file=fat:rw:esp

#    -drive if=pflash,format=raw,readonly=on,file=bios/OVMF_CODE.fd  \
#    -drive if=pflash,format=raw,readonly=on,file=bios/OVMF_VARS.fd  \

# シェル上でコマンドを打ち込みたい場合
# qemu-system-x86_64 -enable-kvm  -monitor stdio \

# 画面のモードを変えたい場合
# https://ja.stackoverflow.com/questions/46831/qemu%E3%82%92%E8%B5%B7%E5%8B%95%E3%81%97%E3%81%A6%E3%82%82%E4%BD%95%E3%82%82%E8%A1%A8%E7%A4%BA%E3%81%95%E3%82%8C%E3%81%AA%E3%81%84
# qemu-system-x86_64 -enable-kvm  -display sdl \