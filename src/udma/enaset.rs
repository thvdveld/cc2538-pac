#[doc = "Register `ENASET` reader"]
pub type R = crate::R<ENASET_SPEC>;
#[doc = "Register `ENASET` writer"]
pub type W = crate::W<ENASET_SPEC>;
#[doc = "Field `SET` reader - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
pub type SET_R = crate::FieldReader<u32>;
#[doc = "Field `SET` writer - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
pub type SET_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
enable set 0: uDMA channel \\[n\\]
is disabled 1: uDMA channel \\[n\\]
is enabled Bit 0 corresponds to channel 0. A bit can only be cleared by setting the corresponding CLR\\[n\\]
bit in the DMAENACLR register."]
    #[inline(always)]
    #[must_use]
    pub fn set(&mut self) -> SET_W<ENASET_SPEC> {
        SET_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel enable set Each bit of the ENASET register represents the corresponding uDMA channel. Setting a bit enables the corresponding uDMA channel. Reading the register returns the enable status of the channels. If a channel is enabled but the request mask is set (REQMASKSET), then the channel can be used for software-initiated transfers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enaset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enaset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENASET_SPEC;
impl crate::RegisterSpec for ENASET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enaset::R`](R) reader structure"]
impl crate::Readable for ENASET_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enaset::W`](W) writer structure"]
impl crate::Writable for ENASET_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENASET to value 0"]
impl crate::Resettable for ENASET_SPEC {
    const RESET_VALUE: u32 = 0;
}
