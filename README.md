# QTe st tools for embedded systems emulation
Comando que me funciona desde el directorio TFG:  ./qemu_new/build/qemu-system-arm     -cpu cortex-m4     -machine netduinoplus2     -nographic     -semihosting-config enable=on,target=native     -monitor stdio     -serial null     -qtest tcp:localhost:3000     -kernel ./qtest/qtest-stm32f4nucleo/src/bin/test_v1.elf     > ./qtest/qtest-stm32f4nucleo/src/bin/output_qemu.txt 2>&1 (Esto es para iniciar qemu y redirija la salida al fichero output_qemu.txt ). Antes tiene que estar Rust y la interfaz gráfica funcionando, qemu es el último paso.----------- INTERFAZ------ RUST-------QEMU  


