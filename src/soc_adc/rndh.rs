#[doc = "Register `RNDH` reader"]
pub type R = crate::R<RndhSpec>;
#[doc = "Register `RNDH` writer"]
pub type W = crate::W<RndhSpec>;
#[doc = "Field `RNDH` reader - Random value or CRC result/input data, high byte When written, a CRC16 calculation is triggered, and the data value written is processed starting with the MSB. The value returned when reading from this register is the 8 MSBs of the LFSR. When used for random-number generation, reading this register returns the 8 MSBs of the random number. When used for CRC calculations, reading this register returns the 8 MSBs of the CRC result."]
pub type RndhR = crate::FieldReader;
#[doc = "Field `RNDH` writer - Random value or CRC result/input data, high byte When written, a CRC16 calculation is triggered, and the data value written is processed starting with the MSB. The value returned when reading from this register is the 8 MSBs of the LFSR. When used for random-number generation, reading this register returns the 8 MSBs of the random number. When used for CRC calculations, reading this register returns the 8 MSBs of the CRC result."]
pub type RndhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Random value or CRC result/input data, high byte When written, a CRC16 calculation is triggered, and the data value written is processed starting with the MSB. The value returned when reading from this register is the 8 MSBs of the LFSR. When used for random-number generation, reading this register returns the 8 MSBs of the random number. When used for CRC calculations, reading this register returns the 8 MSBs of the CRC result."]
    #[inline(always)]
    pub fn rndh(&self) -> RndhR {
        RndhR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Random value or CRC result/input data, high byte When written, a CRC16 calculation is triggered, and the data value written is processed starting with the MSB. The value returned when reading from this register is the 8 MSBs of the LFSR. When used for random-number generation, reading this register returns the 8 MSBs of the random number. When used for CRC calculations, reading this register returns the 8 MSBs of the CRC result."]
    #[inline(always)]
    #[must_use]
    pub fn rndh(&mut self) -> RndhW<RndhSpec> {
        RndhW::new(self, 0)
    }
}
#[doc = "This register contains random-number-generator data; high byte.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rndh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rndh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RndhSpec;
impl crate::RegisterSpec for RndhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rndh::R`](R) reader structure"]
impl crate::Readable for RndhSpec {}
#[doc = "`write(|w| ..)` method takes [`rndh::W`](W) writer structure"]
impl crate::Writable for RndhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RNDH to value 0"]
impl crate::Resettable for RndhSpec {
    const RESET_VALUE: u32 = 0;
}
