#[doc = "Register `PRIOSET` reader"]
pub type R = crate::R<PriosetSpec>;
#[doc = "Register `PRIOSET` writer"]
pub type W = crate::W<PriosetSpec>;
#[doc = "Field `SET` reader - Channel \\[n\\]
priority set 0: uDMA channel \\[n\\]
is using the default priority level 1: uDMA channel \\[n\\]
is using a high priority level Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAPRIOCLR register."]
pub type SetR = crate::FieldReader<u32>;
#[doc = "Field `SET` writer - Channel \\[n\\]
priority set 0: uDMA channel \\[n\\]
is using the default priority level 1: uDMA channel \\[n\\]
is using a high priority level Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAPRIOCLR register."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
priority set 0: uDMA channel \\[n\\]
is using the default priority level 1: uDMA channel \\[n\\]
is using a high priority level Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAPRIOCLR register."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(self.bits)
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
    pub fn set_(&mut self) -> SetW<PriosetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "DMA channel priority set Each bit of the PRIOSET register represents the corresponding uDMA channel. Setting a bit configures the uDMA channel to have a high priority level. Reading the register returns the status of the channel priority mask.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prioset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prioset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PriosetSpec;
impl crate::RegisterSpec for PriosetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prioset::R`](R) reader structure"]
impl crate::Readable for PriosetSpec {}
#[doc = "`write(|w| ..)` method takes [`prioset::W`](W) writer structure"]
impl crate::Writable for PriosetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRIOSET to value 0"]
impl crate::Resettable for PriosetSpec {
    const RESET_VALUE: u32 = 0;
}
