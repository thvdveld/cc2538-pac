#[doc = "Register `RCGCUART` reader"]
pub struct R(crate::R<RCGCUART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCGCUART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCGCUART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCGCUART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCGCUART` writer"]
pub struct W(crate::W<RCGCUART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCGCUART_SPEC>;
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
impl From<crate::W<RCGCUART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCGCUART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART0` reader - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - 0: Clock for UART0 is gated. 1: Clock for UART0 is enabled."]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCGCUART_SPEC, bool, O>;
#[doc = "Field `UART1` reader - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `UART1` writer - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCGCUART_SPEC, bool, O>;
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
    pub fn uart0(&mut self) -> UART0_W<0> {
        UART0_W::new(self)
    }
    #[doc = "Bit 1 - 0: Clock for UART1 is gated. 1: Clock for UART1 is enabled."]
    #[inline(always)]
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
when the CPU is in active (run) mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgcuart](index.html) module"]
pub struct RCGCUART_SPEC;
impl crate::RegisterSpec for RCGCUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcgcuart::R](R) reader structure"]
impl crate::Readable for RCGCUART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcgcuart::W](W) writer structure"]
impl crate::Writable for RCGCUART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCGCUART to value 0"]
impl crate::Resettable for RCGCUART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
