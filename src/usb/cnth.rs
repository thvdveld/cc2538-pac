#[doc = "Register `CNTH` reader"]
pub struct R(crate::R<CNTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOCNTH` reader - Bits 10:8 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
pub struct FIFOCNTH_R(crate::FieldReader<u8, u8>);
impl FIFOCNTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFOCNTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOCNTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Bits 10:8 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
    #[inline(always)]
    pub fn fifocnth(&self) -> FIFOCNTH_R {
        FIFOCNTH_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Indexed register: For USB_INDEX = 1-5: Number of received in the OUT endpoint {1-5} FIFO (high byte)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnth](index.html) module"]
pub struct CNTH_SPEC;
impl crate::RegisterSpec for CNTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnth::R](R) reader structure"]
impl crate::Readable for CNTH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CNTH to value 0"]
impl crate::Resettable for CNTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
