#[doc = "Register `SRCEXTPENDEN1` reader"]
pub struct R(crate::R<SRCEXTPENDEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCEXTPENDEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCEXTPENDEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCEXTPENDEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCEXTPENDEN1` writer"]
pub struct W(crate::W<SRCEXTPENDEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCEXTPENDEN1_SPEC>;
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
impl From<crate::W<SRCEXTPENDEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCEXTPENDEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCEXTPENDEN1` reader - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub struct SRCEXTPENDEN1_R(crate::FieldReader<u8, u8>);
impl SRCEXTPENDEN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SRCEXTPENDEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRCEXTPENDEN1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRCEXTPENDEN1` writer - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
pub struct SRCEXTPENDEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCEXTPENDEN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden1(&self) -> SRCEXTPENDEN1_R {
        SRCEXTPENDEN1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 middle bits of the 24-bit mask that enables or disables automatic pending for each of the 12 extended addresses Entry n is mapped to SRCEXTPENDEN\\[2n\\]. All SRCEXTPENDEN\\[2n + 1\\]
bits are don't care."]
    #[inline(always)]
    pub fn srcextpenden1(&mut self) -> SRCEXTPENDEN1_W {
        SRCEXTPENDEN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcextpenden1](index.html) module"]
pub struct SRCEXTPENDEN1_SPEC;
impl crate::RegisterSpec for SRCEXTPENDEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcextpenden1::R](R) reader structure"]
impl crate::Readable for SRCEXTPENDEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcextpenden1::W](W) writer structure"]
impl crate::Writable for SRCEXTPENDEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRCEXTPENDEN1 to value 0"]
impl crate::Resettable for SRCEXTPENDEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
