#[doc = "Register `PRIOCLR` writer"]
pub type W = crate::W<PrioclrSpec>;
#[doc = "Field `CLR` writer - Channel \\[n\\]
priority clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAPRIOSET register meaning that channel \\[n\\]
is using the default priority level."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
priority clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAPRIOSET register meaning that channel \\[n\\]
is using the default priority level."]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<PrioclrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "DMA channel priority clear Each bit of the DMAPRIOCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the PRIOSET register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prioclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrioclrSpec;
impl crate::RegisterSpec for PrioclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`prioclr::W`](W) writer structure"]
impl crate::Writable for PrioclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIOCLR to value 0"]
impl crate::Resettable for PrioclrSpec {
    const RESET_VALUE: u32 = 0;
}
