#[doc = "Register `ENASET` reader"]
pub struct R(crate::R<ENASET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENASET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENASET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENASET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENASET` writer"]
pub struct W(crate::W<ENASET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENASET_SPEC>;
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
impl From<crate::W<ENASET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENASET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET` reader - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
pub struct SET_R(crate::FieldReader<u32, u32>);
impl SET_R {
    pub(crate) fn new(bits: u32) -> Self {
        SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SET` writer - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits |= value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel enable set Each bit of the ENASET register represents the corresponding uDMA channel. Setting a bit enables the corresponding uDMA channel. Reading the register returns the enable status of the channels. If a channel is enabled but the request mask is set (REQMASKSET), then the channel can be used for software-initiated transfers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enaset](index.html) module"]
pub struct ENASET_SPEC;
impl crate::RegisterSpec for ENASET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enaset::R](R) reader structure"]
impl crate::Readable for ENASET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enaset::W](W) writer structure"]
impl crate::Writable for ENASET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENASET to value 0"]
impl crate::Resettable for ENASET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
