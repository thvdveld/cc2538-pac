#[doc = "Register `WAITSTAT` reader"]
pub struct R(crate::R<WAITSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAITSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAITSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAITSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WAITREQ` reader - Channel \\[n\\]
wait status These bits provide the tchannel wait-on-request status. Bit 0 corresponds to channel 0. 1: The corresponding channel is waiting on a request. 0: The corresponding channel is not waiting on a request."]
pub struct WAITREQ_R(crate::FieldReader<u32, u32>);
impl WAITREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        WAITREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WAITREQ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
wait status These bits provide the tchannel wait-on-request status. Bit 0 corresponds to channel 0. 1: The corresponding channel is waiting on a request. 0: The corresponding channel is not waiting on a request."]
    #[inline(always)]
    pub fn waitreq(&self) -> WAITREQ_R {
        WAITREQ_R::new(self.bits as u32)
    }
}
#[doc = "DMA channel wait-on-request status This read-only register indicates that the uDMA channel is waiting on a request. A peripheral can hold off the uDMA from performing a single request until the peripheral is ready for a burst request to enhance the uDMA performance. The use of this feature is dependent on the design of the peripheral and is not controllable by software in any way. This register cannot be read when the uDMA controller is in the reset state.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waitstat](index.html) module"]
pub struct WAITSTAT_SPEC;
impl crate::RegisterSpec for WAITSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [waitstat::R](R) reader structure"]
impl crate::Readable for WAITSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WAITSTAT to value 0"]
impl crate::Resettable for WAITSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
