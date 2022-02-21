#[doc = "Register `CHIS` reader"]
pub struct R(crate::R<CHIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHIS` writer"]
pub struct W(crate::W<CHIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHIS_SPEC>;
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
impl From<crate::W<CHIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHIS` reader - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
pub struct CHIS_R(crate::FieldReader<u32, u32>);
impl CHIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        CHIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIS` writer - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
pub struct CHIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CHIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn chis(&self) -> CHIS_R {
        CHIS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn chis(&mut self) -> CHIS_W {
        CHIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel interrupt status Each bit of the CHIS register represents the corresponding uDMA channel. A bit is set when that uDMA channel causes a completion interrupt. The bits are cleared by writing 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chis](index.html) module"]
pub struct CHIS_SPEC;
impl crate::RegisterSpec for CHIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chis::R](R) reader structure"]
impl crate::Readable for CHIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chis::W](W) writer structure"]
impl crate::Writable for CHIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHIS to value 0"]
impl crate::Resettable for CHIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
