#[doc = "Register `SRCRESINDEX` reader"]
pub struct R(crate::R<SRCRESINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRCRESINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRCRESINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRCRESINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRCRESINDEX` writer"]
pub struct W(crate::W<SRCRESINDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRCRESINDEX_SPEC>;
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
impl From<crate::W<SRCRESINDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRCRESINDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRCRESINDEX` reader - The bit index of the least-significant entry (0-23 for short addresses or 0-11 for extended addresses) in SRCRESMASK, or 0x3F when there is no source match On a match, bit 5 is 0 when the match is on a short address and 1 when it is on an extended address. On a match, bit 6 is 1 when the conditions for automatic pending bit in acknowledgment have been met (see the description of SRCMATCH.AUTOPEND). The bit does not indicate if the acknowledgment is actually transmitted, and does not consider the PENDING_OR register bit and the SACK/SACKPEND/SNACK strobes."]
pub type SRCRESINDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SRCRESINDEX` writer - The bit index of the least-significant entry (0-23 for short addresses or 0-11 for extended addresses) in SRCRESMASK, or 0x3F when there is no source match On a match, bit 5 is 0 when the match is on a short address and 1 when it is on an extended address. On a match, bit 6 is 1 when the conditions for automatic pending bit in acknowledgment have been met (see the description of SRCMATCH.AUTOPEND). The bit does not indicate if the acknowledgment is actually transmitted, and does not consider the PENDING_OR register bit and the SACK/SACKPEND/SNACK strobes."]
pub type SRCRESINDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRCRESINDEX_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - The bit index of the least-significant entry (0-23 for short addresses or 0-11 for extended addresses) in SRCRESMASK, or 0x3F when there is no source match On a match, bit 5 is 0 when the match is on a short address and 1 when it is on an extended address. On a match, bit 6 is 1 when the conditions for automatic pending bit in acknowledgment have been met (see the description of SRCMATCH.AUTOPEND). The bit does not indicate if the acknowledgment is actually transmitted, and does not consider the PENDING_OR register bit and the SACK/SACKPEND/SNACK strobes."]
    #[inline(always)]
    pub fn srcresindex(&self) -> SRCRESINDEX_R {
        SRCRESINDEX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The bit index of the least-significant entry (0-23 for short addresses or 0-11 for extended addresses) in SRCRESMASK, or 0x3F when there is no source match On a match, bit 5 is 0 when the match is on a short address and 1 when it is on an extended address. On a match, bit 6 is 1 when the conditions for automatic pending bit in acknowledgment have been met (see the description of SRCMATCH.AUTOPEND). The bit does not indicate if the acknowledgment is actually transmitted, and does not consider the PENDING_OR register bit and the SACK/SACKPEND/SNACK strobes."]
    #[inline(always)]
    #[must_use]
    pub fn srcresindex(&mut self) -> SRCRESINDEX_W<0> {
        SRCRESINDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source address matching result This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcresindex](index.html) module"]
pub struct SRCRESINDEX_SPEC;
impl crate::RegisterSpec for SRCRESINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srcresindex::R](R) reader structure"]
impl crate::Readable for SRCRESINDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srcresindex::W](W) writer structure"]
impl crate::Writable for SRCRESINDEX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRCRESINDEX to value 0"]
impl crate::Resettable for SRCRESINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
