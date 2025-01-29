#[doc = "Register `DR` reader"]
pub type R = crate::R<DrSpec>;
#[doc = "Register `DR` writer"]
pub type W = crate::W<DrSpec>;
#[doc = "Field `DATA` reader - Data transmitted or received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Data transmitted or received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FE` reader - UART framing error 1: The received character does not have a valid stop bit (a valid stop bit is 1). 0: No framing error has occurred."]
pub type FeR = crate::BitReader;
#[doc = "Field `PE` reader - UART parity error 1: The parity of the received data character does not match the parity defined by bits 2 and 7 of the UARTLCRH register 0: No parity error has occurred. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type PeR = crate::BitReader;
#[doc = "Field `BE` reader - UART break error 1: A break condition has been detected, indicating that the receive data input was held low for longer than a full-word transmission time (defined as start, data, parity, and stop bits). 0: No break condition has occurred. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only the one 0 character is loaded into the FIFO. The next character is only enabled after the received data input goes to a 1 (marking state), and the next valid start bit is received."]
pub type BeR = crate::BitReader;
#[doc = "Field `OE` reader - UART overrun error 1: New data was received when the FIFO was full, resulting in data loss. 0: No data has been lost due to a FIFO overrun."]
pub type OeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Data transmitted or received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - UART framing error 1: The received character does not have a valid stop bit (a valid stop bit is 1). 0: No framing error has occurred."]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART parity error 1: The parity of the received data character does not match the parity defined by bits 2 and 7 of the UARTLCRH register 0: No parity error has occurred. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART break error 1: A break condition has been detected, indicating that the receive data input was held low for longer than a full-word transmission time (defined as start, data, parity, and stop bits). 0: No break condition has occurred. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only the one 0 character is loaded into the FIFO. The next character is only enabled after the received data input goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART overrun error 1: New data was received when the FIFO was full, resulting in data loss. 0: No data has been lost due to a FIFO overrun."]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data transmitted or received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<DrSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "UART data Important: This register is read-sensitive. See the register description for details. This register is the data register (the interface to the FIFOs). For transmitted data, if the FIFO is enabled, data written to this location is pushed onto the transmit FIFO. If the FIFO is disabled, data is stored in the transmitter holding register (the bottom word of the transmit FIFO). A write to this register initiates a transmission from the UART. For received data, if the FIFO is enabled, the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO. If the FIFO is disabled, the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data can be retrieved by reading this register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DrSpec;
impl crate::RegisterSpec for DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr::R`](R) reader structure"]
impl crate::Readable for DrSpec {}
#[doc = "`write(|w| ..)` method takes [`dr::W`](W) writer structure"]
impl crate::Writable for DrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DrSpec {
    const RESET_VALUE: u32 = 0;
}
