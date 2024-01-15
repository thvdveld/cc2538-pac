#[doc = "Register `PTEST1` reader"]
pub type R = crate::R<PTEST1_SPEC>;
#[doc = "Register `PTEST1` writer"]
pub type W = crate::W<PTEST1_SPEC>;
#[doc = "Field `LODIV_PD` reader - LO power-down signal When PD_OVERRIDE = 1"]
pub type LODIV_PD_R = crate::BitReader;
#[doc = "Field `LODIV_PD` writer - LO power-down signal When PD_OVERRIDE = 1"]
pub type LODIV_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCO_PD` reader - VCO power-down signal When PD_OVERRIDE = 1"]
pub type VCO_PD_R = crate::BitReader;
#[doc = "Field `VCO_PD` writer - VCO power-down signal When PD_OVERRIDE = 1"]
pub type VCO_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA_PD` reader - Power amplifier power-down signal When PD_OVERRIDE = 1"]
pub type PA_PD_R = crate::BitReader;
#[doc = "Field `PA_PD` writer - Power amplifier power-down signal When PD_OVERRIDE = 1"]
pub type PA_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD_OVERRIDE` reader - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
pub type PD_OVERRIDE_R = crate::BitReader;
#[doc = "Field `PD_OVERRIDE` writer - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
pub type PD_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lodiv_pd(&self) -> LODIV_PD_R {
        LODIV_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VCO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn vco_pd(&self) -> VCO_PD_R {
        VCO_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power amplifier power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pa_pd(&self) -> PA_PD_R {
        PA_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
    #[inline(always)]
    pub fn pd_override(&self) -> PD_OVERRIDE_R {
        PD_OVERRIDE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn lodiv_pd(&mut self) -> LODIV_PD_W<PTEST1_SPEC> {
        LODIV_PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - VCO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn vco_pd(&mut self) -> VCO_PD_W<PTEST1_SPEC> {
        VCO_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Power amplifier power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pd(&mut self) -> PA_PD_W<PTEST1_SPEC> {
        PA_PD_W::new(self, 2)
    }
    #[doc = "Bit 3 - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
    #[inline(always)]
    #[must_use]
    pub fn pd_override(&mut self) -> PD_OVERRIDE_W<PTEST1_SPEC> {
        PD_OVERRIDE_W::new(self, 3)
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
#[doc = "Override power-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptest1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptest1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTEST1_SPEC;
impl crate::RegisterSpec for PTEST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptest1::R`](R) reader structure"]
impl crate::Readable for PTEST1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptest1::W`](W) writer structure"]
impl crate::Writable for PTEST1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTEST1 to value 0"]
impl crate::Resettable for PTEST1_SPEC {
    const RESET_VALUE: u32 = 0;
}
