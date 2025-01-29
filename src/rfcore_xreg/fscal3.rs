#[doc = "Register `FSCAL3` reader"]
pub type R = crate::R<Fscal3Spec>;
#[doc = "Register `FSCAL3` writer"]
pub type W = crate::W<Fscal3Spec>;
#[doc = "Field `VCO_CAPARR_CAL_CTRL` reader - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
pub type VcoCaparrCalCtrlR = crate::FieldReader;
#[doc = "Field `VCO_CAPARR_CAL_CTRL` writer - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
pub type VcoCaparrCalCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VCO_VC_DAC` reader - Bit vector for programming varactor control voltage from VC DAC"]
pub type VcoVcDacR = crate::FieldReader;
#[doc = "Field `VCO_VC_DAC` writer - Bit vector for programming varactor control voltage from VC DAC"]
pub type VcoVcDacW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VCO_DAC_EN_OV` reader - Enables the VCO DAC when 1"]
pub type VcoDacEnOvR = crate::BitReader;
#[doc = "Field `VCO_DAC_EN_OV` writer - Enables the VCO DAC when 1"]
pub type VcoDacEnOvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
    #[inline(always)]
    pub fn vco_caparr_cal_ctrl(&self) -> VcoCaparrCalCtrlR {
        VcoCaparrCalCtrlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Bit vector for programming varactor control voltage from VC DAC"]
    #[inline(always)]
    pub fn vco_vc_dac(&self) -> VcoVcDacR {
        VcoVcDacR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Enables the VCO DAC when 1"]
    #[inline(always)]
    pub fn vco_dac_en_ov(&self) -> VcoDacEnOvR {
        VcoDacEnOvR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
    #[inline(always)]
    pub fn vco_caparr_cal_ctrl(&mut self) -> VcoCaparrCalCtrlW<Fscal3Spec> {
        VcoCaparrCalCtrlW::new(self, 0)
    }
    #[doc = "Bits 2:5 - Bit vector for programming varactor control voltage from VC DAC"]
    #[inline(always)]
    pub fn vco_vc_dac(&mut self) -> VcoVcDacW<Fscal3Spec> {
        VcoVcDacW::new(self, 2)
    }
    #[doc = "Bit 6 - Enables the VCO DAC when 1"]
    #[inline(always)]
    pub fn vco_dac_en_ov(&mut self) -> VcoDacEnOvW<Fscal3Spec> {
        VcoDacEnOvW::new(self, 6)
    }
}
#[doc = "Tune frequency calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`fscal3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fscal3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fscal3Spec;
impl crate::RegisterSpec for Fscal3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscal3::R`](R) reader structure"]
impl crate::Readable for Fscal3Spec {}
#[doc = "`write(|w| ..)` method takes [`fscal3::W`](W) writer structure"]
impl crate::Writable for Fscal3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSCAL3 to value 0"]
impl crate::Resettable for Fscal3Spec {
    const RESET_VALUE: u32 = 0;
}
