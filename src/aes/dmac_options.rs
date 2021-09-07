#[doc = "Register `DMAC_OPTIONS` reader"]
pub struct R(crate::R<DMAC_OPTIONS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_OPTIONS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_OPTIONS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_OPTIONS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NR_OF_CHANNELS` reader - Number of channels implemented, value in the range 1-8."]
pub struct NR_OF_CHANNELS_R(crate::FieldReader<u8, u8>);
impl NR_OF_CHANNELS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NR_OF_CHANNELS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NR_OF_CHANNELS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NR_OF_PORTS` reader - Number of ports implemented, value in range 1-4."]
pub struct NR_OF_PORTS_R(crate::FieldReader<u8, u8>);
impl NR_OF_PORTS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NR_OF_PORTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NR_OF_PORTS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:11 - Number of channels implemented, value in the range 1-8."]
    #[inline(always)]
    pub fn nr_of_channels(&self) -> NR_OF_CHANNELS_R {
        NR_OF_CHANNELS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:2 - Number of ports implemented, value in range 1-4."]
    #[inline(always)]
    pub fn nr_of_ports(&self) -> NR_OF_PORTS_R {
        NR_OF_PORTS_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "DMAC options register These registers contain information regarding the different options configured in this DMAC.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_options](index.html) module"]
pub struct DMAC_OPTIONS_SPEC;
impl crate::RegisterSpec for DMAC_OPTIONS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_options::R](R) reader structure"]
impl crate::Readable for DMAC_OPTIONS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMAC_OPTIONS to value 0"]
impl crate::Resettable for DMAC_OPTIONS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
