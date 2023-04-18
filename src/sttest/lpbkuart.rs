#[doc = "Register `LPBKUART` reader"]
pub struct R(crate::R<LPBKUART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPBKUART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPBKUART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPBKUART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPBKUART` writer"]
pub struct W(crate::W<LPBKUART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPBKUART_SPEC>;
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
impl From<crate::W<LPBKUART_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPBKUART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPBKUART` reader - UART0/1 loopback mode 0: Normal operation 1: UART0 TX (RX) connected to UART1 RX (TX)"]
pub type LPBKUART_R = crate::BitReader<bool>;
#[doc = "Field `LPBKUART` writer - UART0/1 loopback mode 0: Normal operation 1: UART0 TX (RX) connected to UART1 RX (TX)"]
pub type LPBKUART_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPBKUART_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - UART0/1 loopback mode 0: Normal operation 1: UART0 TX (RX) connected to UART1 RX (TX)"]
    #[inline(always)]
    pub fn lpbkuart(&self) -> LPBKUART_R {
        LPBKUART_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0/1 loopback mode 0: Normal operation 1: UART0 TX (RX) connected to UART1 RX (TX)"]
    #[inline(always)]
    #[must_use]
    pub fn lpbkuart(&mut self) -> LPBKUART_W<0> {
        LPBKUART_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART internal loopback\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpbkuart](index.html) module"]
pub struct LPBKUART_SPEC;
impl crate::RegisterSpec for LPBKUART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpbkuart::R](R) reader structure"]
impl crate::Readable for LPBKUART_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpbkuart::W](W) writer structure"]
impl crate::Writable for LPBKUART_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPBKUART to value 0"]
impl crate::Resettable for LPBKUART_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
