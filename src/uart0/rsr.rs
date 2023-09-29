#[doc = "Register `RSR` reader"]
pub type R = crate::R<RSR_SPEC>;
#[doc = "Field `FE` reader - UART framing error 1: The received character does not have a valid stop bit (a valid stop bit is 1). 0: No framing error has occurred. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type FE_R = crate::BitReader;
#[doc = "Field `PE` reader - UART parity error 1: The parity of the received data character does not match the parity defined by bits 2 and 7 of the UARTLCRH register. 0: No parity error has occurred. This bit is cleared to 0 by a write to UARTECR."]
pub type PE_R = crate::BitReader;
#[doc = "Field `BE` reader - UART break error 1: A break condition has been detected, indicating that the receive data input was held low for longer than a full-word transmission time (defined as start, data, parity, and stop bits). 0: No break condition has occurred. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
pub type BE_R = crate::BitReader;
#[doc = "Field `OE` reader - UART overrun error 1: New data was received when the FIFO was full, resulting in data loss. 0: No data has been lost due to a FIFO overrun. This bit is cleared by a write to UARTECR. The FIFO contents remain valid because no further data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must read the data in order to empty the FIFO."]
pub type OE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - UART framing error 1: The received character does not have a valid stop bit (a valid stop bit is 1). 0: No framing error has occurred. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART parity error 1: The parity of the received data character does not match the parity defined by bits 2 and 7 of the UARTLCRH register. 0: No parity error has occurred. This bit is cleared to 0 by a write to UARTECR."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART break error 1: A break condition has been detected, indicating that the receive data input was held low for longer than a full-word transmission time (defined as start, data, parity, and stop bits). 0: No break condition has occurred. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART overrun error 1: New data was received when the FIFO was full, resulting in data loss. 0: No data has been lost due to a FIFO overrun. This bit is cleared by a write to UARTECR. The FIFO contents remain valid because no further data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must read the data in order to empty the FIFO."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "UART receive status and error clear The RSR/ECR register is the receive status register and error clear register. In addition to the DR register, receive status can also be read from the RSR register. If the status is read from this register, then the status information corresponds to the entry read from DR before reading RSR. The status information for overrun is set immediately when an overrun condition occurs. The RSR register cannot be written. Read-only status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSR_SPEC;
impl crate::RegisterSpec for RSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsr::R`](R) reader structure"]
impl crate::Readable for RSR_SPEC {}
#[doc = "`reset()` method sets RSR to value 0"]
impl crate::Resettable for RSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
