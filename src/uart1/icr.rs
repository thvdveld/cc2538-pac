#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `RXIC` writer - Receive interrupt clear Writing 1 to this bit clears the RXRIS bit in the UARTRIS register and the RXMIS bit in the UARTMIS register."]
pub type RXIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXIC` writer - Transmit interrupt clear Writing 1 to this bit clears the TXRIS bit in the UARTRIS register and the TXMIS bit in the UARTMIS register."]
pub type TXIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTIC` writer - Receive time-out interrupt clear Writing 1 to this bit clears the RTRIS bit in the UARTRIS register and the RTMIS bit in the UARTMIS register."]
pub type RTIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `FEIC` writer - Framing error interrupt clear Writing 1 to this bit clears the FERIS bit in the UARTRIS register and the FEMIS bit in the UARTMIS register."]
pub type FEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PEIC` writer - Parity error interrupt clear Writing 1 to this bit clears the PERIS bit in the UARTRIS register and the PEMIS bit in the UARTMIS register."]
pub type PEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `BEIC` writer - Break error interrupt clear Writing 1 to this bit clears the BERIS bit in the UARTRIS register and the BEMIS bit in the UARTMIS register."]
pub type BEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OEIC` writer - Overrun error interrupt clear Writing 1 to this bit clears the OERIS bit in the UARTRIS register and the OEMIS bit in the UARTMIS register."]
pub type OEIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `NINEBITIC` writer - 9-bit mode interrupt clear Writing 1 to this bit clears the 9BITRIS bit in the UARTRIS register and the 9BITMIS bit in the UARTMIS register."]
pub type NINEBITIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LMSBIC` writer - LIN mode sync break interrupt clear Writing 1 to this bit clears the LMSBRIS bit in the UARTRIS register and the LMSBMIS bit in the UARTMIS register."]
pub type LMSBIC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LME1IC` writer - LIN mode edge 1 interrupt clear Writing 1 to this bit clears the LME1RIS bit in the UARTRIS register and the LME1MIS bit in the UARTMIS register."]
pub type LME1IC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LME5IC` writer - LIN mode edge 5 interrupt clear Writing 1 to this bit clears the LME5RIS bit in the UARTRIS register and the LME5MIS bit in the UARTMIS register."]
pub type LME5IC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl W {
    #[doc = "Bit 4 - Receive interrupt clear Writing 1 to this bit clears the RXRIS bit in the UARTRIS register and the RXMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn rxic(&mut self) -> RXIC_W<ICR_SPEC, 4> {
        RXIC_W::new(self)
    }
    #[doc = "Bit 5 - Transmit interrupt clear Writing 1 to this bit clears the TXRIS bit in the UARTRIS register and the TXMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn txic(&mut self) -> TXIC_W<ICR_SPEC, 5> {
        TXIC_W::new(self)
    }
    #[doc = "Bit 6 - Receive time-out interrupt clear Writing 1 to this bit clears the RTRIS bit in the UARTRIS register and the RTMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RTIC_W<ICR_SPEC, 6> {
        RTIC_W::new(self)
    }
    #[doc = "Bit 7 - Framing error interrupt clear Writing 1 to this bit clears the FERIS bit in the UARTRIS register and the FEMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn feic(&mut self) -> FEIC_W<ICR_SPEC, 7> {
        FEIC_W::new(self)
    }
    #[doc = "Bit 8 - Parity error interrupt clear Writing 1 to this bit clears the PERIS bit in the UARTRIS register and the PEMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn peic(&mut self) -> PEIC_W<ICR_SPEC, 8> {
        PEIC_W::new(self)
    }
    #[doc = "Bit 9 - Break error interrupt clear Writing 1 to this bit clears the BERIS bit in the UARTRIS register and the BEMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn beic(&mut self) -> BEIC_W<ICR_SPEC, 9> {
        BEIC_W::new(self)
    }
    #[doc = "Bit 10 - Overrun error interrupt clear Writing 1 to this bit clears the OERIS bit in the UARTRIS register and the OEMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn oeic(&mut self) -> OEIC_W<ICR_SPEC, 10> {
        OEIC_W::new(self)
    }
    #[doc = "Bit 12 - 9-bit mode interrupt clear Writing 1 to this bit clears the 9BITRIS bit in the UARTRIS register and the 9BITMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn ninebitic(&mut self) -> NINEBITIC_W<ICR_SPEC, 12> {
        NINEBITIC_W::new(self)
    }
    #[doc = "Bit 13 - LIN mode sync break interrupt clear Writing 1 to this bit clears the LMSBRIS bit in the UARTRIS register and the LMSBMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn lmsbic(&mut self) -> LMSBIC_W<ICR_SPEC, 13> {
        LMSBIC_W::new(self)
    }
    #[doc = "Bit 14 - LIN mode edge 1 interrupt clear Writing 1 to this bit clears the LME1RIS bit in the UARTRIS register and the LME1MIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn lme1ic(&mut self) -> LME1IC_W<ICR_SPEC, 14> {
        LME1IC_W::new(self)
    }
    #[doc = "Bit 15 - LIN mode edge 5 interrupt clear Writing 1 to this bit clears the LME5RIS bit in the UARTRIS register and the LME5MIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn lme5ic(&mut self) -> LME5IC_W<ICR_SPEC, 15> {
        LME5IC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART interrupt clear The ICR register is the interrupt clear register. On a write of 1, the corresponding interrupt (both raw interrupt and masked interrupt, if enabled) is cleared. A write of 0 has no effect.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
