#[doc = "Register `PTEST1` reader"]
pub struct R(crate::R<PTEST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTEST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTEST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTEST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTEST1` writer"]
pub struct W(crate::W<PTEST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTEST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PTEST1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTEST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LODIV_PD` reader - LO power-down signal When PD_OVERRIDE = 1"]
pub type LODIV_PD_R = crate::BitReader<bool>;
#[doc = "Field `LODIV_PD` writer - LO power-down signal When PD_OVERRIDE = 1"]
pub type LODIV_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTEST1_SPEC, bool, O>;
#[doc = "Field `VCO_PD` reader - VCO power-down signal When PD_OVERRIDE = 1"]
pub type VCO_PD_R = crate::BitReader<bool>;
#[doc = "Field `VCO_PD` writer - VCO power-down signal When PD_OVERRIDE = 1"]
pub type VCO_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTEST1_SPEC, bool, O>;
#[doc = "Field `PA_PD` reader - Power amplifier power-down signal When PD_OVERRIDE = 1"]
pub type PA_PD_R = crate::BitReader<bool>;
#[doc = "Field `PA_PD` writer - Power amplifier power-down signal When PD_OVERRIDE = 1"]
pub type PA_PD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTEST1_SPEC, bool, O>;
#[doc = "Field `PD_OVERRIDE` reader - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
pub type PD_OVERRIDE_R = crate::BitReader<bool>;
#[doc = "Field `PD_OVERRIDE` writer - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
pub type PD_OVERRIDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTEST1_SPEC, bool, O>;
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
    pub fn lodiv_pd(&mut self) -> LODIV_PD_W<0> {
        LODIV_PD_W::new(self)
    }
    #[doc = "Bit 1 - VCO power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn vco_pd(&mut self) -> VCO_PD_W<1> {
        VCO_PD_W::new(self)
    }
    #[doc = "Bit 2 - Power amplifier power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pa_pd(&mut self) -> PA_PD_W<2> {
        PA_PD_W::new(self)
    }
    #[doc = "Bit 3 - Override enabling and disabling of various modules (for debug and testing only) It is impossible to override hard-coded BIAS_PD\\[1:0\\]
depenancy."]
    #[inline(always)]
    pub fn pd_override(&mut self) -> PD_OVERRIDE_W<3> {
        PD_OVERRIDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Override power-down register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptest1](index.html) module"]
pub struct PTEST1_SPEC;
impl crate::RegisterSpec for PTEST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptest1::R](R) reader structure"]
impl crate::Readable for PTEST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptest1::W](W) writer structure"]
impl crate::Writable for PTEST1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PTEST1 to value 0"]
impl crate::Resettable for PTEST1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
