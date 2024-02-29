#[doc = "Register `IM` reader"]
pub type R = crate::R<ImSpec>;
#[doc = "Register `IM` writer"]
pub type W = crate::W<ImSpec>;
#[doc = "Field `RXIM` reader - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type RximR = crate::BitReader;
#[doc = "Field `RXIM` writer - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type RximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXIM` reader - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type TximR = crate::BitReader;
#[doc = "Field `TXIM` writer - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type TximW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIM` reader - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type RtimR = crate::BitReader;
#[doc = "Field `RTIM` writer - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type RtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEIM` reader - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type FeimR = crate::BitReader;
#[doc = "Field `FEIM` writer - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type FeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEIM` reader - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type PeimR = crate::BitReader;
#[doc = "Field `PEIM` writer - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type PeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIM` reader - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type BeimR = crate::BitReader;
#[doc = "Field `BEIM` writer - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type BeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEIM` reader - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type OeimR = crate::BitReader;
#[doc = "Field `OEIM` writer - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type OeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NINEBITIM` reader - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type NinebitimR = crate::BitReader;
#[doc = "Field `NINEBITIM` writer - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type NinebitimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LMSBIM` reader - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type LmsbimR = crate::BitReader;
#[doc = "Field `LMSBIM` writer - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type LmsbimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LME1IM` reader - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type Lme1imR = crate::BitReader;
#[doc = "Field `LME1IM` writer - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type Lme1imW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LME5IM` reader - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type Lme5imR = crate::BitReader;
#[doc = "Field `LME5IM` writer - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type Lme5imW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rxim(&self) -> RximR {
        RximR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn txim(&self) -> TximR {
        TximR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rtim(&self) -> RtimR {
        RtimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn feim(&self) -> FeimR {
        FeimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn peim(&self) -> PeimR {
        PeimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn beim(&self) -> BeimR {
        BeimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn oeim(&self) -> OeimR {
        OeimR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn ninebitim(&self) -> NinebitimR {
        NinebitimR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lmsbim(&self) -> LmsbimR {
        LmsbimR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme1im(&self) -> Lme1imR {
        Lme1imR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme5im(&self) -> Lme5imR {
        Lme5imR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn rxim(&mut self) -> RximW<ImSpec> {
        RximW::new(self, 4)
    }
    #[doc = "Bit 5 - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn txim(&mut self) -> TximW<ImSpec> {
        TximW::new(self, 5)
    }
    #[doc = "Bit 6 - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn rtim(&mut self) -> RtimW<ImSpec> {
        RtimW::new(self, 6)
    }
    #[doc = "Bit 7 - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn feim(&mut self) -> FeimW<ImSpec> {
        FeimW::new(self, 7)
    }
    #[doc = "Bit 8 - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn peim(&mut self) -> PeimW<ImSpec> {
        PeimW::new(self, 8)
    }
    #[doc = "Bit 9 - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn beim(&mut self) -> BeimW<ImSpec> {
        BeimW::new(self, 9)
    }
    #[doc = "Bit 10 - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn oeim(&mut self) -> OeimW<ImSpec> {
        OeimW::new(self, 10)
    }
    #[doc = "Bit 12 - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn ninebitim(&mut self) -> NinebitimW<ImSpec> {
        NinebitimW::new(self, 12)
    }
    #[doc = "Bit 13 - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn lmsbim(&mut self) -> LmsbimW<ImSpec> {
        LmsbimW::new(self, 13)
    }
    #[doc = "Bit 14 - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn lme1im(&mut self) -> Lme1imW<ImSpec> {
        Lme1imW::new(self, 14)
    }
    #[doc = "Bit 15 - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    #[must_use]
    pub fn lme5im(&mut self) -> Lme5imW<ImSpec> {
        Lme5imW::new(self, 15)
    }
}
#[doc = "UART interrupt mask The IM register is the interrupt mask set/clear register. On a read, this register gives the current value of the mask on the relevant interrupt. Setting a bit allows the corresponding raw interrupt signal to be routed to the interrupt controller. Clearing a bit prevents the raw interrupt signal from being sent to the interrupt controller.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`im::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`im::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImSpec;
impl crate::RegisterSpec for ImSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`im::R`](R) reader structure"]
impl crate::Readable for ImSpec {}
#[doc = "`write(|w| ..)` method takes [`im::W`](W) writer structure"]
impl crate::Writable for ImSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for ImSpec {
    const RESET_VALUE: u32 = 0;
}
