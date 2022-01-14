#[doc = "Register `RNDL` reader"]
pub struct R(crate::R<RNDL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNDL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNDL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNDL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNDL` writer"]
pub struct W(crate::W<RNDL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNDL_SPEC>;
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
impl From<crate::W<RNDL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNDL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNDL` reader - Random value/seed or CRC result, low byte When used for random-number generation, writing to this register twice seeds the random-number generator. Writing to this register copies the 8 LSBs of the LFSR to the 8 MSBs and replaces the 8 LSBs with the data value written. The value returned when reading from this register is the 8 LSBs of the LFSR. When used for random-number generation, reading this register returns the 8 LSBs of the random number. When used for CRC calculations, reading this register returns the 8 LSBs of the CRC result."]
pub struct RNDL_R(crate::FieldReader<u8, u8>);
impl RNDL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RNDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RNDL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNDL` writer - Random value/seed or CRC result, low byte When used for random-number generation, writing to this register twice seeds the random-number generator. Writing to this register copies the 8 LSBs of the LFSR to the 8 MSBs and replaces the 8 LSBs with the data value written. The value returned when reading from this register is the 8 LSBs of the LFSR. When used for random-number generation, reading this register returns the 8 LSBs of the random number. When used for CRC calculations, reading this register returns the 8 LSBs of the CRC result."]
pub struct RNDL_W<'a> {
    w: &'a mut W,
}
impl<'a> RNDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Random value/seed or CRC result, low byte When used for random-number generation, writing to this register twice seeds the random-number generator. Writing to this register copies the 8 LSBs of the LFSR to the 8 MSBs and replaces the 8 LSBs with the data value written. The value returned when reading from this register is the 8 LSBs of the LFSR. When used for random-number generation, reading this register returns the 8 LSBs of the random number. When used for CRC calculations, reading this register returns the 8 LSBs of the CRC result."]
    #[inline(always)]
    pub fn rndl(&self) -> RNDL_R {
        RNDL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Random value/seed or CRC result, low byte When used for random-number generation, writing to this register twice seeds the random-number generator. Writing to this register copies the 8 LSBs of the LFSR to the 8 MSBs and replaces the 8 LSBs with the data value written. The value returned when reading from this register is the 8 LSBs of the LFSR. When used for random-number generation, reading this register returns the 8 LSBs of the random number. When used for CRC calculations, reading this register returns the 8 LSBs of the CRC result."]
    #[inline(always)]
    pub fn rndl(&mut self) -> RNDL_W {
        RNDL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This registers contains random-number-generator data; low byte.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rndl](index.html) module"]
pub struct RNDL_SPEC;
impl crate::RegisterSpec for RNDL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rndl::R](R) reader structure"]
impl crate::Readable for RNDL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rndl::W](W) writer structure"]
impl crate::Writable for RNDL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNDL to value 0"]
impl crate::Resettable for RNDL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
