#[doc = "Register `SRCEXTPENDEN2` reader"]
pub struct R(crate::R<SRCEXTPENDEN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCEXTPENDEN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCEXTPENDEN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCEXTPENDEN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCEXTPENDEN2` writer"]
pub struct W(crate::W<SRCEXTPENDEN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCEXTPENDEN2_SPEC>;
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
impl From<crate::W<SRCEXTPENDEN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCEXTPENDEN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCEXTPENDEN2` reader - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type SRCEXTPENDEN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCEXTPENDEN2` writer - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub type SRCEXTPENDEN2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRCEXTPENDEN2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden2(&self) -> SRCEXTPENDEN2_R {
        SRCEXTPENDEN2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    #[must_use]
    pub fn srcextpenden2(&mut self) -> SRCEXTPENDEN2_W<0> {
        SRCEXTPENDEN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcextpenden2](index.html) module"]
pub struct SRCEXTPENDEN2_SPEC;
impl crate::RegisterSpec for SRCEXTPENDEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcextpenden2::R](R) reader structure"]
impl crate::Readable for SRCEXTPENDEN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcextpenden2::W](W) writer structure"]
impl crate::Writable for SRCEXTPENDEN2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCEXTPENDEN2 to value 0"]
impl crate::Resettable for SRCEXTPENDEN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
