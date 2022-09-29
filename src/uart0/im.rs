#[doc = "Register `IM` reader"]
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IM` writer"]
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXIM` reader - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type RXIM_R = crate::BitReader<bool>;
#[doc = "Field `RXIM` writer - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type RXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `TXIM` reader - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type TXIM_R = crate::BitReader<bool>;
#[doc = "Field `TXIM` writer - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type TXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `RTIM` reader - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type RTIM_R = crate::BitReader<bool>;
#[doc = "Field `RTIM` writer - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type RTIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `FEIM` reader - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type FEIM_R = crate::BitReader<bool>;
#[doc = "Field `FEIM` writer - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type FEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `PEIM` reader - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type PEIM_R = crate::BitReader<bool>;
#[doc = "Field `PEIM` writer - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type PEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `BEIM` reader - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type BEIM_R = crate::BitReader<bool>;
#[doc = "Field `BEIM` writer - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type BEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `OEIM` reader - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type OEIM_R = crate::BitReader<bool>;
#[doc = "Field `OEIM` writer - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
pub type OEIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `NINEBITIM` reader - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type NINEBITIM_R = crate::BitReader<bool>;
#[doc = "Field `NINEBITIM` writer - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type NINEBITIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `LMSBIM` reader - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type LMSBIM_R = crate::BitReader<bool>;
#[doc = "Field `LMSBIM` writer - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
pub type LMSBIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `LME1IM` reader - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type LME1IM_R = crate::BitReader<bool>;
#[doc = "Field `LME1IM` writer - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type LME1IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
#[doc = "Field `LME5IM` reader - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type LME5IM_R = crate::BitReader<bool>;
#[doc = "Field `LME5IM` writer - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
pub type LME5IM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 4 - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rxim(&self) -> RXIM_R {
        RXIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn txim(&self) -> TXIM_R {
        TXIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rtim(&self) -> RTIM_R {
        RTIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn feim(&self) -> FEIM_R {
        FEIM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn peim(&self) -> PEIM_R {
        PEIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn beim(&self) -> BEIM_R {
        BEIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn oeim(&self) -> OEIM_R {
        OEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn ninebitim(&self) -> NINEBITIM_R {
        NINEBITIM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lmsbim(&self) -> LMSBIM_R {
        LMSBIM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme1im(&self) -> LME1IM_R {
        LME1IM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme5im(&self) -> LME5IM_R {
        LME5IM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - UART receive interrupt mask 1: An interrupt is sent to the interrupt controller when the RXRIS bit in the UARTRIS register is set. 0: The RXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rxim(&mut self) -> RXIM_W<4> {
        RXIM_W::new(self)
    }
    #[doc = "Bit 5 - UART transmit interrupt mask 1: An interrupt is sent to the interrupt controller when the TXRIS bit in the UARTRIS register is set. 0: The TXRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn txim(&mut self) -> TXIM_W<5> {
        TXIM_W::new(self)
    }
    #[doc = "Bit 6 - UART receive time-out interrupt mask 1: An interrupt is sent to the interrupt controller when the RTRIS bit in the UARTRIS register is set. 0: The RTRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn rtim(&mut self) -> RTIM_W<6> {
        RTIM_W::new(self)
    }
    #[doc = "Bit 7 - UART framing error interrupt mask 1: An interrupt is sent to the interrupt controller when the FERIS bit in the UARTRIS register is set. 0: The FERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn feim(&mut self) -> FEIM_W<7> {
        FEIM_W::new(self)
    }
    #[doc = "Bit 8 - UART parity error interrupt mask 1: An interrupt is sent to the interrupt controller when the PERIS bit in the UARTRIS register is set. 0: The PERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn peim(&mut self) -> PEIM_W<8> {
        PEIM_W::new(self)
    }
    #[doc = "Bit 9 - UART break error interrupt mask 1: An interrupt is sent to the interrupt controller when the BERIS bit in the UARTRIS register is set. 0: The BERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn beim(&mut self) -> BEIM_W<9> {
        BEIM_W::new(self)
    }
    #[doc = "Bit 10 - UART overrun error interrupt mask 1: An interrupt is sent to the interrupt controller when the OERIS bit in the UARTRIS register is set. 0: The OERIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn oeim(&mut self) -> OEIM_W<10> {
        OEIM_W::new(self)
    }
    #[doc = "Bit 12 - 9-bit mode interrupt mask 1: An interrupt is sent to the interrupt controller when the 9BITRIS bit in the UARTRIS register is set. 0: The 9BITRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn ninebitim(&mut self) -> NINEBITIM_W<12> {
        NINEBITIM_W::new(self)
    }
    #[doc = "Bit 13 - LIN mode sync break interrupt mask 1: An interrupt is sent to the interrupt controller when the LMSBRIS bit in the UARTRIS register is set. 0: The LMSBRIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lmsbim(&mut self) -> LMSBIM_W<13> {
        LMSBIM_W::new(self)
    }
    #[doc = "Bit 14 - LIN mode edge 1 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME1RIS bit in the UARTRIS register is set. 0: The LME1RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme1im(&mut self) -> LME1IM_W<14> {
        LME1IM_W::new(self)
    }
    #[doc = "Bit 15 - LIN mode edge 5 interrupt mask 1: An interrupt is sent to the interrupt controller when the LME5RIS bit in the UARTRIS register is set. 0: The LME5RIS interrupt is suppressed and not sent to the interrupt controller."]
    #[inline(always)]
    pub fn lme5im(&mut self) -> LME5IM_W<15> {
        LME5IM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART interrupt mask The IM register is the interrupt mask set/clear register. On a read, this register gives the current value of the mask on the relevant interrupt. Setting a bit allows the corresponding raw interrupt signal to be routed to the interrupt controller. Clearing a bit prevents the raw interrupt signal from being sent to the interrupt controller.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](index.html) module"]
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [im::R](R) reader structure"]
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [im::W](W) writer structure"]
impl crate::Writable for IM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IM to value 0"]
impl crate::Resettable for IM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
