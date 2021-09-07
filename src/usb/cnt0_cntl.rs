#[doc = "Register `CNT0_CNTL` reader"]
pub struct R(crate::R<CNT0_CNTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT0_CNTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT0_CNTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT0_CNTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIFOCNT_or_FIFOCNTL` reader - USB_CS0.FIFOCNT (USBINDEX = 0) \\[RO\\]: Number of bytes received in the packet in the endpoint 0 FIFO Valid only when USB_CS0.OUTPKTRDY is set USB_CSIL.FIFOCNTL (USBINDEX = 1 to 5) \\[RW\\]: Bits 7:0 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
pub struct FIFOCNT_OR_FIFOCNTL_R(crate::FieldReader<u8, u8>);
impl FIFOCNT_OR_FIFOCNTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FIFOCNT_OR_FIFOCNTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOCNT_OR_FIFOCNTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - USB_CS0.FIFOCNT (USBINDEX = 0) \\[RO\\]: Number of bytes received in the packet in the endpoint 0 FIFO Valid only when USB_CS0.OUTPKTRDY is set USB_CSIL.FIFOCNTL (USBINDEX = 1 to 5) \\[RW\\]: Bits 7:0 of the of the number of bytes received in the packet in the OUT endpoint {1-5} FIFO Valid only when USB_CSOL.OUTPKTRDY is set"]
    #[inline(always)]
    pub fn fifocnt_or_fifocntl(&self) -> FIFOCNT_OR_FIFOCNTL_R {
        FIFOCNT_OR_FIFOCNTL_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Indexed register: For USB_INDEX = 0: Number of received bytes in the endpoint 0 FIFO For USB_INDEX = 1-5: Number of received bytes in the OUT endpoint {1-5} FIFO (low byte)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt0_cntl](index.html) module"]
pub struct CNT0_CNTL_SPEC;
impl crate::RegisterSpec for CNT0_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt0_cntl::R](R) reader structure"]
impl crate::Readable for CNT0_CNTL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CNT0_CNTL to value 0"]
impl crate::Resettable for CNT0_CNTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
