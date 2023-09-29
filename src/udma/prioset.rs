#[doc = "Register `PRIOSET` reader"]
pub type R = crate::R<PRIOSET_SPEC>;
#[doc = "Register `PRIOSET` writer"]
pub type W = crate::W<PRIOSET_SPEC>;
#[doc = "Field `SET` reader - Channel \\[n\\]
priority set 0: uDMA channel \\[n\\]
is using the default priority level 1: uDMA channel \\[n\\]
is using a high priority level Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAPRIOCLR register."]
pub type SET_R = crate::FieldReader<u32>;
#[doc = "Field `SET` writer - Channel \\[n\\]
priority set 0: uDMA channel \\[n\\]
is using the default priority level 1: uDMA channel \\[n\\]
is using a high priority level Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAPRIOCLR register."]
pub type SET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
priority set 0: uDMA channel \\[n\\]
is using the default priority level 1: uDMA channel \\[n\\]
is using a high priority level Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAPRIOCLR register."]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
priority set 0: uDMA channel \\[n\\]
is using the default priority level 1: uDMA channel \\[n\\]
is using a high priority level Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAPRIOCLR register."]
    #[inline(always)]
    #[must_use]
    pub fn set(&mut self) -> SET_W<PRIOSET_SPEC, 0> {
        SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel priority set Each bit of the PRIOSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to have a high priority level. Reading the register returns the status of the channel priority mask.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prioset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prioset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRIOSET_SPEC;
impl crate::RegisterSpec for PRIOSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prioset::R`](R) reader structure"]
impl crate::Readable for PRIOSET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prioset::W`](W) writer structure"]
impl crate::Writable for PRIOSET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIOSET to value 0"]
impl crate::Resettable for PRIOSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
