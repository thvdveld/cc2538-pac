#[doc = "Register `RNDH` reader"]
pub type R = crate::R<RNDH_SPEC>;
#[doc = "Register `RNDH` writer"]
pub type W = crate::W<RNDH_SPEC>;
#[doc = "Field `RNDH` reader - Random value or CRC result/input data, high byte When written, a CRC16 calculation is triggered, and the data value written is processed starting with the MSB. The value returned when reading from this register is the 8 MSBs of the LFSR. When used for random-number generation, reading this register returns the 8 MSBs of the random number. When used for CRC calculations, reading this register returns the 8 MSBs of the CRC result."]
pub type RNDH_R = crate::FieldReader;
#[doc = "Field `RNDH` writer - Random value or CRC result/input data, high byte When written, a CRC16 calculation is triggered, and the data value written is processed starting with the MSB. The value returned when reading from this register is the 8 MSBs of the LFSR. When used for random-number generation, reading this register returns the 8 MSBs of the random number. When used for CRC calculations, reading this register returns the 8 MSBs of the CRC result."]
pub type RNDH_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
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
    pub fn rndh(&mut self) -> RNDH_W<RNDH_SPEC, 0> {
        RNDH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register contains random-number-generator data; high byte.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rndh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rndh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RNDH_SPEC;
impl crate::RegisterSpec for RNDH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rndh::R`](R) reader structure"]
impl crate::Readable for RNDH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rndh::W`](W) writer structure"]
impl crate::Writable for RNDH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RNDH to value 0"]
impl crate::Resettable for RNDH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
