#[doc = "Register `REQMASKCLR` writer"]
pub type W = crate::W<ReqmaskclrSpec>;
#[doc = "Field `CLR` writer - Channel \\[n\\]
request mask clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAREQMASKSET register meaning that the peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
request mask clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAREQMASKSET register meaning that the peripheral associated with channel \\[n\\]
is enabled to request uDMA transfers."]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<ReqmaskclrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "DMA channel request mask clear Each bit of the REQMASKCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the REQMASKSET register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqmaskclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqmaskclrSpec;
impl crate::RegisterSpec for ReqmaskclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`reqmaskclr::W`](W) writer structure"]
impl crate::Writable for ReqmaskclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REQMASKCLR to value 0"]
impl crate::Resettable for ReqmaskclrSpec {
    const RESET_VALUE: u32 = 0;
}
