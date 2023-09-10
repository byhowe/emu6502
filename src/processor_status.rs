use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy)]
    pub struct ProcessorStatus: u8
    {
        /// # C: Carry
        ///
        /// * After ADC, this is the carry result of the addition.
        /// * After SBC or CMP, this flag will be set if no borrow was the result,
        ///   or alternatively a "greater than or equal" result.
        /// * After a shift instruction (ASL, LSR, ROL, ROR), this contains the bit
        ///   that was shifted out.
        /// * Increment and decrement instructions do not affect the carry flag.
        /// * Can be set or cleared directly with SEC, CLC.
        const Carry = 1 << 0;

        /// # Z: Zero
        ///
        /// * After most instructions that have a value result, this flag will
        ///   either be set or cleared based on whether or not that value is equal
        ///   to zero.
        const Zero = 1 << 1;

        /// # I: Interrupt Disable
        ///
        /// * When set, all interrupts except the NMI are inhibited.
        /// * Can be set or cleared directly with SEI, CLI.
        /// * Automatically set by the CPU when an IRQ is triggered, and restored to
        ///   its previous state by RTI.
        /// * If the /IRQ line is low (IRQ pending) when this flag is cleared, an
        ///   interrupt will immediately be triggered.
        const InterruptDisable = 1 << 2;

        /// # D: Decimal
        ///
        /// * On the NES, this flag has no effect.
        /// * On the original 6502, this flag causes some arithmetic instructions to
        ///   use binary-coded decimal representation to make base 10 calculations
        ///   easier.
        /// * Can be set or cleared directly with SED, CLD.
        const Decimal = 1 << 3;

        /// # V: Overflow
        ///
        /// * ADC and SBC will set this flag if the signed result would be
        ///   invalid[1], necessary for making signed comparisons[2].
        /// * BIT will load bit 6 of the addressed value directly into the V flag.
        /// * Can be cleared directly with CLV. There is no corresponding set
        ///   instruction.
        const Overflow = 1 << 6;

        /// # N: Negative
        ///
        /// * After most instructions that have a value result, this flag will
        ///   contain bit 7 of that result.
        /// * BIT will load bit 7 of the addressed value directly into the N flag.
        const Negative = 1 << 7;

        // Only 6 flags are used.
        const _ = 0b11001111;
    }
}
