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
pub type SHORT_ADDR_EN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHORT_ADDR_EN` writer - 7:0 part of the 24-bit word SHORT_ADDR_EN that enables or disables source address matching for each of the 24 short address table entries Optional safety feature: To ensure that an entry in the source matching table is not used while it is being updated, set the corresponding SHORT_ADDR_EN bit to 0 while updating."]
pub type SHORT_ADDR_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRCSHORTEN0_SPEC, u8, u8, 8, O>;
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
    #[must_use]
    pub fn short_addr_en(&mut self) -> SHORT_ADDR_EN_W<0> {
        SHORT_ADDR_EN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCSHORTEN0 to value 0"]
impl crate::Resettable for SRCSHORTEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
