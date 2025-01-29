#[doc = "Register `RNDL` reader"]
pub type R = crate::R<RndlSpec>;
#[doc = "Register `RNDL` writer"]
pub type W = crate::W<RndlSpec>;
#[doc = "Field `RNDL` reader - Random value/seed or CRC result, low byte When used for random-number generation, writing to this register twice seeds the random-number generator. Writing to this register copies the 8 LSBs of the LFSR to the 8 MSBs and replaces the 8 LSBs with the data value written. The value returned when reading from this register is the 8 LSBs of the LFSR. When used for random-number generation, reading this register returns the 8 LSBs of the random number. When used for CRC calculations, reading this register returns the 8 LSBs of the CRC result."]
pub type RndlR = crate::FieldReader;
#[doc = "Field `RNDL` writer - Random value/seed or CRC result, low byte When used for random-number generation, writing to this register twice seeds the random-number generator. Writing to this register copies the 8 LSBs of the LFSR to the 8 MSBs and replaces the 8 LSBs with the data value written. The value returned when reading from this register is the 8 LSBs of the LFSR. When used for random-number generation, reading this register returns the 8 LSBs of the random number. When used for CRC calculations, reading this register returns the 8 LSBs of the CRC result."]
pub type RndlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Random value/seed or CRC result, low byte When used for random-number generation, writing to this register twice seeds the random-number generator. Writing to this register copies the 8 LSBs of the LFSR to the 8 MSBs and replaces the 8 LSBs with the data value written. The value returned when reading from this register is the 8 LSBs of the LFSR. When used for random-number generation, reading this register returns the 8 LSBs of the random number. When used for CRC calculations, reading this register returns the 8 LSBs of the CRC result."]
    #[inline(always)]
    pub fn rndl(&self) -> RndlR {
        RndlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Random value/seed or CRC result, low byte When used for random-number generation, writing to this register twice seeds the random-number generator. Writing to this register copies the 8 LSBs of the LFSR to the 8 MSBs and replaces the 8 LSBs with the data value written. The value returned when reading from this register is the 8 LSBs of the LFSR. When used for random-number generation, reading this register returns the 8 LSBs of the random number. When used for CRC calculations, reading this register returns the 8 LSBs of the CRC result."]
    #[inline(always)]
    pub fn rndl(&mut self) -> RndlW<RndlSpec> {
        RndlW::new(self, 0)
    }
}
#[doc = "This registers contains random-number-generator data; low byte.\n\nYou can [`read`](crate::Reg::read) this register and get [`rndl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rndl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RndlSpec;
impl crate::RegisterSpec for RndlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rndl::R`](R) reader structure"]
impl crate::Readable for RndlSpec {}
#[doc = "`write(|w| ..)` method takes [`rndl::W`](W) writer structure"]
impl crate::Writable for RndlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNDL to value 0"]
impl crate::Resettable for RndlSpec {
    const RESET_VALUE: u32 = 0;
}
