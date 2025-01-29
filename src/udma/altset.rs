#[doc = "Register `ALTSET` reader"]
pub type R = crate::R<AltsetSpec>;
#[doc = "Register `ALTSET` writer"]
pub type W = crate::W<AltsetSpec>;
#[doc = "Field `SET` reader - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
pub type SetR = crate::FieldReader<u32>;
#[doc = "Field `SET` writer - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<AltsetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "DMA channel primary alternate set Each bit of the ALTSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to use the alternate control data structure. Reading the register returns the status of which control data structure is in use for the corresponding uDMA channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`altset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltsetSpec;
impl crate::RegisterSpec for AltsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altset::R`](R) reader structure"]
impl crate::Readable for AltsetSpec {}
#[doc = "`write(|w| ..)` method takes [`altset::W`](W) writer structure"]
impl crate::Writable for AltsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALTSET to value 0"]
impl crate::Resettable for AltsetSpec {
    const RESET_VALUE: u32 = 0;
}
