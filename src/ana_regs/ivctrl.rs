#[doc = "Register `IVCTRL` reader"]
pub type R = crate::R<IVCTRL_SPEC>;
#[doc = "Register `IVCTRL` writer"]
pub type W = crate::W<IVCTRL_SPEC>;
#[doc = "Field `PA_BIAS_CTRL` reader - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
pub type PA_BIAS_CTRL_R = crate::FieldReader;
#[doc = "Field `PA_BIAS_CTRL` writer - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
pub type PA_BIAS_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXMIX_DC_CTRL` reader - Controls DC bias in TXMIX"]
pub type TXMIX_DC_CTRL_R = crate::BitReader;
#[doc = "Field `TXMIX_DC_CTRL` writer - Controls DC bias in TXMIX"]
pub type TXMIX_DC_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LODIV_BIAS_CTRL` reader - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
pub type LODIV_BIAS_CTRL_R = crate::BitReader;
#[doc = "Field `LODIV_BIAS_CTRL` writer - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
pub type LODIV_BIAS_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_CURR_CTRL` reader - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
pub type DAC_CURR_CTRL_R = crate::FieldReader;
#[doc = "Field `DAC_CURR_CTRL` writer - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
pub type DAC_CURR_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
    #[inline(always)]
    pub fn pa_bias_ctrl(&self) -> PA_BIAS_CTRL_R {
        PA_BIAS_CTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Controls DC bias in TXMIX"]
    #[inline(always)]
    pub fn txmix_dc_ctrl(&self) -> TXMIX_DC_CTRL_R {
        TXMIX_DC_CTRL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
    #[inline(always)]
    pub fn lodiv_bias_ctrl(&self) -> LODIV_BIAS_CTRL_R {
        LODIV_BIAS_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
    #[inline(always)]
    pub fn dac_curr_ctrl(&self) -> DAC_CURR_CTRL_R {
        DAC_CURR_CTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
    #[inline(always)]
    #[must_use]
    pub fn pa_bias_ctrl(&mut self) -> PA_BIAS_CTRL_W<IVCTRL_SPEC> {
        PA_BIAS_CTRL_W::new(self, 0)
    }
    #[doc = "Bit 2 - Controls DC bias in TXMIX"]
    #[inline(always)]
    #[must_use]
    pub fn txmix_dc_ctrl(&mut self) -> TXMIX_DC_CTRL_W<IVCTRL_SPEC> {
        TXMIX_DC_CTRL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
    #[inline(always)]
    #[must_use]
    pub fn lodiv_bias_ctrl(&mut self) -> LODIV_BIAS_CTRL_W<IVCTRL_SPEC> {
        LODIV_BIAS_CTRL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
    #[inline(always)]
    #[must_use]
    pub fn dac_curr_ctrl(&mut self) -> DAC_CURR_CTRL_W<IVCTRL_SPEC> {
        DAC_CURR_CTRL_W::new(self, 4)
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
#[doc = "Analog control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IVCTRL_SPEC;
impl crate::RegisterSpec for IVCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivctrl::R`](R) reader structure"]
impl crate::Readable for IVCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ivctrl::W`](W) writer structure"]
impl crate::Writable for IVCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IVCTRL to value 0"]
impl crate::Resettable for IVCTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
