#[doc = "Register `FBRD` reader"]
pub struct R(crate::R<FBRD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FBRD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FBRD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FBRD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FBRD` writer"]
pub struct W(crate::W<FBRD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FBRD_SPEC>;
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
impl From<crate::W<FBRD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FBRD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVFRAC` reader - Fractional baud-rate divisor"]
pub type DIVFRAC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVFRAC` writer - Fractional baud-rate divisor"]
pub type DIVFRAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FBRD_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Fractional baud-rate divisor"]
    #[inline(always)]
    pub fn divfrac(&self) -> DIVFRAC_R {
        DIVFRAC_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fractional baud-rate divisor"]
    #[inline(always)]
    #[must_use]
    pub fn divfrac(&mut self) -> DIVFRAC_W<0> {
        DIVFRAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART fractional baud-rate divisor The FBRD register is the fractional part of the baud-rate divisor value. All the bits are cleared on reset. When changing the FBRD register, the new value does not take effect until transmission or reception of the current character is complete. Any changes to the baud-rate divisor must be followed by a write to the LCRH register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fbrd](index.html) module"]
pub struct FBRD_SPEC;
impl crate::RegisterSpec for FBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fbrd::R](R) reader structure"]
impl crate::Readable for FBRD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fbrd::W](W) writer structure"]
impl crate::Writable for FBRD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FBRD to value 0"]
impl crate::Resettable for FBRD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
