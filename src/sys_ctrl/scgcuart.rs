#[doc = "Register `SCGCUART` reader"]
pub struct R(crate::R<SCGCUART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGCUART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGCUART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGCUART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGCUART` writer"]
pub struct W(crate::W<SCGCUART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGCUART_SPEC>;
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
impl From<crate::W<SCGCUART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGCUART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART0` reader - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGCUART_SPEC, bool, O>;
#[doc = "Field `UART1` reader - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `UART1` writer - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGCUART_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<0> {
        UART0_W::new(self)
    }
    #[doc = "Bit 1 - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<1> {
        UART1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the module clocks for UART\\[1:0\\]
when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgcuart](index.html) module"]
pub struct SCGCUART_SPEC;
impl crate::RegisterSpec for SCGCUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgcuart::R](R) reader structure"]
impl crate::Readable for SCGCUART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgcuart::W](W) writer structure"]
impl crate::Writable for SCGCUART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGCUART to value 0"]
impl crate::Resettable for SCGCUART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
