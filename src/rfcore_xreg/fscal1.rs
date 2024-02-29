#[doc = "Register `FSCAL1` reader"]
pub type R = crate::R<Fscal1Spec>;
#[doc = "Register `FSCAL1` writer"]
pub type W = crate::W<Fscal1Spec>;
#[doc = "Field `VCO_CURR` reader - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
pub type VcoCurrR = crate::FieldReader;
#[doc = "Field `VCO_CURR` writer - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
pub type VcoCurrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VCO_CURR_CAL` reader - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
pub type VcoCurrCalR = crate::FieldReader;
#[doc = "Field `VCO_CURR_CAL` writer - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
pub type VcoCurrCalW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VCO_CURR_CAL_OE` reader - Override current calibration"]
pub type VcoCurrCalOeR = crate::BitReader;
#[doc = "Field `VCO_CURR_CAL_OE` writer - Override current calibration"]
pub type VcoCurrCalOeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
    #[inline(always)]
    pub fn vco_curr(&self) -> VcoCurrR {
        VcoCurrR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
    #[inline(always)]
    pub fn vco_curr_cal(&self) -> VcoCurrCalR {
        VcoCurrCalR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Override current calibration"]
    #[inline(always)]
    pub fn vco_curr_cal_oe(&self) -> VcoCurrCalOeR {
        VcoCurrCalOeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
    #[inline(always)]
    #[must_use]
    pub fn vco_curr(&mut self) -> VcoCurrW<Fscal1Spec> {
        VcoCurrW::new(self, 0)
    }
    #[doc = "Bits 2:6 - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn vco_curr_cal(&mut self) -> VcoCurrCalW<Fscal1Spec> {
        VcoCurrCalW::new(self, 2)
    }
    #[doc = "Bit 7 - Override current calibration"]
    #[inline(always)]
    #[must_use]
    pub fn vco_curr_cal_oe(&mut self) -> VcoCurrCalOeW<Fscal1Spec> {
        VcoCurrCalOeW::new(self, 7)
    }
}
#[doc = "Tune frequency calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscal1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscal1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fscal1Spec;
impl crate::RegisterSpec for Fscal1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscal1::R`](R) reader structure"]
impl crate::Readable for Fscal1Spec {}
#[doc = "`write(|w| ..)` method takes [`fscal1::W`](W) writer structure"]
impl crate::Writable for Fscal1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSCAL1 to value 0"]
impl crate::Resettable for Fscal1Spec {
    const RESET_VALUE: u32 = 0;
}
