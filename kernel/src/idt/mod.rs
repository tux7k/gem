use core::mem::size_of;
mod handlers;

//* The IDT struct used when inserting IDT using the `lidt` instruction
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
struct IDTR {
    limit: u16, //? The limit of the IDT
    base: u64,  //? The base address of the IDT
}

//* An IDT entry used in a array of IDT entries
#[repr(packed)]
#[derive(Debug, Clone, Copy)]
struct IDT {
        isr_low: u16,      //? The lower 16 bits of the ISR's address
    kernel_cs: u16,    //? The GDT segment selector that the CPU will load into CS before calling the ISR
    ist: u8,           //? The IST in the TSS that the CPU will load into RSP; set to zero for now
    attributes: u8,    //? Type and attributes; see the IDT page
    isr_mid: u16,      //? The higher 16 bits of the lower 32 bits of the ISR's address
    isr_high: u32,     //? The higher 32 bits of the ISR's address
    reserved: u32,     //? Set to zero
}

//* IDT Entries
static mut idt_entry_t: [IDT; 256] = [IDT {
    isr_low: 0,
    kernel_cs: 0,
    ist: 0,
    attributes: 0,
    isr_mid: 0,
    isr_high: 0,
    reserved: 0,
}; 256];

//* The IDT static pointed to when inserting the IDT using the `lidt` instruction
static mut idtr_t: IDTR = IDTR {
    limit: 0,
    base: 0,
};

//* A function that sets up an IDT entry
fn idt_set_descriptor(vector: u8, isr: unsafe extern "C" fn(), flags: u8) {
    unsafe {
        idt_entry_t[vector as usize] = IDT {
            isr_low: ((isr as u64) & 0xFFFF) as u16,
            kernel_cs: 0x8,
            ist: 0,
            attributes: flags,
            isr_mid: (((isr as u64) >> 16) & 0xFFFF) as u16,
            isr_high: (((isr as u64) >> 32) & 0xFFFFFFFF) as u32,
            reserved: 0,
        }
    }
}

//* A function that sets up the IDT
pub extern "C" fn idt_init () {
    unsafe {
        //? Set the IDT limit
        idtr_t.limit = ((size_of::<IDT>() * 256) - 1) as u16;

        //? Set the IDT base address
        idtr_t.base = idt_entry_t.as_ptr() as u64;

        //? Make all entries use default handlers
        //? so we don't get unhandled exceptions
        for i in 0..=255 {
            idt_set_descriptor(i, handlers::default_handler, 0x8E);
        }

        //? Setup basic IDT entries
        idt_set_descriptor(0xE, handlers::page_fault, 0x8E);
        idt_set_descriptor(0x3, handlers::breakpoint, 0x8E);
        idt_set_descriptor(0x8, handlers::double_fault, 0x8E);

        //? Load the IDT
        llvm_asm!("lidt ($0)" :: "r"(&idtr_t) :: "volatile");
    }
}