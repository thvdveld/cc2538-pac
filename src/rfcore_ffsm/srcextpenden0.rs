#[doc = "Register `SRCEXTPENDEN0` reader"]
pub struct R(crate::R<SRCEXTPENDEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCEXTPENDEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCEXTPENDEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCEXTPENDEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCEXTPENDEN0` writer"]
pub struct W(crate::W<SRCEXTPENDEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCEXTPENDEN0_SPEC>;
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
impl From<crate::W<SRCEXTPENDEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCEXTPENDEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCEXTPENDEN0` reader - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type SRCEXTPENDEN0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCEXTPENDEN0` writer - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type SRCEXTPENDEN0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRCEXTPENDEN0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden0(&self) -> SRCEXTPENDEN0_R {
        SRCEXTPENDEN0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses. Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden0(&mut self) -> SRCEXTPENDEN0_W<0> {
        SRCEXTPENDEN0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcextpenden0](index.html) module"]
pub struct SRCEXTPENDEN0_SPEC;
impl crate::RegisterSpec for SRCEXTPENDEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcextpenden0::R](R) reader structure"]
impl crate::Readable for SRCEXTPENDEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcextpenden0::W](W) writer structure"]
impl crate::Writable for SRCEXTPENDEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRCEXTPENDEN0 to value 0"]
impl crate::Resettable for SRCEXTPENDEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
