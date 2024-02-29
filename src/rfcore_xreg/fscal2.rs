#[doc = "Register `FSCAL2` reader"]
pub type R = crate::R<Fscal2Spec>;
#[doc = "Register `FSCAL2` writer"]
pub type W = crate::W<Fscal2Spec>;
#[doc = "Field `VCO_CAPARR` reader - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
pub type VcoCaparrR = crate::FieldReader;
#[doc = "Field `VCO_CAPARR` writer - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
pub type VcoCaparrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VCO_CAPARR_OE` reader - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
pub type VcoCaparrOeR = crate::BitReader;
#[doc = "Field `VCO_CAPARR_OE` writer - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
pub type VcoCaparrOeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
    #[inline(always)]
    pub fn vco_caparr(&self) -> VcoCaparrR {
        VcoCaparrR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
    #[inline(always)]
    pub fn vco_caparr_oe(&self) -> VcoCaparrOeR {
        VcoCaparrOeR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn vco_caparr(&mut self) -> VcoCaparrW<Fscal2Spec> {
        VcoCaparrW::new(self, 0)
    }
    #[doc = "Bit 6 - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn vco_caparr_oe(&mut self) -> VcoCaparrOeW<Fscal2Spec> {
        VcoCaparrOeW::new(self, 6)
    }
}
#[doc = "Tune frequency calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscal2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscal2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fscal2Spec;
impl crate::RegisterSpec for Fscal2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscal2::R`](R) reader structure"]
impl crate::Readable for Fscal2Spec {}
#[doc = "`write(|w| ..)` method takes [`fscal2::W`](W) writer structure"]
impl crate::Writable for Fscal2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSCAL2 to value 0"]
impl crate::Resettable for Fscal2Spec {
    const RESET_VALUE: u32 = 0;
}
