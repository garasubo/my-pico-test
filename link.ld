MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* To suit Raspberry Pi RP2040 SoC */
  BOOT_LOADER : ORIGIN = 0x10000000, LENGTH = 0x100
  FLASH : ORIGIN = 0x10000100, LENGTH = 2048K - 0x100
  RAM : ORIGIN = 0x20000000, LENGTH = 256K
}

EXTERN(Reset);
ENTRY(Reset);

SECTIONS {

  /* ### Boot loader */
  .boot_loader ORIGIN(BOOT_LOADER) :
  {
    KEEP(*(.boot_loader*));
  } > BOOT_LOADER

  .vector_table ORIGIN(FLASH):
  {
    /* First entry: initial Stack Pointer value */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* Second entry: reset vector */
    KEEP(*(.vector_table.reset_vector));

    KEEP(*(.vector_table.exceptions));
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH

  .rodata : ALIGN(4) {
    . = ALIGN(4);
    *(.rodata .rodata.*);
    . = ALIGN(4);
  } > FLASH

  .bss (NOLOAD) : ALIGN(4) {
    . = ALIGN(4);
    _sbss = .;
    *(.bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } > RAM

  .data : ALIGN(4)
  {
    . = ALIGN(4);
    _sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > RAM AT> FLASH

  _sidata = LOADADDR(.data);

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }
}

PROVIDE(NMI = DefaultExceptionHandler);
PROVIDE(HardFault = DefaultExceptionHandler);
PROVIDE(MemManage = DefaultExceptionHandler);
PROVIDE(BusFault = DefaultExceptionHandler);
PROVIDE(UsageFault = DefaultExceptionHandler);
PROVIDE(SVCall = DefaultExceptionHandler);
PROVIDE(PendSV = DefaultExceptionHandler);
PROVIDE(SysTick = DefaultExceptionHandler);
