#[doc = "Register `USEBURSTCLR` writer"]
pub type W = crate::W<UseburstclrSpec>;
#[doc = "Field `CLR` writer - Channel \\[n\\]
useburst clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAUSEBURSTSET register meaning that uDMA channel \\[n\\]
responds to single and burst requests."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
useburst clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAUSEBURSTSET register meaning that uDMA channel \\[n\\]
responds to single and burst requests."]
    #[inline(always)]
    pub fn clr(&mut self) -> ClrW<UseburstclrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "DMA channel useburst clear Each bit of the USEBURSTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the USEBURSTSET register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`useburstclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UseburstclrSpec;
impl crate::RegisterSpec for UseburstclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`useburstclr::W`](W) writer structure"]
impl crate::Writable for UseburstclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USEBURSTCLR to value 0"]
impl crate::Resettable for UseburstclrSpec {
    const RESET_VALUE: u32 = 0;
}
