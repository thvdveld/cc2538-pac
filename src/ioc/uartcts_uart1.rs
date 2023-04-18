#[doc = "Register `UARTCTS_UART1` reader"]
pub struct R(crate::R<UARTCTS_UART1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UARTCTS_UART1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UARTCTS_UART1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UARTCTS_UART1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UARTCTS_UART1` writer"]
pub struct W(crate::W<UARTCTS_UART1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UARTCTS_UART1_SPEC>;
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
impl From<crate::W<UARTCTS_UART1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UARTCTS_UART1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as UART1 CTS 1: PA1 selected as UART1 CTS ... 31: PD7 selected as UART1 CTS"]
pub type INPUT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as UART1 CTS 1: PA1 selected as UART1 CTS ... 31: PD7 selected as UART1 CTS"]
pub type INPUT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, UARTCTS_UART1_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as UART1 CTS 1: PA1 selected as UART1 CTS ... 31: PD7 selected as UART1 CTS"]
    #[inline(always)]
    pub fn input_sel(&self) -> INPUT_SEL_R {
        INPUT_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as UART1 CTS 1: PA1 selected as UART1 CTS ... 31: PD7 selected as UART1 CTS"]
    #[inline(always)]
    #[must_use]
    pub fn input_sel(&mut self) -> INPUT_SEL_W<0> {
        INPUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the UART1 CTS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uartcts_uart1](index.html) module"]
pub struct UARTCTS_UART1_SPEC;
impl crate::RegisterSpec for UARTCTS_UART1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uartcts_uart1::R](R) reader structure"]
impl crate::Readable for UARTCTS_UART1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uartcts_uart1::W](W) writer structure"]
impl crate::Writable for UARTCTS_UART1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UARTCTS_UART1 to value 0"]
impl crate::Resettable for UARTCTS_UART1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
