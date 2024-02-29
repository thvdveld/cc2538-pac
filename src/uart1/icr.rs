#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `RXIC` writer - Receive interrupt clear Writing 1 to this bit clears the RXRIS bit in the UARTRIS register and the RXMIS bit in the UARTMIS register."]
pub type RxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIC` writer - Transmit interrupt clear Writing 1 to this bit clears the TXRIS bit in the UARTRIS register and the TXMIS bit in the UARTMIS register."]
pub type TxicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIC` writer - Receive time-out interrupt clear Writing 1 to this bit clears the RTRIS bit in the UARTRIS register and the RTMIS bit in the UARTMIS register."]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIC` writer - Framing error interrupt clear Writing 1 to this bit clears the FERIS bit in the UARTRIS register and the FEMIS bit in the UARTMIS register."]
pub type FeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIC` writer - Parity error interrupt clear Writing 1 to this bit clears the PERIS bit in the UARTRIS register and the PEMIS bit in the UARTMIS register."]
pub type PeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIC` writer - Break error interrupt clear Writing 1 to this bit clears the BERIS bit in the UARTRIS register and the BEMIS bit in the UARTMIS register."]
pub type BeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIC` writer - Overrun error interrupt clear Writing 1 to this bit clears the OERIS bit in the UARTRIS register and the OEMIS bit in the UARTMIS register."]
pub type OeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NINEBITIC` writer - 9-bit mode interrupt clear Writing 1 to this bit clears the 9BITRIS bit in the UARTRIS register and the 9BITMIS bit in the UARTMIS register."]
pub type NinebiticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LMSBIC` writer - LIN mode sync break interrupt clear Writing 1 to this bit clears the LMSBRIS bit in the UARTRIS register and the LMSBMIS bit in the UARTMIS register."]
pub type LmsbicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LME1IC` writer - LIN mode edge 1 interrupt clear Writing 1 to this bit clears the LME1RIS bit in the UARTRIS register and the LME1MIS bit in the UARTMIS register."]
pub type Lme1icW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LME5IC` writer - LIN mode edge 5 interrupt clear Writing 1 to this bit clears the LME5RIS bit in the UARTRIS register and the LME5MIS bit in the UARTMIS register."]
pub type Lme5icW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 4 - Receive interrupt clear Writing 1 to this bit clears the RXRIS bit in the UARTRIS register and the RXMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn rxic(&mut self) -> RxicW<IcrSpec> {
        RxicW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit interrupt clear Writing 1 to this bit clears the TXRIS bit in the UARTRIS register and the TXMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn txic(&mut self) -> TxicW<IcrSpec> {
        TxicW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive time-out interrupt clear Writing 1 to this bit clears the RTRIS bit in the UARTRIS register and the RTMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn rtic(&mut self) -> RticW<IcrSpec> {
        RticW::new(self, 6)
    }
    #[doc = "Bit 7 - Framing error interrupt clear Writing 1 to this bit clears the FERIS bit in the UARTRIS register and the FEMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn feic(&mut self) -> FeicW<IcrSpec> {
        FeicW::new(self, 7)
    }
    #[doc = "Bit 8 - Parity error interrupt clear Writing 1 to this bit clears the PERIS bit in the UARTRIS register and the PEMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn peic(&mut self) -> PeicW<IcrSpec> {
        PeicW::new(self, 8)
    }
    #[doc = "Bit 9 - Break error interrupt clear Writing 1 to this bit clears the BERIS bit in the UARTRIS register and the BEMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn beic(&mut self) -> BeicW<IcrSpec> {
        BeicW::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun error interrupt clear Writing 1 to this bit clears the OERIS bit in the UARTRIS register and the OEMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn oeic(&mut self) -> OeicW<IcrSpec> {
        OeicW::new(self, 10)
    }
    #[doc = "Bit 12 - 9-bit mode interrupt clear Writing 1 to this bit clears the 9BITRIS bit in the UARTRIS register and the 9BITMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn ninebitic(&mut self) -> NinebiticW<IcrSpec> {
        NinebiticW::new(self, 12)
    }
    #[doc = "Bit 13 - LIN mode sync break interrupt clear Writing 1 to this bit clears the LMSBRIS bit in the UARTRIS register and the LMSBMIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn lmsbic(&mut self) -> LmsbicW<IcrSpec> {
        LmsbicW::new(self, 13)
    }
    #[doc = "Bit 14 - LIN mode edge 1 interrupt clear Writing 1 to this bit clears the LME1RIS bit in the UARTRIS register and the LME1MIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn lme1ic(&mut self) -> Lme1icW<IcrSpec> {
        Lme1icW::new(self, 14)
    }
    #[doc = "Bit 15 - LIN mode edge 5 interrupt clear Writing 1 to this bit clears the LME5RIS bit in the UARTRIS register and the LME5MIS bit in the UARTMIS register."]
    #[inline(always)]
    #[must_use]
    pub fn lme5ic(&mut self) -> Lme5icW<IcrSpec> {
        Lme5icW::new(self, 15)
    }
}
#[doc = "UART interrupt clear The ICR register is the interrupt clear register. On a write of 1, the corresponding interrupt (both raw interrupt and masked interrupt, if enabled) is cleared. A write of 0 has no effect.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
