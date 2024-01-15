#[doc = "Register `CHIS` reader"]
pub type R = crate::R<CHIS_SPEC>;
#[doc = "Register `CHIS` writer"]
pub type W = crate::W<CHIS_SPEC>;
#[doc = "Field `CHIS` reader - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
pub type CHIS_R = crate::FieldReader<u32>;
#[doc = "Field `CHIS` writer - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
pub type CHIS_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    pub fn chis(&self) -> CHIS_R {
        CHIS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
interrupt status 0: The corresponding uDMA channel has not caused an interrupt. 1: The corresponding uDMA channel has caused an interrupt. This bit is cleared by writing 1 to it."]
    #[inline(always)]
    #[must_use]
    pub fn chis(&mut self) -> CHIS_W<CHIS_SPEC> {
        CHIS_W::new(self, 0)
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
#[doc = "DMA channel interrupt status Each bit of the CHIS register represents the corresponding uDMA channel. A bit is set when that uDMA channel causes a completion interrupt. The bits are cleared by writing 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHIS_SPEC;
impl crate::RegisterSpec for CHIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chis::R`](R) reader structure"]
impl crate::Readable for CHIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chis::W`](W) writer structure"]
impl crate::Writable for CHIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHIS to value 0"]
impl crate::Resettable for CHIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
