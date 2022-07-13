#[doc = "Register `SRCSHORTPENDEN2` reader"]
pub struct R(crate::R<SRCSHORTPENDEN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCSHORTPENDEN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCSHORTPENDEN2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCSHORTPENDEN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCSHORTPENDEN2` writer"]
pub struct W(crate::W<SRCSHORTPENDEN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCSHORTPENDEN2_SPEC>;
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
impl From<crate::W<SRCSHORTPENDEN2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCSHORTPENDEN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCSHORTPENDEN2` reader - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type SRCSHORTPENDEN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCSHORTPENDEN2` writer - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type SRCSHORTPENDEN2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRCSHORTPENDEN2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden2(&self) -> SRCSHORTPENDEN2_R {
        SRCSHORTPENDEN2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 MSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden2(&mut self) -> SRCSHORTPENDEN2_W<0> {
        SRCSHORTPENDEN2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcshortpenden2](index.html) module"]
pub struct SRCSHORTPENDEN2_SPEC;
impl crate::RegisterSpec for SRCSHORTPENDEN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcshortpenden2::R](R) reader structure"]
impl crate::Readable for SRCSHORTPENDEN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcshortpenden2::W](W) writer structure"]
impl crate::Writable for SRCSHORTPENDEN2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRCSHORTPENDEN2 to value 0"]
impl crate::Resettable for SRCSHORTPENDEN2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
