#[doc = "Register `SRCSHORTPENDEN0` reader"]
pub struct R(crate::R<SRCSHORTPENDEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCSHORTPENDEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCSHORTPENDEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCSHORTPENDEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCSHORTPENDEN0` writer"]
pub struct W(crate::W<SRCSHORTPENDEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCSHORTPENDEN0_SPEC>;
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
impl From<crate::W<SRCSHORTPENDEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCSHORTPENDEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCSHORTPENDEN0` reader - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type SRCSHORTPENDEN0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCSHORTPENDEN0` writer - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type SRCSHORTPENDEN0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRCSHORTPENDEN0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden0(&self) -> SRCSHORTPENDEN0_R {
        SRCSHORTPENDEN0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden0(&mut self) -> SRCSHORTPENDEN0_W<0> {
        SRCSHORTPENDEN0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcshortpenden0](index.html) module"]
pub struct SRCSHORTPENDEN0_SPEC;
impl crate::RegisterSpec for SRCSHORTPENDEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcshortpenden0::R](R) reader structure"]
impl crate::Readable for SRCSHORTPENDEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcshortpenden0::W](W) writer structure"]
impl crate::Writable for SRCSHORTPENDEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRCSHORTPENDEN0 to value 0"]
impl crate::Resettable for SRCSHORTPENDEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
