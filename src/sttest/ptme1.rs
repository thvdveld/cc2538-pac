#[doc = "Register `PTME1` reader"]
pub struct R(crate::R<PTME1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTME1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTME1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTME1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTME1` writer"]
pub struct W(crate::W<PTME1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTME1_SPEC>;
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
impl From<crate::W<PTME1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTME1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART1TME` reader - UART1 test mode enable"]
pub type UART1TME_R = crate::BitReader<bool>;
#[doc = "Field `UART1TME` writer - UART1 test mode enable"]
pub type UART1TME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTME1_SPEC, bool, O>;
#[doc = "Field `UART0TME` reader - UART0 test mode enable"]
pub type UART0TME_R = crate::BitReader<bool>;
#[doc = "Field `UART0TME` writer - UART0 test mode enable"]
pub type UART0TME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTME1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 9 - UART1 test mode enable"]
    #[inline(always)]
    pub fn uart1tme(&self) -> UART1TME_R {
        UART1TME_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - UART0 test mode enable"]
    #[inline(always)]
    pub fn uart0tme(&self) -> UART0TME_R {
        UART0TME_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - UART1 test mode enable"]
    #[inline(always)]
    pub fn uart1tme(&mut self) -> UART1TME_W<9> {
        UART1TME_W::new(self)
    }
    #[doc = "Bit 8 - UART0 test mode enable"]
    #[inline(always)]
    pub fn uart0tme(&mut self) -> UART0TME_W<8> {
        UART0TME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral test mode enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptme1](index.html) module"]
pub struct PTME1_SPEC;
impl crate::RegisterSpec for PTME1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptme1::R](R) reader structure"]
impl crate::Readable for PTME1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptme1::W](W) writer structure"]
impl crate::Writable for PTME1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTME1 to value 0"]
impl crate::Resettable for PTME1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
