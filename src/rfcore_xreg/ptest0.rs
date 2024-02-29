#[doc = "Register `PTEST0` reader"]
pub type R = crate::R<Ptest0Spec>;
#[doc = "Register `PTEST0` writer"]
pub type W = crate::W<Ptest0Spec>;
#[doc = "Field `AAF_PD` reader - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
pub type AafPdR = crate::BitReader;
#[doc = "Field `AAF_PD` writer - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
pub type AafPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMIX_PD` reader - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
pub type TxmixPdR = crate::BitReader;
#[doc = "Field `TXMIX_PD` writer - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
pub type TxmixPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LNA_PD` reader - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
pub type LnaPdR = crate::FieldReader;
#[doc = "Field `LNA_PD` writer - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
pub type LnaPdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DAC_PD` reader - DAC power-down signal When PD_OVERRIDE = 1"]
pub type DacPdR = crate::BitReader;
#[doc = "Field `DAC_PD` writer - DAC power-down signal When PD_OVERRIDE = 1"]
pub type DacPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_PD` reader - ADC power-down signal When PD_OVERRIDE = 1"]
pub type AdcPdR = crate::BitReader;
#[doc = "Field `ADC_PD` writer - ADC power-down signal When PD_OVERRIDE = 1"]
pub type AdcPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHP_PD` reader - Charge pump power-down signal When PD_OVERRIDE = 1"]
pub type ChpPdR = crate::BitReader;
#[doc = "Field `CHP_PD` writer - Charge pump power-down signal When PD_OVERRIDE = 1"]
pub type ChpPdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRE_PD` reader - Prescaler power-down signal When PD_OVERRIDE = 1"]
pub type PrePdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn aaf_pd(&self) -> AafPdR {
        AafPdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn txmix_pd(&self) -> TxmixPdR {
        TxmixPdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn lna_pd(&self) -> LnaPdR {
        LnaPdR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - DAC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn dac_pd(&self) -> DacPdR {
        DacPdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn adc_pd(&self) -> AdcPdR {
        AdcPdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Charge pump power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn chp_pd(&self) -> ChpPdR {
        ChpPdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Prescaler power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    pub fn pre_pd(&self) -> PrePdR {
        PrePdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Antialiasing filter power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn aaf_pd(&mut self) -> AafPdW<Ptest0Spec> {
        AafPdW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit mixer power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn txmix_pd(&mut self) -> TxmixPdW<Ptest0Spec> {
        TxmixPdW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Low-noise amplifier power-down signal Defines LNA/mixer power-down modes: 00: Power up 01: LNA off, mixer/regulator on 10: LNA/mixer off, regulator on 11: PD When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn lna_pd(&mut self) -> LnaPdW<Ptest0Spec> {
        LnaPdW::new(self, 2)
    }
    #[doc = "Bit 4 - DAC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn dac_pd(&mut self) -> DacPdW<Ptest0Spec> {
        DacPdW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pd(&mut self) -> AdcPdW<Ptest0Spec> {
        AdcPdW::new(self, 5)
    }
    #[doc = "Bit 6 - Charge pump power-down signal When PD_OVERRIDE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn chp_pd(&mut self) -> ChpPdW<Ptest0Spec> {
        ChpPdW::new(self, 6)
    }
}
#[doc = "Override power-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptest0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptest0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ptest0Spec;
impl crate::RegisterSpec for Ptest0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptest0::R`](R) reader structure"]
impl crate::Readable for Ptest0Spec {}
#[doc = "`write(|w| ..)` method takes [`ptest0::W`](W) writer structure"]
impl crate::Writable for Ptest0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTEST0 to value 0"]
impl crate::Resettable for Ptest0Spec {
    const RESET_VALUE: u32 = 0;
}
