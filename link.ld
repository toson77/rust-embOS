/* STM32F401 */
/* 1K = 1024 bytes */
/* https://www.st.com/resource/ja/reference_manual/rm0368-stm32f401xbc-and-stm32f401xde-advanced-armbased-32bit-mcus-stmicroelectronics.pdf 3.3 flash 2.4 sram*/

MEMORY
{
	FLASH : ORIGIN = 0x08000000, LENGTH = 512K
	RAM : ORIGIN = 0x20000000, LENGTH = 96K
}

/* entry point is the reset handler */
ENTRY(Reset);
EXTERN(RESET_VECTOR);

SECTIONS
{
	.vector_table ORIGIN(FLASH) :
	{
		/* First entry: initial Stack Pointer value */
		LONG(ORIGIN(RAM) + LENGTH(RAM));

		/* Second entry: reset vector */
		KEEP(*(.vector_table.reset_vector));
	} > FLASH

	.text :
	{
		*(.text .text.*);
	} > FLASH

	.rodata :
	{
		*(.rodata .rodata.*);
	} > FLASH

	.bss (NOLOAD):
	{
		_sbss = .;
		*(.bss .bss.*);
		_ebss = .;
	} > RAM

	.data : AT(ADDR(.rodata) + SIZEOF(.rodata))
	{
		_sdata = .;
		*(.data .data.*);
		_edata = .;
	} > RAM

	_sidata = LOADADDR(.data);

	/DISCARD/ :
	{
		*(.ARM.exidx .ARM.exidx.*);
	}
}
