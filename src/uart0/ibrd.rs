#[doc = "Register `IBRD` reader"]
pub struct R(crate::R<IBRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IBRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IBRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IBRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IBRD` writer"]
pub struct W(crate::W<IBRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IBRD_SPEC>;
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
impl From<crate::W<IBRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IBRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVINT` reader - Integer baud-rate divisor"]
pub type DIVINT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIVINT` writer - Integer baud-rate divisor"]
pub type DIVINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IBRD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Integer baud-rate divisor"]
    #[inline(always)]
    pub fn divint(&self) -> DIVINT_R {
        DIVINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Integer baud-rate divisor"]
    #[inline(always)]
    pub fn divint(&mut self) -> DIVINT_W<0> {
        DIVINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART integer baud-rate divisor The IBRD register is the integer part of the baud-rate divisor value. All the bits are cleared on reset. The minimum possible divide ratio is 1 (when IBRD = 0), in which case the FBRD register is ignored. When changing the IBRD register, the new value does not take effect until transmission or reception of the current character is complete. Any changes to the baud-rate divisor must be followed by a write to the LCRH register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ibrd](index.html) module"]
pub struct IBRD_SPEC;
impl crate::RegisterSpec for IBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ibrd::R](R) reader structure"]
impl crate::Readable for IBRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ibrd::W](W) writer structure"]
impl crate::Writable for IBRD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IBRD to value 0"]
impl crate::Resettable for IBRD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
