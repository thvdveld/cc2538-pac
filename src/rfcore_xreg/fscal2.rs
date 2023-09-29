#[doc = "Register `FSCAL2` reader"]
pub type R = crate::R<FSCAL2_SPEC>;
#[doc = "Register `FSCAL2` writer"]
pub type W = crate::W<FSCAL2_SPEC>;
#[doc = "Field `VCO_CAPARR` reader - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
pub type VCO_CAPARR_R = crate::FieldReader;
#[doc = "Field `VCO_CAPARR` writer - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
pub type VCO_CAPARR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `VCO_CAPARR_OE` reader - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
pub type VCO_CAPARR_OE_R = crate::BitReader;
#[doc = "Field `VCO_CAPARR_OE` writer - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
pub type VCO_CAPARR_OE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:5 - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
    #[inline(always)]
    pub fn vco_caparr(&self) -> VCO_CAPARR_R {
        VCO_CAPARR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
    #[inline(always)]
    pub fn vco_caparr_oe(&self) -> VCO_CAPARR_OE_R {
        VCO_CAPARR_OE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
    #[inline(always)]
    #[must_use]
    pub fn vco_caparr(&mut self) -> VCO_CAPARR_W<FSCAL2_SPEC, 0> {
        VCO_CAPARR_W::new(self)
    }
    #[doc = "Bit 6 - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn vco_caparr_oe(&mut self) -> VCO_CAPARR_OE_W<FSCAL2_SPEC, 6> {
        VCO_CAPARR_OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Tune frequency calibration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fscal2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fscal2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSCAL2_SPEC;
impl crate::RegisterSpec for FSCAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fscal2::R`](R) reader structure"]
impl crate::Readable for FSCAL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fscal2::W`](W) writer structure"]
impl crate::Writable for FSCAL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSCAL2 to value 0"]
impl crate::Resettable for FSCAL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
