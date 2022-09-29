#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXIC` writer - Receive interrupt clear Writing 1 to this bit clears the RXRIS bit in the UARTRIS register and the RXMIS bit in the UARTMIS register."]
pub type RXIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TXIC` writer - Transmit interrupt clear Writing 1 to this bit clears the TXRIS bit in the UARTRIS register and the TXMIS bit in the UARTMIS register."]
pub type TXIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `RTIC` writer - Receive time-out interrupt clear Writing 1 to this bit clears the RTRIS bit in the UARTRIS register and the RTMIS bit in the UARTMIS register."]
pub type RTIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `FEIC` writer - Framing error interrupt clear Writing 1 to this bit clears the FERIS bit in the UARTRIS register and the FEMIS bit in the UARTMIS register."]
pub type FEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `PEIC` writer - Parity error interrupt clear Writing 1 to this bit clears the PERIS bit in the UARTRIS register and the PEMIS bit in the UARTMIS register."]
pub type PEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `BEIC` writer - Break error interrupt clear Writing 1 to this bit clears the BERIS bit in the UARTRIS register and the BEMIS bit in the UARTMIS register."]
pub type BEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `OEIC` writer - Overrun error interrupt clear Writing 1 to this bit clears the OERIS bit in the UARTRIS register and the OEMIS bit in the UARTMIS register."]
pub type OEIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `NINEBITIC` writer - 9-bit mode interrupt clear Writing 1 to this bit clears the 9BITRIS bit in the UARTRIS register and the 9BITMIS bit in the UARTMIS register."]
pub type NINEBITIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `LMSBIC` writer - LIN mode sync break interrupt clear Writing 1 to this bit clears the LMSBRIS bit in the UARTRIS register and the LMSBMIS bit in the UARTMIS register."]
pub type LMSBIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `LME1IC` writer - LIN mode edge 1 interrupt clear Writing 1 to this bit clears the LME1RIS bit in the UARTRIS register and the LME1MIS bit in the UARTMIS register."]
pub type LME1IC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `LME5IC` writer - LIN mode edge 5 interrupt clear Writing 1 to this bit clears the LME5RIS bit in the UARTRIS register and the LME5MIS bit in the UARTMIS register."]
pub type LME5IC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 4 - Receive interrupt clear Writing 1 to this bit clears the RXRIS bit in the UARTRIS register and the RXMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn rxic(&mut self) -> RXIC_W<4> {
        RXIC_W::new(self)
    }
    #[doc = "Bit 5 - Transmit interrupt clear Writing 1 to this bit clears the TXRIS bit in the UARTRIS register and the TXMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn txic(&mut self) -> TXIC_W<5> {
        TXIC_W::new(self)
    }
    #[doc = "Bit 6 - Receive time-out interrupt clear Writing 1 to this bit clears the RTRIS bit in the UARTRIS register and the RTMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn rtic(&mut self) -> RTIC_W<6> {
        RTIC_W::new(self)
    }
    #[doc = "Bit 7 - Framing error interrupt clear Writing 1 to this bit clears the FERIS bit in the UARTRIS register and the FEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn feic(&mut self) -> FEIC_W<7> {
        FEIC_W::new(self)
    }
    #[doc = "Bit 8 - Parity error interrupt clear Writing 1 to this bit clears the PERIS bit in the UARTRIS register and the PEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn peic(&mut self) -> PEIC_W<8> {
        PEIC_W::new(self)
    }
    #[doc = "Bit 9 - Break error interrupt clear Writing 1 to this bit clears the BERIS bit in the UARTRIS register and the BEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn beic(&mut self) -> BEIC_W<9> {
        BEIC_W::new(self)
    }
    #[doc = "Bit 10 - Overrun error interrupt clear Writing 1 to this bit clears the OERIS bit in the UARTRIS register and the OEMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn oeic(&mut self) -> OEIC_W<10> {
        OEIC_W::new(self)
    }
    #[doc = "Bit 12 - 9-bit mode interrupt clear Writing 1 to this bit clears the 9BITRIS bit in the UARTRIS register and the 9BITMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn ninebitic(&mut self) -> NINEBITIC_W<12> {
        NINEBITIC_W::new(self)
    }
    #[doc = "Bit 13 - LIN mode sync break interrupt clear Writing 1 to this bit clears the LMSBRIS bit in the UARTRIS register and the LMSBMIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn lmsbic(&mut self) -> LMSBIC_W<13> {
        LMSBIC_W::new(self)
    }
    #[doc = "Bit 14 - LIN mode edge 1 interrupt clear Writing 1 to this bit clears the LME1RIS bit in the UARTRIS register and the LME1MIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn lme1ic(&mut self) -> LME1IC_W<14> {
        LME1IC_W::new(self)
    }
    #[doc = "Bit 15 - LIN mode edge 5 interrupt clear Writing 1 to this bit clears the LME5RIS bit in the UARTRIS register and the LME5MIS bit in the UARTMIS register."]
    #[inline(always)]
    pub fn lme5ic(&mut self) -> LME5IC_W<15> {
        LME5IC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART interrupt clear The ICR register is the interrupt clear register. On a write of 1, the corresponding interrupt (both raw interrupt and masked interrupt, if enabled) is cleared. A write of 0 has no effect.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
