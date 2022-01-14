#[doc = "Register `SRCRESMASK2` reader"]
pub struct R(crate::R<SRCRESMASK2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCRESMASK2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCRESMASK2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCRESMASK2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCRESMASK2` writer"]
pub struct W(crate::W<SRCRESMASK2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCRESMASK2_SPEC>;
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
impl From<crate::W<SRCRESMASK2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCRESMASK2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCRESMASK2` reader - 24-bit mask that indicates source address match for each individual entry in the source address table"]
pub struct SRCRESMASK2_R(crate::FieldReader<u8, u8>);
impl SRCRESMASK2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRCRESMASK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRCRESMASK2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRCRESMASK2` writer - 24-bit mask that indicates source address match for each individual entry in the source address table"]
pub struct SRCRESMASK2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCRESMASK2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 24-bit mask that indicates source address match for each individual entry in the source address table"]
    #[inline(always)]
    pub fn srcresmask2(&self) -> SRCRESMASK2_R {
        SRCRESMASK2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 24-bit mask that indicates source address match for each individual entry in the source address table"]
    #[inline(always)]
    pub fn srcresmask2(&mut self) -> SRCRESMASK2_W {
        SRCRESMASK2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcresmask2](index.html) module"]
pub struct SRCRESMASK2_SPEC;
impl crate::RegisterSpec for SRCRESMASK2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcresmask2::R](R) reader structure"]
impl crate::Readable for SRCRESMASK2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcresmask2::W](W) writer structure"]
impl crate::Writable for SRCRESMASK2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRCRESMASK2 to value 0"]
impl crate::Resettable for SRCRESMASK2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
