#[doc = "Register `IMR` reader"]
pub type R = crate::R<IMR_SPEC>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<IMR_SPEC>;
#[doc = "Field `TATOIM` reader - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TATOIM_R = crate::BitReader;
#[doc = "Field `TATOIM` writer - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TATOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAMIM` reader - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CAMIM_R = crate::BitReader;
#[doc = "Field `CAMIM` writer - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CAMIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAEIM` reader - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CAEIM_R = crate::BitReader;
#[doc = "Field `CAEIM` writer - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CAEIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMIM` reader - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TAMIM_R = crate::BitReader;
#[doc = "Field `TAMIM` writer - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TAMIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBTOIM` reader - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TBTOIM_R = crate::BitReader;
#[doc = "Field `TBTOIM` writer - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TBTOIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBMIM` reader - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CBMIM_R = crate::BitReader;
#[doc = "Field `CBMIM` writer - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CBMIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBEIM` reader - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CBEIM_R = crate::BitReader;
#[doc = "Field `CBEIM` writer - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type CBEIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBMIM` reader - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TBMIM_R = crate::BitReader;
#[doc = "Field `TBMIM` writer - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
pub type TBMIM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tatoim(&self) -> TATOIM_R {
        TATOIM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn camim(&self) -> CAMIM_R {
        CAMIM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn caeim(&self) -> CAEIM_R {
        CAEIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tamim(&self) -> TAMIM_R {
        TAMIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbtoim(&self) -> TBTOIM_R {
        TBTOIM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbmim(&self) -> CBMIM_R {
        CBMIM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn cbeim(&self) -> CBEIM_R {
        CBEIM_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    pub fn tbmim(&self) -> TBMIM_R {
        TBMIM_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tatoim(&mut self) -> TATOIM_W<IMR_SPEC> {
        TATOIM_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn camim(&mut self) -> CAMIM_W<IMR_SPEC> {
        CAMIM_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn caeim(&mut self) -> CAEIM_W<IMR_SPEC> {
        CAEIM_W::new(self, 2)
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tamim(&mut self) -> TAMIM_W<IMR_SPEC> {
        TAMIM_W::new(self, 4)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tbtoim(&mut self) -> TBTOIM_W<IMR_SPEC> {
        TBTOIM_W::new(self, 8)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cbmim(&mut self) -> CBMIM_W<IMR_SPEC> {
        CBMIM_W::new(self, 9)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cbeim(&mut self) -> CBEIM_W<IMR_SPEC> {
        CBEIM_W::new(self, 10)
    }
    #[doc = "Bit 11 - GPTM Timer B match interrupt mask 0: Interrupt is disabled. 1: Interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tbmim(&mut self) -> TBMIM_W<IMR_SPEC> {
        TBMIM_W::new(self, 11)
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
#[doc = "GPTM interrupt mask This register allows software to enable and disable GPTM controller-level interrupts. Setting a bit enables the corresponding interrupt, while clearing a bit disables it.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for IMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for IMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: u32 = 0;
}
