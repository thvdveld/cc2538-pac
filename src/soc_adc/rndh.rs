#[doc = "Register `RNDH` reader"]
pub struct R(crate::R<RNDH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNDH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNDH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNDH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNDH` writer"]
pub struct W(crate::W<RNDH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNDH_SPEC>;
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
impl From<crate::W<RNDH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNDH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNDH` reader - Random value or CRC result/input data, high byte When written, a CRC16 calculation is triggered, and the data value written is processed starting with the MSB. The value returned when reading from this register is the 8 MSBs of the LFSR. When used for random-number generation, reading this register returns the 8 MSBs of the random number. When used for CRC calculations, reading this register returns the 8 MSBs of the CRC result."]
pub type RNDH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RNDH` writer - Random value or CRC result/input data, high byte When written, a CRC16 calculation is triggered, and the data value written is processed starting with the MSB. The value returned when reading from this register is the 8 MSBs of the LFSR. When used for random-number generation, reading this register returns the 8 MSBs of the random number. When used for CRC calculations, reading this register returns the 8 MSBs of the CRC result."]
pub type RNDH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RNDH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Random value or CRC result/input data, high byte When written, a CRC16 calculation is triggered, and the data value written is processed starting with the MSB. The value returned when reading from this register is the 8 MSBs of the LFSR. When used for random-number generation, reading this register returns the 8 MSBs of the random number. When used for CRC calculations, reading this register returns the 8 MSBs of the CRC result."]
    #[inline(always)]
    pub fn rndh(&self) -> RNDH_R {
        RNDH_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Random value or CRC result/input data, high byte When written, a CRC16 calculation is triggered, and the data value written is processed starting with the MSB. The value returned when reading from this register is the 8 MSBs of the LFSR. When used for random-number generation, reading this register returns the 8 MSBs of the random number. When used for CRC calculations, reading this register returns the 8 MSBs of the CRC result."]
    #[inline(always)]
    #[must_use]
    pub fn rndh(&mut self) -> RNDH_W<0> {
        RNDH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register contains random-number-generator data; high byte.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rndh](index.html) module"]
pub struct RNDH_SPEC;
impl crate::RegisterSpec for RNDH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rndh::R](R) reader structure"]
impl crate::Readable for RNDH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rndh::W](W) writer structure"]
impl crate::Writable for RNDH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNDH to value 0"]
impl crate::Resettable for RNDH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
