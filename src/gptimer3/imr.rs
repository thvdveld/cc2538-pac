#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `TATOIM` reader - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TatoimR = crate::BitReader;
#[doc = "Field `TATOIM` writer - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TatoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAMIM` reader - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CamimR = crate::BitReader;
#[doc = "Field `CAMIM` writer - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CamimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAEIM` reader - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CaeimR = crate::BitReader;
#[doc = "Field `CAEIM` writer - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CaeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMIM` reader - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TamimR = crate::BitReader;
#[doc = "Field `TAMIM` writer - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TamimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBTOIM` reader - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TbtoimR = crate::BitReader;
#[doc = "Field `TBTOIM` writer - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TbtoimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBMIM` reader - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CbmimR = crate::BitReader;
#[doc = "Field `CBMIM` writer - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CbmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBEIM` reader - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CbeimR = crate::BitReader;
#[doc = "Field `CBEIM` writer - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CbeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBMIM` reader - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TbmimR = crate::BitReader;
#[doc = "Field `TBMIM` writer - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TbmimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tatoim(&self) -> TatoimR {
        TatoimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn camim(&self) -> CamimR {
        CamimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn caeim(&self) -> CaeimR {
        CaeimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tamim(&self) -> TamimR {
        TamimR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbtoim(&self) -> TbtoimR {
        TbtoimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbmim(&self) -> CbmimR {
        CbmimR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbeim(&self) -> CbeimR {
        CbeimR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbmim(&self) -> TbmimR {
        TbmimR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tatoim(&mut self) -> TatoimW<ImrSpec> {
        TatoimW::new(self, 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn camim(&mut self) -> CamimW<ImrSpec> {
        CamimW::new(self, 1)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn caeim(&mut self) -> CaeimW<ImrSpec> {
        CaeimW::new(self, 2)
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tamim(&mut self) -> TamimW<ImrSpec> {
        TamimW::new(self, 4)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbtoim(&mut self) -> TbtoimW<ImrSpec> {
        TbtoimW::new(self, 8)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbmim(&mut self) -> CbmimW<ImrSpec> {
        CbmimW::new(self, 9)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbeim(&mut self) -> CbeimW<ImrSpec> {
        CbeimW::new(self, 10)
    }
    #[doc = "Bit 11 - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbmim(&mut self) -> TbmimW<ImrSpec> {
        TbmimW::new(self, 11)
    }
}
#[doc = "GPTM interrupt mask This register allows software to enable and disable GPTM controller-level interrupts. Setting a bit enables the corresponding interrupt, while clearing a bit disables it.\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {
    const RESET_VALUE: u32 = 0;
}
