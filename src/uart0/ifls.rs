#[doc = "Register `IFLS` reader"]
pub struct R(crate::R<IFLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFLS` writer"]
pub struct W(crate::W<IFLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFLS_SPEC>;
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
impl From<crate::W<IFLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXIFLSEL` reader - UART receive interrupt FIFO level select The trigger points for the receive interrupt are as follows: 0x0: RX FIFO >= 1/8 full 0x1: RX FIFO >= 1/4 full 0x2: RX FIFO >= 1/2 full (default) 0x3: RX FIFO >= 3/4 full 0x4: RX FIFO >= 7/8 full 0x5-0x7: Reserved"]
pub struct RXIFLSEL_R(crate::FieldReader<u8, u8>);
impl RXIFLSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RXIFLSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXIFLSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXIFLSEL` writer - UART receive interrupt FIFO level select The trigger points for the receive interrupt are as follows: 0x0: RX FIFO >= 1/8 full 0x1: RX FIFO >= 1/4 full 0x2: RX FIFO >= 1/2 full (default) 0x3: RX FIFO >= 3/4 full 0x4: RX FIFO >= 7/8 full 0x5-0x7: Reserved"]
pub struct RXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXIFLSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `TXIFLSEL` reader - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: 0x0: TX FIFO <= 7/8 empty 0x1: TX FIFO <= 3/4 empty 0x2: TX FIFO <= 1/2 empty (default) 0x3: TX FIFO <= 1/4 empty 0x4: TX FIFO <= 1/8 empty 0x5-0x7: Reserved Note: If the EOT bit in UARTCTL is set, the transmit interrupt is generated once the FIFO is completely empty and all data including stop bits have left the transmit serializer. In this case, the setting of TXIFLSEL is ignored."]
pub struct TXIFLSEL_R(crate::FieldReader<u8, u8>);
impl TXIFLSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TXIFLSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXIFLSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXIFLSEL` writer - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: 0x0: TX FIFO <= 7/8 empty 0x1: TX FIFO <= 3/4 empty 0x2: TX FIFO <= 1/2 empty (default) 0x3: TX FIFO <= 1/4 empty 0x4: TX FIFO <= 1/8 empty 0x5-0x7: Reserved Note: If the EOT bit in UARTCTL is set, the transmit interrupt is generated once the FIFO is completely empty and all data including stop bits have left the transmit serializer. In this case, the setting of TXIFLSEL is ignored."]
pub struct TXIFLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXIFLSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - UART receive interrupt FIFO level select The trigger points for the receive interrupt are as follows: 0x0: RX FIFO >= 1/8 full 0x1: RX FIFO >= 1/4 full 0x2: RX FIFO >= 1/2 full (default) 0x3: RX FIFO >= 3/4 full 0x4: RX FIFO >= 7/8 full 0x5-0x7: Reserved"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RXIFLSEL_R {
        RXIFLSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: 0x0: TX FIFO <= 7/8 empty 0x1: TX FIFO <= 3/4 empty 0x2: TX FIFO <= 1/2 empty (default) 0x3: TX FIFO <= 1/4 empty 0x4: TX FIFO <= 1/8 empty 0x5-0x7: Reserved Note: If the EOT bit in UARTCTL is set, the transmit interrupt is generated once the FIFO is completely empty and all data including stop bits have left the transmit serializer. In this case, the setting of TXIFLSEL is ignored."]
    #[inline(always)]
    pub fn txiflsel(&self) -> TXIFLSEL_R {
        TXIFLSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - UART receive interrupt FIFO level select The trigger points for the receive interrupt are as follows: 0x0: RX FIFO >= 1/8 full 0x1: RX FIFO >= 1/4 full 0x2: RX FIFO >= 1/2 full (default) 0x3: RX FIFO >= 3/4 full 0x4: RX FIFO >= 7/8 full 0x5-0x7: Reserved"]
    #[inline(always)]
    pub fn rxiflsel(&mut self) -> RXIFLSEL_W {
        RXIFLSEL_W { w: self }
    }
    #[doc = "Bits 0:2 - UART Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows: 0x0: TX FIFO <= 7/8 empty 0x1: TX FIFO <= 3/4 empty 0x2: TX FIFO <= 1/2 empty (default) 0x3: TX FIFO <= 1/4 empty 0x4: TX FIFO <= 1/8 empty 0x5-0x7: Reserved Note: If the EOT bit in UARTCTL is set, the transmit interrupt is generated once the FIFO is completely empty and all data including stop bits have left the transmit serializer. In this case, the setting of TXIFLSEL is ignored."]
    #[inline(always)]
    pub fn txiflsel(&mut self) -> TXIFLSEL_W {
        TXIFLSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART interrupt FIFO level select The IFLS register is the interrupt FIFO level select register. This register can be used to define the FIFO level at which the TXRIS and RXRIS bits in the RIS register are triggered. The interrupts are generated based on a transition through a level rather than being based on the level. That is, the interrupts are generated when the fill level progresses through the trigger level. For example, if the receive trigger level is set to the half-way mark, the interrupt is triggered as the module is receiving the 9th character. Out of reset, the TXIFLSEL and RXIFLSEL bits are configured so that the FIFOs trigger an interrupt at the half-way mark.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifls](index.html) module"]
pub struct IFLS_SPEC;
impl crate::RegisterSpec for IFLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifls::R](R) reader structure"]
impl crate::Readable for IFLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifls::W](W) writer structure"]
impl crate::Writable for IFLS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFLS to value 0"]
impl crate::Resettable for IFLS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
