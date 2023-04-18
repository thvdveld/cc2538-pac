#[doc = "Register `FSCAL2` reader"]
pub struct R(crate::R<FSCAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSCAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSCAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSCAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSCAL2` writer"]
pub struct W(crate::W<FSCAL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSCAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FSCAL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSCAL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCO_CAPARR` reader - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
pub type VCO_CAPARR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCO_CAPARR` writer - VCO capacitor array setting Programmed during calibration Override value when VCO_CAPARR_OE = 1"]
pub type VCO_CAPARR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FSCAL2_SPEC, u8, u8, 6, O>;
#[doc = "Field `VCO_CAPARR_OE` reader - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
pub type VCO_CAPARR_OE_R = crate::BitReader<bool>;
#[doc = "Field `VCO_CAPARR_OE` writer - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
pub type VCO_CAPARR_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FSCAL2_SPEC, bool, O>;
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
    pub fn vco_caparr(&mut self) -> VCO_CAPARR_W<0> {
        VCO_CAPARR_W::new(self)
    }
    #[doc = "Bit 6 - Override the calibration result with the value from VCO_CAPARR\\[5:0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn vco_caparr_oe(&mut self) -> VCO_CAPARR_OE_W<6> {
        VCO_CAPARR_OE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tune frequency calibration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fscal2](index.html) module"]
pub struct FSCAL2_SPEC;
impl crate::RegisterSpec for FSCAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fscal2::R](R) reader structure"]
impl crate::Readable for FSCAL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fscal2::W](W) writer structure"]
impl crate::Writable for FSCAL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSCAL2 to value 0"]
impl crate::Resettable for FSCAL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
