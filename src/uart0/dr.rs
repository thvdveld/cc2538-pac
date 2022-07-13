#[doc = "Register `DR` reader"]
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DR` writer"]
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OE` reader - UART overrun error 1: New data was received when the FIFO was full, resulting in data loss. 0: No data has been lost due to a FIFO overrun."]
pub type OE_R = crate::BitReader<bool>;
#[doc = "Field `BE` reader - UART break error 1: A break condition has been detected, indicating that the receive data input was held low for longer than a full-word transmission time (defined as start, data, parity, and stop bits). 0: No break condition has occurred. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only the one 0 character is loaded into the FIFO. The next character is only enabled after the received data input goes to a 1 (marking state), and the next valid start bit is received."]
pub type BE_R = crate::BitReader<bool>;
#[doc = "Field `PE` reader - UART parity error 1: The parity of the received data character does not match the parity defined by bits 2 and 7 of the UARTLCRH register 0: No parity error has occurred. In FIFO mode, this error is associated with the character at the top of the FIFO."]
pub type PE_R = crate::BitReader<bool>;
#[doc = "Field `FE` reader - UART framing error 1: The received character does not have a valid stop bit (a valid stop bit is 1). 0: No framing error has occurred."]
pub type FE_R = crate::BitReader<bool>;
#[doc = "Field `DATA` reader - Data transmitted or received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Data transmitted or received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 11 - UART overrun error 1: New data was received when the FIFO was full, resulting in data loss. 0: No data has been lost due to a FIFO overrun."]
    #[inline(always)]
    pub fn oe(&self) -> OE_R {
        OE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - UART break error 1: A break condition has been detected, indicating that the receive data input was held low for longer than a full-word transmission time (defined as start, data, parity, and stop bits). 0: No break condition has occurred. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only the one 0 character is loaded into the FIFO. The next character is only enabled after the received data input goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    pub fn be(&self) -> BE_R {
        BE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - UART parity error 1: The parity of the received data character does not match the parity defined by bits 2 and 7 of the UARTLCRH register 0: No parity error has occurred. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - UART framing error 1: The received character does not have a valid stop bit (a valid stop bit is 1). 0: No framing error has occurred."]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 0:7 - Data transmitted or received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data transmitted or received Data that is to be transmitted via the UART is written to this field. When read, this field contains the data that was received by the UART."]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART data Important: This register is read-sensitive. See the register description for details. This register is the data register (the interface to the FIFOs). For transmitted data, if the FIFO is enabled, data written to this location is pushed onto the transmit FIFO. If the FIFO is disabled, data is stored in the transmitter holding register (the bottom word of the transmit FIFO). A write to this register initiates a transmission from the UART. For received data, if the FIFO is enabled, the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO. If the FIFO is disabled, the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data can be retrieved by reading this register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](index.html) module"]
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dr::R](R) reader structure"]
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dr::W](W) writer structure"]
impl crate::Writable for DR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DR to value 0"]
impl crate::Resettable for DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
