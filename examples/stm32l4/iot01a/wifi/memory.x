MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH                             : ORIGIN = 0x08000000, LENGTH = 1024K
  RAM                         (rwx) : ORIGIN = 0x20000000, LENGTH = 96K
}
