#[doc = "Register `RNDL` reader"]
pub type R = crate::R<RNDL_SPEC>;
#[doc = "Register `RNDL` writer"]
pub type W = crate::W<RNDL_SPEC>;
#[doc = "Field `RNDL` reader - Random value/seed or CRC result, low byte When used for random-number generation, writing to this register twice seeds the random-number generator. Writing to this register copies the 8 LSBs of the LFSR to the 8 MSBs and replaces the 8 LSBs with the data value written. The value returned when reading from this register is the 8 LSBs of the LFSR. When used for random-number generation, reading this register returns the 8 LSBs of the random number. When used for CRC calculations, reading this register returns the 8 LSBs of the CRC result."]
pub type RNDL_R = crate::FieldReader;
#[doc = "Field `RNDL` writer - Random value/seed or CRC result, low byte When used for random-number generation, writing to this register twice seeds the random-number generator. Writing to this register copies the 8 LSBs of the LFSR to the 8 MSBs and replaces the 8 LSBs with the data value written. The value returned when reading from this register is the 8 LSBs of the LFSR. When used for random-number generation, reading this register returns the 8 LSBs of the random number. When used for CRC calculations, reading this register returns the 8 LSBs of the CRC result."]
pub type RNDL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
    #[must_use]
    pub fn rndl(&mut self) -> RNDL_W<RNDL_SPEC, 0> {
        RNDL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This registers contains random-number-generator data; low byte.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rndl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rndl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNDL_SPEC;
impl crate::RegisterSpec for RNDL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rndl::R`](R) reader structure"]
impl crate::Readable for RNDL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rndl::W`](W) writer structure"]
impl crate::Writable for RNDL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNDL to value 0"]
impl crate::Resettable for RNDL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
