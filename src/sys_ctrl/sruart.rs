#[doc = "Register `SRUART` reader"]
pub struct R(crate::R<SRUART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRUART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRUART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRUART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRUART` writer"]
pub struct W(crate::W<SRUART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRUART_SPEC>;
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
impl From<crate::W<SRUART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRUART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART0` reader - 0: UART0 module is not reset 1: UART0 module is reset"]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - 0: UART0 module is not reset 1: UART0 module is reset"]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRUART_SPEC, bool, O>;
#[doc = "Field `UART1` reader - 0: UART1 module is not reset 1: UART1 module is reset"]
pub type UART1_R = crate::BitReader<bool>;
#[doc = "Field `UART1` writer - 0: UART1 module is not reset 1: UART1 module is reset"]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SRUART_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0: UART0 module is not reset 1: UART0 module is reset"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: UART1 module is not reset 1: UART1 module is reset"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: UART0 module is not reset 1: UART0 module is reset"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W<0> {
        UART0_W::new(self)
    }
    #[doc = "Bit 1 - 0: UART1 module is not reset 1: UART1 module is reset"]
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
#[doc = "This register controls the reset for UART\\[1:0\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sruart](index.html) module"]
pub struct SRUART_SPEC;
impl crate::RegisterSpec for SRUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sruart::R](R) reader structure"]
impl crate::Readable for SRUART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sruart::W](W) writer structure"]
impl crate::Writable for SRUART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRUART to value 0"]
impl crate::Resettable for SRUART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
