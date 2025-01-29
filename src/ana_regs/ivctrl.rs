#[doc = "Register `IVCTRL` reader"]
pub type R = crate::R<IvctrlSpec>;
#[doc = "Register `IVCTRL` writer"]
pub type W = crate::W<IvctrlSpec>;
#[doc = "Field `PA_BIAS_CTRL` reader - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
pub type PaBiasCtrlR = crate::FieldReader;
#[doc = "Field `PA_BIAS_CTRL` writer - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
pub type PaBiasCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXMIX_DC_CTRL` reader - Controls DC bias in TXMIX"]
pub type TxmixDcCtrlR = crate::BitReader;
#[doc = "Field `TXMIX_DC_CTRL` writer - Controls DC bias in TXMIX"]
pub type TxmixDcCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LODIV_BIAS_CTRL` reader - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
pub type LodivBiasCtrlR = crate::BitReader;
#[doc = "Field `LODIV_BIAS_CTRL` writer - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
pub type LodivBiasCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_CURR_CTRL` reader - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
pub type DacCurrCtrlR = crate::FieldReader;
#[doc = "Field `DAC_CURR_CTRL` writer - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
pub type DacCurrCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
    #[inline(always)]
    pub fn pa_bias_ctrl(&self) -> PaBiasCtrlR {
        PaBiasCtrlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Controls DC bias in TXMIX"]
    #[inline(always)]
    pub fn txmix_dc_ctrl(&self) -> TxmixDcCtrlR {
        TxmixDcCtrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
    #[inline(always)]
    pub fn lodiv_bias_ctrl(&self) -> LodivBiasCtrlR {
        LodivBiasCtrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
    #[inline(always)]
    pub fn dac_curr_ctrl(&self) -> DacCurrCtrlR {
        DacCurrCtrlR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controls bias current to PA 00: IREF bias 01: IREF and IVREF bias (CC2530 mode) 10: PTAT bias 11: Increased PTAT slope bias"]
    #[inline(always)]
    pub fn pa_bias_ctrl(&mut self) -> PaBiasCtrlW<IvctrlSpec> {
        PaBiasCtrlW::new(self, 0)
    }
    #[doc = "Bit 2 - Controls DC bias in TXMIX"]
    #[inline(always)]
    pub fn txmix_dc_ctrl(&mut self) -> TxmixDcCtrlW<IvctrlSpec> {
        TxmixDcCtrlW::new(self, 2)
    }
    #[doc = "Bit 3 - Controls bias current to LODIV 1: PTAT bias 0: IVREF bias"]
    #[inline(always)]
    pub fn lodiv_bias_ctrl(&mut self) -> LodivBiasCtrlW<IvctrlSpec> {
        LodivBiasCtrlW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Controls bias current to DAC 00: 100% IVREF, 0% IREF bias 01: 60% IVREF, 40% IREF bias 10: 40% IVREF, 60% IREF bias 11: 0% IVREF, 100% IREF bias"]
    #[inline(always)]
    pub fn dac_curr_ctrl(&mut self) -> DacCurrCtrlW<IvctrlSpec> {
        DacCurrCtrlW::new(self, 4)
    }
}
#[doc = "Analog control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ivctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ivctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IvctrlSpec;
impl crate::RegisterSpec for IvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ivctrl::R`](R) reader structure"]
impl crate::Readable for IvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ivctrl::W`](W) writer structure"]
impl crate::Writable for IvctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IVCTRL to value 0"]
impl crate::Resettable for IvctrlSpec {
    const RESET_VALUE: u32 = 0;
}
