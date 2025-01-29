#[doc = "Register `SWREQ` writer"]
pub type W = crate::W<SwreqSpec>;
#[doc = "Field `SWREQ` writer - Channel \\[n\\]
software request These bits generate software requests. Bit 0 corresponds to channel 0. 1: Generate a software request for the corresponding channel 0: No request generated These bits are automatically cleared when the software request has been completed."]
pub type SwreqW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
software request These bits generate software requests. Bit 0 corresponds to channel 0. 1: Generate a software request for the corresponding channel 0: No request generated These bits are automatically cleared when the software request has been completed."]
    #[inline(always)]
    pub fn swreq(&mut self) -> SwreqW<SwreqSpec> {
        SwreqW::new(self, 0)
    }
}
#[doc = "DMA channel software request Each bit of the SWREQ register represents the corresponding uDMA channel. Setting a bit generates a request for the specified uDMA channel.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreq::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwreqSpec;
impl crate::RegisterSpec for SwreqSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swreq::W`](W) writer structure"]
impl crate::Writable for SwreqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREQ to value 0"]
impl crate::Resettable for SwreqSpec {
    const RESET_VALUE: u32 = 0;
}
