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
#[doc = "Field `UART1` reader - 0: UART1 module is not reset 1: UART1 module is reset"]
pub struct UART1_R(crate::FieldReader<bool, bool>);
impl UART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART1` writer - 0: UART1 module is not reset 1: UART1 module is reset"]
pub struct UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `UART0` reader - 0: UART0 module is not reset 1: UART0 module is reset"]
pub struct UART0_R(crate::FieldReader<bool, bool>);
impl UART0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART0` writer - 0: UART0 module is not reset 1: UART0 module is reset"]
pub struct UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - 0: UART1 module is not reset 1: UART1 module is reset"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - 0: UART0 module is not reset 1: UART0 module is reset"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - 0: UART1 module is not reset 1: UART1 module is reset"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W { w: self }
    }
    #[doc = "Bit 0 - 0: UART0 module is not reset 1: UART0 module is reset"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W { w: self }
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
