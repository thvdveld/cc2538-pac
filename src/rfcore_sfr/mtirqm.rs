#[doc = "Register `MTIRQM` reader"]
pub type R = crate::R<MTIRQM_SPEC>;
#[doc = "Register `MTIRQM` writer"]
pub type W = crate::W<MTIRQM_SPEC>;
#[doc = "Field `MACTIMER_PERM` reader - Enables the MACTIMER_PER interrupt"]
pub type MACTIMER_PERM_R = crate::BitReader;
#[doc = "Field `MACTIMER_PERM` writer - Enables the MACTIMER_PER interrupt"]
pub type MACTIMER_PERM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_COMPARE1M` reader - Enables the MACTIMER_COMPARE1 interrupt"]
pub type MACTIMER_COMPARE1M_R = crate::BitReader;
#[doc = "Field `MACTIMER_COMPARE1M` writer - Enables the MACTIMER_COMPARE1 interrupt"]
pub type MACTIMER_COMPARE1M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_COMPARE2M` reader - Enables the MACTIMER_COMPARE2 interrupt"]
pub type MACTIMER_COMPARE2M_R = crate::BitReader;
#[doc = "Field `MACTIMER_COMPARE2M` writer - Enables the MACTIMER_COMPARE2 interrupt"]
pub type MACTIMER_COMPARE2M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_OVF_PERM` reader - Enables the MACTIMER_OVF_PER interrupt"]
pub type MACTIMER_OVF_PERM_R = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_PERM` writer - Enables the MACTIMER_OVF_PER interrupt"]
pub type MACTIMER_OVF_PERM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_OVF_COMPARE1M` reader - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
pub type MACTIMER_OVF_COMPARE1M_R = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_COMPARE1M` writer - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
pub type MACTIMER_OVF_COMPARE1M_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACTIMER_OVF_COMPARE2M` reader - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
pub type MACTIMER_OVF_COMPARE2M_R = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_COMPARE2M` writer - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
pub type MACTIMER_OVF_COMPARE2M_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the MACTIMER_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_perm(&self) -> MACTIMER_PERM_R {
        MACTIMER_PERM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the MACTIMER_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare1m(&self) -> MACTIMER_COMPARE1M_R {
        MACTIMER_COMPARE1M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the MACTIMER_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare2m(&self) -> MACTIMER_COMPARE2M_R {
        MACTIMER_COMPARE2M_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the MACTIMER_OVF_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_perm(&self) -> MACTIMER_OVF_PERM_R {
        MACTIMER_OVF_PERM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1m(&self) -> MACTIMER_OVF_COMPARE1M_R {
        MACTIMER_OVF_COMPARE1M_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2m(&self) -> MACTIMER_OVF_COMPARE2M_R {
        MACTIMER_OVF_COMPARE2M_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the MACTIMER_PER interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_perm(&mut self) -> MACTIMER_PERM_W<MTIRQM_SPEC> {
        MACTIMER_PERM_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the MACTIMER_COMPARE1 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_compare1m(&mut self) -> MACTIMER_COMPARE1M_W<MTIRQM_SPEC> {
        MACTIMER_COMPARE1M_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables the MACTIMER_COMPARE2 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_compare2m(&mut self) -> MACTIMER_COMPARE2M_W<MTIRQM_SPEC> {
        MACTIMER_COMPARE2M_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables the MACTIMER_OVF_PER interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_perm(&mut self) -> MACTIMER_OVF_PERM_W<MTIRQM_SPEC> {
        MACTIMER_OVF_PERM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_compare1m(&mut self) -> MACTIMER_OVF_COMPARE1M_W<MTIRQM_SPEC> {
        MACTIMER_OVF_COMPARE1M_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_compare2m(&mut self) -> MACTIMER_OVF_COMPARE2M_W<MTIRQM_SPEC> {
        MACTIMER_OVF_COMPARE2M_W::new(self, 5)
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
#[doc = "MAC Timer interrupt mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtirqm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtirqm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIRQM_SPEC;
impl crate::RegisterSpec for MTIRQM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtirqm::R`](R) reader structure"]
impl crate::Readable for MTIRQM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtirqm::W`](W) writer structure"]
impl crate::Writable for MTIRQM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTIRQM to value 0"]
impl crate::Resettable for MTIRQM_SPEC {
    const RESET_VALUE: u32 = 0;
}
