#[doc = "Register `ENASET` reader"]
pub type R = crate::R<EnasetSpec>;
#[doc = "Register `ENASET` writer"]
pub type W = crate::W<EnasetSpec>;
#[doc = "Field `SET` reader - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
pub type SetR = crate::FieldReader<u32>;
#[doc = "Field `SET` writer - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
pub type SetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
    #[inline(always)]
    pub fn set_(&mut self) -> SetW<EnasetSpec> {
        SetW::new(self, 0)
    }
}
#[doc = "DMA channel enable set Each bit of the ENASET register represents the corresponding uDMA channel. Setting a bit enables the corresponding uDMA channel. Reading the register returns the enable status of the channels. If a channel is enabled but the request mask is set (REQMASKSET), then the channel can be used for software-initiated transfers.\n\nYou can [`read`](crate::Reg::read) this register and get [`enaset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enaset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnasetSpec;
impl crate::RegisterSpec for EnasetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enaset::R`](R) reader structure"]
impl crate::Readable for EnasetSpec {}
#[doc = "`write(|w| ..)` method takes [`enaset::W`](W) writer structure"]
impl crate::Writable for EnasetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENASET to value 0"]
impl crate::Resettable for EnasetSpec {
    const RESET_VALUE: u32 = 0;
}
