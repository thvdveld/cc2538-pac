#[doc = "Register `FSCAL0` reader"]
pub type R = crate::R<Fscal0Spec>;
#[doc = "Register `FSCAL0` writer"]
pub type W = crate::W<Fscal0Spec>;
#[doc = "Field `BW_BOOST_MODE` reader - Control signal Defines the synthesizer boost mode 00: No BW_BOOST 01: BW_BOOST is high during calibration and approximately 30 us into the settling. 10: BW_BOOST is always on (or high). 11: Reserved"]
pub type BwBoostModeR = crate::FieldReader;
#[doc = "Field `BW_BOOST_MODE` writer - Control signal Defines the synthesizer boost mode 00: No BW_BOOST 01: BW_BOOST is high during calibration and approximately 30 us into the settling. 10: BW_BOOST is always on (or high). 11: Reserved"]
pub type BwBoostModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHP_CURRENT` reader - Digital bit vector defining the charge-pump output current on an exponential scale If FFC_BW_BOOST = 0, the read value is the value stored in CHP_CURRENT. If FFC_BW_BOOST = 1, the read value is CHP_CURRENT + 4. If the addition causes overflow, the signal is saturated."]
pub type ChpCurrentR = crate::FieldReader;
#[doc = "Field `CHP_CURRENT` writer - Digital bit vector defining the charge-pump output current on an exponential scale If FFC_BW_BOOST = 0, the read value is the value stored in CHP_CURRENT. If FFC_BW_BOOST = 1, the read value is CHP_CURRENT + 4. If the addition causes overflow, the signal is saturated."]
pub type ChpCurrentW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHP_DISABLE` reader - Set this bit to manually disable charge pump by masking the up and down pulses from the phase detector."]
pub type ChpDisableR = crate::BitReader;
#[doc = "Field `CHP_DISABLE` writer - Set this bit to manually disable charge pump by masking the up and down pulses from the phase detector."]
pub type ChpDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCO_CURR_COMP_EN_OV` reader - Force on the current comparator in the VCO. This signal is ORed with the signal coming from the calibration module."]
pub type VcoCurrCompEnOvR = crate::BitReader;
#[doc = "Field `VCO_CURR_COMP_EN_OV` writer - Force on the current comparator in the VCO. This signal is ORed with the signal coming from the calibration module."]
pub type VcoCurrCompEnOvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Control signal Defines the synthesizer boost mode 00: No BW_BOOST 01: BW_BOOST is high during calibration and approximately 30 us into the settling. 10: BW_BOOST is always on (or high). 11: Reserved"]
    #[inline(always)]
    pub fn bw_boost_mode(&self) -> BwBoostModeR {
        BwBoostModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - Digital bit vector defining the charge-pump output current on an exponential scale If FFC_BW_BOOST = 0, the read value is the value stored in CHP_CURRENT. If FFC_BW_BOOST = 1, the read value is CHP_CURRENT + 4. If the addition causes overflow, the signal is saturated."]
    #[inline(always)]
    pub fn chp_current(&self) -> ChpCurrentR {
        ChpCurrentR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Set this bit to manually disable charge pump by masking the up and down pulses from the phase detector."]
    #[inline(always)]
    pub fn chp_disable(&self) -> ChpDisableR {
        ChpDisableR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force on the current comparator in the VCO. This signal is ORed with the signal coming from the calibration module."]
    #[inline(always)]
    pub fn vco_curr_comp_en_ov(&self) -> VcoCurrCompEnOvR {
        VcoCurrCompEnOvR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Control signal Defines the synthesizer boost mode 00: No BW_BOOST 01: BW_BOOST is high during calibration and approximately 30 us into the settling. 10: BW_BOOST is always on (or high). 11: Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn bw_boost_mode(&mut self) -> BwBoostModeW<Fscal0Spec> {
        BwBoostModeW::new(self, 0)
    }
    #[doc = "Bits 2:5 - Digital bit vector defining the charge-pump output current on an exponential scale If FFC_BW_BOOST = 0, the read value is the value stored in CHP_CURRENT. If FFC_BW_BOOST = 1, the read value is CHP_CURRENT + 4. If the addition causes overflow, the signal is saturated."]
    #[inline(always)]
    #[must_use]
    pub fn chp_current(&mut self) -> ChpCurrentW<Fscal0Spec> {
        ChpCurrentW::new(self, 2)
    }
    #[doc = "Bit 6 - Set this bit to manually disable charge pump by masking the up and down pulses from the phase detector."]
    #[inline(always)]
    #[must_use]
    pub fn chp_disable(&mut self) -> ChpDisableW<Fscal0Spec> {
        ChpDisableW::new(self, 6)
    }
    #[doc = "Bit 7 - Force on the current comparator in the VCO. This signal is ORed with the signal coming from the calibration module."]
    #[inline(always)]
    #[must_use]
    pub fn vco_curr_comp_en_ov(&mut self) -> VcoCurrCompEnOvW<Fscal0Spec> {
        VcoCurrCompEnOvW::new(self, 7)
    }
}
#[doc = "Tune frequency calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscal0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscal0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fscal0Spec;
impl crate::RegisterSpec for Fscal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscal0::R`](R) reader structure"]
impl crate::Readable for Fscal0Spec {}
#[doc = "`write(|w| ..)` method takes [`fscal0::W`](W) writer structure"]
impl crate::Writable for Fscal0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSCAL0 to value 0"]
impl crate::Resettable for Fscal0Spec {
    const RESET_VALUE: u32 = 0;
}
