#[doc = "Register `PRIOSET` reader"]
pub struct R(crate::R<PRIOSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIOSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIOSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIOSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIOSET` writer"]
pub struct W(crate::W<PRIOSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIOSET_SPEC>;
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
impl From<crate::W<PRIOSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIOSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET` reader - Channel \\[n\\]
priority set 0: uDMA channel \\[n\\]
is using the default priority level 1: uDMA channel \\[n\\]
is using a high priority level Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAPRIOCLR register."]
pub type SET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SET` writer - Channel \\[n\\]
priority set 0: uDMA channel \\[n\\]
is using the default priority level 1: uDMA channel \\[n\\]
is using a high priority level Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAPRIOCLR register."]
pub type SET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRIOSET_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
priority set 0: uDMA channel \\[n\\]
is using the default priority level 1: uDMA channel \\[n\\]
is using a high priority level Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAPRIOCLR register."]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
priority set 0: uDMA channel \\[n\\]
is using the default priority level 1: uDMA channel \\[n\\]
is using a high priority level Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAPRIOCLR register."]
    #[inline(always)]
    #[must_use]
    pub fn set(&mut self) -> SET_W<0> {
        SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel priority set Each bit of the PRIOSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to have a high priority level. Reading the register returns the status of the channel priority mask.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prioset](index.html) module"]
pub struct PRIOSET_SPEC;
impl crate::RegisterSpec for PRIOSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prioset::R](R) reader structure"]
impl crate::Readable for PRIOSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prioset::W](W) writer structure"]
impl crate::Writable for PRIOSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIOSET to value 0"]
impl crate::Resettable for PRIOSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
