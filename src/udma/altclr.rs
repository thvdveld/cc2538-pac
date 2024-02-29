#[doc = "Register `ALTCLR` writer"]
pub type W = crate::W<AltclrSpec>;
#[doc = "Field `CLR` writer - Channel \\[n\\]
alternate clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAALTSET register meaning that channel \\[n\\]
is using the primary control structure. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
alternate clear 0: No effect 1: Setting a bit clears the corresponding SET\\[n\\]
bit in the DMAALTSET register meaning that channel \\[n\\]
is using the primary control structure. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> ClrW<AltclrSpec> {
        ClrW::new(self, 0)
    }
}
#[doc = "DMA channel primary alternate clear Each bit of the ALTCLR register represents the corresponding uDMA channel. Setting a bit clears the corresponding SET\\[n\\]
bit in the ALTSET register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltclrSpec;
impl crate::RegisterSpec for AltclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`altclr::W`](W) writer structure"]
impl crate::Writable for AltclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALTCLR to value 0"]
impl crate::Resettable for AltclrSpec {
    const RESET_VALUE: u32 = 0;
}
