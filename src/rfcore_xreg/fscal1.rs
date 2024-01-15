#[doc = "Register `FSCAL1` reader"]
pub type R = crate::R<FSCAL1_SPEC>;
#[doc = "Register `FSCAL1` writer"]
pub type W = crate::W<FSCAL1_SPEC>;
#[doc = "Field `VCO_CURR` reader - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
pub type VCO_CURR_R = crate::FieldReader;
#[doc = "Field `VCO_CURR` writer - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
pub type VCO_CURR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VCO_CURR_CAL` reader - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
pub type VCO_CURR_CAL_R = crate::FieldReader;
#[doc = "Field `VCO_CURR_CAL` writer - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
pub type VCO_CURR_CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VCO_CURR_CAL_OE` reader - Override current calibration"]
pub type VCO_CURR_CAL_OE_R = crate::BitReader;
#[doc = "Field `VCO_CURR_CAL_OE` writer - Override current calibration"]
pub type VCO_CURR_CAL_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
    #[inline(always)]
    pub fn vco_curr(&self) -> VCO_CURR_R {
        VCO_CURR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
    #[inline(always)]
    pub fn vco_curr_cal(&self) -> VCO_CURR_CAL_R {
        VCO_CURR_CAL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Override current calibration"]
    #[inline(always)]
    pub fn vco_curr_cal_oe(&self) -> VCO_CURR_CAL_OE_R {
        VCO_CURR_CAL_OE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Defines current in VCO core Sets the multiplier between calibrated current and VCO current."]
    #[inline(always)]
    #[must_use]
    pub fn vco_curr(&mut self) -> VCO_CURR_W<FSCAL1_SPEC> {
        VCO_CURR_W::new(self, 0)
    }
    #[doc = "Bits 2:6 - Calibration result Override value if VCO_CURR_CAL_OE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn vco_curr_cal(&mut self) -> VCO_CURR_CAL_W<FSCAL1_SPEC> {
        VCO_CURR_CAL_W::new(self, 2)
    }
    #[doc = "Bit 7 - Override current calibration"]
    #[inline(always)]
    #[must_use]
    pub fn vco_curr_cal_oe(&mut self) -> VCO_CURR_CAL_OE_W<FSCAL1_SPEC> {
        VCO_CURR_CAL_OE_W::new(self, 7)
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
#[doc = "Tune frequency calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscal1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscal1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSCAL1_SPEC;
impl crate::RegisterSpec for FSCAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscal1::R`](R) reader structure"]
impl crate::Readable for FSCAL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fscal1::W`](W) writer structure"]
impl crate::Writable for FSCAL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSCAL1 to value 0"]
impl crate::Resettable for FSCAL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
