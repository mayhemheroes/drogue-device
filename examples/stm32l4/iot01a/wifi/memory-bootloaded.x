MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  BOOTLOADER                        : ORIGIN = 0x08000000, LENGTH = 24K
  BOOTLOADER_STATE                  : ORIGIN = 0x08006000, LENGTH = 4K
  FLASH                             : ORIGIN = 0x08008000, LENGTH = 491520
  DFU                               : ORIGIN = 0x08080000, LENGTH = 493568
  RAM                         (rwx) : ORIGIN = 0x20000008, LENGTH = 98296
}

__bootloader_state_start = ORIGIN(BOOTLOADER_STATE) - ORIGIN(BOOTLOADER);
__bootloader_state_end = ORIGIN(BOOTLOADER_STATE) + LENGTH(BOOTLOADER_STATE) - ORIGIN(BOOTLOADER);

__bootloader_dfu_start = ORIGIN(DFU) - ORIGIN(BOOTLOADER);
__bootloader_dfu_end = ORIGIN(DFU) + LENGTH(DFU) - ORIGIN(BOOTLOADER);
