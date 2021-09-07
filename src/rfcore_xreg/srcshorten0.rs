#[doc = "Register `SRCSHORTEN0` reader"]
pub struct R(crate::R<SRCSHORTEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCSHORTEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCSHORTEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCSHORTEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCSHORTEN0` writer"]
pub struct W(crate::W<SRCSHORTEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCSHORTEN0_SPEC>;
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
impl From<crate::W<SRCSHORTEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCSHORTEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHORT_ADDR_EN` reader - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
pub struct SHORT_ADDR_EN_R(crate::FieldReader<u8, u8>);
impl SHORT_ADDR_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SHORT_ADDR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHORT_ADDR_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHORT_ADDR_EN` writer - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
pub struct SHORT_ADDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SHORT_ADDR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
    #[inline(always)]
    pub fn short_addr_en(&self) -> SHORT_ADDR_EN_R {
        SHORT_ADDR_EN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
    #[inline(always)]
    pub fn short_addr_en(&mut self) -> SHORT_ADDR_EN_W {
        SHORT_ADDR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Short address matching\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcshorten0](index.html) module"]
pub struct SRCSHORTEN0_SPEC;
impl crate::RegisterSpec for SRCSHORTEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcshorten0::R](R) reader structure"]
impl crate::Readable for SRCSHORTEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcshorten0::W](W) writer structure"]
impl crate::Writable for SRCSHORTEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRCSHORTEN0 to value 0"]
impl crate::Resettable for SRCSHORTEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
