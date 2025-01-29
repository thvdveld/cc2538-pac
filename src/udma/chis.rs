#[doc = "Register `CHIS` reader"]
pub type R = crate::R<ChisSpec>;
#[doc = "Register `CHIS` writer"]
pub type W = crate::W<ChisSpec>;
#[doc = "Field `CHIS` reader - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
pub type ChisR = crate::FieldReader<u32>;
#[doc = "Field `CHIS` writer - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
pub type ChisW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn chis(&self) -> ChisR {
        ChisR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn chis(&mut self) -> ChisW<ChisSpec> {
        ChisW::new(self, 0)
    }
}
#[doc = "DMA channel interrupt status Each bit of the CHIS register represents the corresponding uDMA channel. A bit is set when that uDMA channel causes a completion interrupt. The bits are cleared by writing 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`chis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChisSpec;
impl crate::RegisterSpec for ChisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chis::R`](R) reader structure"]
impl crate::Readable for ChisSpec {}
#[doc = "`write(|w| ..)` method takes [`chis::W`](W) writer structure"]
impl crate::Writable for ChisSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHIS to value 0"]
impl crate::Resettable for ChisSpec {
    const RESET_VALUE: u32 = 0;
}
