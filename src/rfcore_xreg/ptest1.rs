#[doc = "Register `PTEST1` reader"]
pub type R = crate::R<Ptest1Spec>;
#[doc = "Register `PTEST1` writer"]
pub type W = crate::W<Ptest1Spec>;
#[doc = "Field `LODIV_PD` reader - LO power-down signal When PD_OVERRIDE = 1"]
pub type LodivPdR = crate::BitReader;
#[doc = "Field `LODIV_PD` writer - LO power-down signal When PD_OVERRIDE = 1"]
pub type LodivPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCO_PD` reader - VCO power-down signal When PD_OVERRIDE = 1"]
pub type VcoPdR = crate::BitReader;
#[doc = "Field `VCO_PD` writer - VCO power-down signal When PD_OVERRIDE = 1"]
pub type VcoPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA_PD` reader - Power amplifier power-down signal When PD_OVERRIDE = 1"]
pub type PaPdR = crate::BitReader;
#[doc = "Field `PA_PD` writer - Power amplifier power-down signal When PD_OVERRIDE = 1"]
pub type PaPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD_OVERRIDE` reader - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
pub type PdOverrideR = crate::BitReader;
#[doc = "Field `PD_OVERRIDE` writer - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
pub type PdOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lodiv_pd(&self) -> LodivPdR {
        LodivPdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VCO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn vco_pd(&self) -> VcoPdR {
        VcoPdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power amplifier power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pa_pd(&self) -> PaPdR {
        PaPdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
    #[inline(always)]
    pub fn pd_override(&self) -> PdOverrideR {
        PdOverrideR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn lodiv_pd(&mut self) -> LodivPdW<Ptest1Spec> {
        LodivPdW::new(self, 0)
    }
    #[doc = "Bit 1 - VCO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn vco_pd(&mut self) -> VcoPdW<Ptest1Spec> {
        VcoPdW::new(self, 1)
    }
    #[doc = "Bit 2 - Power amplifier power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn pa_pd(&mut self) -> PaPdW<Ptest1Spec> {
        PaPdW::new(self, 2)
    }
    #[doc = "Bit 3 - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
    #[inline(always)]
    #[must_use]
    pub fn pd_override(&mut self) -> PdOverrideW<Ptest1Spec> {
        PdOverrideW::new(self, 3)
    }
}
#[doc = "Override power-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptest1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptest1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ptest1Spec;
impl crate::RegisterSpec for Ptest1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptest1::R`](R) reader structure"]
impl crate::Readable for Ptest1Spec {}
#[doc = "`write(|w| ..)` method takes [`ptest1::W`](W) writer structure"]
impl crate::Writable for Ptest1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTEST1 to value 0"]
impl crate::Resettable for Ptest1Spec {
    const RESET_VALUE: u32 = 0;
}
