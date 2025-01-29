#[doc = "Register `WAITSTAT` reader"]
pub type R = crate::R<WaitstatSpec>;
#[doc = "Field `WAITREQ` reader - Channel \\[n\\]
wait status These bits provide the tchannel wait-on-request status. Bit 0 corresponds to channel 0. 1: The corresponding channel is waiting on a request. 0: The corresponding channel is not waiting on a request."]
pub type WaitreqR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
wait status These bits provide the tchannel wait-on-request status. Bit 0 corresponds to channel 0. 1: The corresponding channel is waiting on a request. 0: The corresponding channel is not waiting on a request."]
    #[inline(always)]
    pub fn waitreq(&self) -> WaitreqR {
        WaitreqR::new(self.bits)
    }
}
#[doc = "DMA channel wait-on-request status This read-only register indicates that the uDMA channel is waiting on a request. A peripheral can hold off the uDMA from performing a single request until the peripheral is ready for a burst request to enhance the uDMA performance. The use of this feature is dependent on the design of the peripheral and is not controllable by software in any way. This register cannot be read when the uDMA controller is in the reset state.\n\nYou can [`read`](crate::Reg::read) this register and get [`waitstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WaitstatSpec;
impl crate::RegisterSpec for WaitstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`waitstat::R`](R) reader structure"]
impl crate::Readable for WaitstatSpec {}
#[doc = "`reset()` method sets WAITSTAT to value 0"]
impl crate::Resettable for WaitstatSpec {
    const RESET_VALUE: u32 = 0;
}
