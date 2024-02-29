#[doc = "Register `REQMASKSET` reader"]
pub type R = crate::R<ReqmasksetSpec>;
#[doc = "Register `REQMASKSET` writer"]
pub type W = crate::W<ReqmasksetSpec>;
#[doc = "Field `SET` reader - Channel \\[n\\]
request mask set 0: The peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers 1: The peripheral associated with channel \\[n\\]
is not able to request uDMA transfers. Channel \\[n\\]
may be used for software-initiated transfers. Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAREQMASKCLR register."]
pub type SetR = crate::FieldReader<u32>;
#[doc = "Field `SET` writer - Channel \\[n\\]
request mask set 0: The peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers 1: The peripheral associated with channel \\[n\\]
is not able to request uDMA transfers. Channel \\[n\\]
may be used for software-initiated transfers. Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAREQMASKCLR register."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
request mask set 0: The peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers 1: The peripheral associated with channel \\[n\\]
is not able to request uDMA transfers. Channel \\[n\\]
may be used for software-initiated transfers. Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAREQMASKCLR register."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
request mask set 0: The peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers 1: The peripheral associated with channel \\[n\\]
is not able to request uDMA transfers. Channel \\[n\\]
may be used for software-initiated transfers. Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAREQMASKCLR register."]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<ReqmasksetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "DMA channel request mask set Each bit of the REQMASKSET register represents the corresponding uDMA channel. Setting a bit disables uDMA requests for the channel. Reading the register returns the request mask status. When a uDMA channel request is masked, that means the peripheral can no longer request uDMA transfers. The channel can then be used for software-initiated transfers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqmaskset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqmaskset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqmasksetSpec;
impl crate::RegisterSpec for ReqmasksetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqmaskset::R`](R) reader structure"]
impl crate::Readable for ReqmasksetSpec {}
#[doc = "`write(|w| ..)` method takes [`reqmaskset::W`](W) writer structure"]
impl crate::Writable for ReqmasksetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REQMASKSET to value 0"]
impl crate::Resettable for ReqmasksetSpec {
    const RESET_VALUE: u32 = 0;
}
