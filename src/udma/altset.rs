#[doc = "Register `ALTSET` reader"]
pub type R = crate::R<ALTSET_SPEC>;
#[doc = "Register `ALTSET` writer"]
pub type W = crate::W<ALTSET_SPEC>;
#[doc = "Field `SET` reader - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
pub type SET_R = crate::FieldReader<u32>;
#[doc = "Field `SET` writer - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
pub type SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
alternate set 0: uDMA channel \\[n\\]
is using the primary control structure 1: uDMA channel \\[n\\]
is using the alternate control structure Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAALTCLR register. Note: For Ping-Pong and Scatter-Gather cycle types, the uDMA controller automatically sets these bits to select the alternate channel control data structure."]
    #[inline(always)]
    #[must_use]
    pub fn set(&mut self) -> SET_W<ALTSET_SPEC, 0> {
        SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel primary alternate set Each bit of the ALTSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to use the alternate control data structure. Reading the register returns the status of which control data structure is in use for the corresponding uDMA channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALTSET_SPEC;
impl crate::RegisterSpec for ALTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altset::R`](R) reader structure"]
impl crate::Readable for ALTSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`altset::W`](W) writer structure"]
impl crate::Writable for ALTSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ALTSET to value 0"]
impl crate::Resettable for ALTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
