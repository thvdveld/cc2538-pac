#[doc = "Register `PTEST0` reader"]
pub type R = crate::R<PTEST0_SPEC>;
#[doc = "Register `PTEST0` writer"]
pub type W = crate::W<PTEST0_SPEC>;
#[doc = "Field `AAF_PD` reader - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
pub type AAF_PD_R = crate::BitReader;
#[doc = "Field `AAF_PD` writer - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
pub type AAF_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `TXMIX_PD` reader - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
pub type TXMIX_PD_R = crate::BitReader;
#[doc = "Field `TXMIX_PD` writer - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
pub type TXMIX_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `LNA_PD` reader - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
pub type LNA_PD_R = crate::FieldReader;
#[doc = "Field `LNA_PD` writer - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
pub type LNA_PD_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `DAC_PD` reader - DAC power-down signal When PD_OVERRIDE = 1"]
pub type DAC_PD_R = crate::BitReader;
#[doc = "Field `DAC_PD` writer - DAC power-down signal When PD_OVERRIDE = 1"]
pub type DAC_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `ADC_PD` reader - ADC power-down signal When PD_OVERRIDE = 1"]
pub type ADC_PD_R = crate::BitReader;
#[doc = "Field `ADC_PD` writer - ADC power-down signal When PD_OVERRIDE = 1"]
pub type ADC_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CHP_PD` reader - Charge pump power-down signal When PD_OVERRIDE = 1"]
pub type CHP_PD_R = crate::BitReader;
#[doc = "Field `CHP_PD` writer - Charge pump power-down signal When PD_OVERRIDE = 1"]
pub type CHP_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PRE_PD` reader - Prescaler power-down signal When PD_OVERRIDE = 1"]
pub type PRE_PD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn aaf_pd(&self) -> AAF_PD_R {
        AAF_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn txmix_pd(&self) -> TXMIX_PD_R {
        TXMIX_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lna_pd(&self) -> LNA_PD_R {
        LNA_PD_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - DAC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn dac_pd(&self) -> DAC_PD_R {
        DAC_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn adc_pd(&self) -> ADC_PD_R {
        ADC_PD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Charge pump power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn chp_pd(&self) -> CHP_PD_R {
        CHP_PD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Prescaler power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pre_pd(&self) -> PRE_PD_R {
        PRE_PD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn aaf_pd(&mut self) -> AAF_PD_W<PTEST0_SPEC, 0> {
        AAF_PD_W::new(self)
    }
    #[doc = "Bit 1 - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn txmix_pd(&mut self) -> TXMIX_PD_W<PTEST0_SPEC, 1> {
        TXMIX_PD_W::new(self)
    }
    #[doc = "Bits 2:3 - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn lna_pd(&mut self) -> LNA_PD_W<PTEST0_SPEC, 2> {
        LNA_PD_W::new(self)
    }
    #[doc = "Bit 4 - DAC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn dac_pd(&mut self) -> DAC_PD_W<PTEST0_SPEC, 4> {
        DAC_PD_W::new(self)
    }
    #[doc = "Bit 5 - ADC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pd(&mut self) -> ADC_PD_W<PTEST0_SPEC, 5> {
        ADC_PD_W::new(self)
    }
    #[doc = "Bit 6 - Charge pump power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn chp_pd(&mut self) -> CHP_PD_W<PTEST0_SPEC, 6> {
        CHP_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Override power-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptest0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptest0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTEST0_SPEC;
impl crate::RegisterSpec for PTEST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptest0::R`](R) reader structure"]
impl crate::Readable for PTEST0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptest0::W`](W) writer structure"]
impl crate::Writable for PTEST0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTEST0 to value 0"]
impl crate::Resettable for PTEST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
