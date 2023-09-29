#[doc = "Register `FSCAL3` reader"]
pub type R = crate::R<FSCAL3_SPEC>;
#[doc = "Register `FSCAL3` writer"]
pub type W = crate::W<FSCAL3_SPEC>;
#[doc = "Field `VCO_CAPARR_CAL_CTRL` reader - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
pub type VCO_CAPARR_CAL_CTRL_R = crate::FieldReader;
#[doc = "Field `VCO_CAPARR_CAL_CTRL` writer - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
pub type VCO_CAPARR_CAL_CTRL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `VCO_VC_DAC` reader - Bit vector for programming varactor control voltage from VC DAC"]
pub type VCO_VC_DAC_R = crate::FieldReader;
#[doc = "Field `VCO_VC_DAC` writer - Bit vector for programming varactor control voltage from VC DAC"]
pub type VCO_VC_DAC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `VCO_DAC_EN_OV` reader - Enables the VCO DAC when 1"]
pub type VCO_DAC_EN_OV_R = crate::BitReader;
#[doc = "Field `VCO_DAC_EN_OV` writer - Enables the VCO DAC when 1"]
pub type VCO_DAC_EN_OV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:1 - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
    #[inline(always)]
    pub fn vco_caparr_cal_ctrl(&self) -> VCO_CAPARR_CAL_CTRL_R {
        VCO_CAPARR_CAL_CTRL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Bit vector for programming varactor control voltage from VC DAC"]
    #[inline(always)]
    pub fn vco_vc_dac(&self) -> VCO_VC_DAC_R {
        VCO_VC_DAC_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Enables the VCO DAC when 1"]
    #[inline(always)]
    pub fn vco_dac_en_ov(&self) -> VCO_DAC_EN_OV_R {
        VCO_DAC_EN_OV_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Calibration accuracy setting for the cap_array calibration part of the calibration 00: 80 XOSC periods 01: 100 XOSC periods 10: 125 XOSC periods 11: 250 XOSC periods"]
    #[inline(always)]
    #[must_use]
    pub fn vco_caparr_cal_ctrl(&mut self) -> VCO_CAPARR_CAL_CTRL_W<FSCAL3_SPEC, 0> {
        VCO_CAPARR_CAL_CTRL_W::new(self)
    }
    #[doc = "Bits 2:5 - Bit vector for programming varactor control voltage from VC DAC"]
    #[inline(always)]
    #[must_use]
    pub fn vco_vc_dac(&mut self) -> VCO_VC_DAC_W<FSCAL3_SPEC, 2> {
        VCO_VC_DAC_W::new(self)
    }
    #[doc = "Bit 6 - Enables the VCO DAC when 1"]
    #[inline(always)]
    #[must_use]
    pub fn vco_dac_en_ov(&mut self) -> VCO_DAC_EN_OV_W<FSCAL3_SPEC, 6> {
        VCO_DAC_EN_OV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Tune frequency calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscal3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscal3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSCAL3_SPEC;
impl crate::RegisterSpec for FSCAL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscal3::R`](R) reader structure"]
impl crate::Readable for FSCAL3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fscal3::W`](W) writer structure"]
impl crate::Writable for FSCAL3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSCAL3 to value 0"]
impl crate::Resettable for FSCAL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
