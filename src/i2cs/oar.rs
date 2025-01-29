#[doc = "Register `OAR` reader"]
pub type R = crate::R<OarSpec>;
#[doc = "Register `OAR` writer"]
pub type W = crate::W<OarSpec>;
#[doc = "Field `OAR` reader - I2C slave own address This field specifies bits A6 through A0 of the slave address."]
pub type OarR = crate::FieldReader;
#[doc = "Field `OAR` writer - I2C slave own address This field specifies bits A6 through A0 of the slave address."]
pub type OarW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - I2C slave own address This field specifies bits A6 through A0 of the slave address."]
    #[inline(always)]
    pub fn oar(&self) -> OarR {
        OarR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - I2C slave own address This field specifies bits A6 through A0 of the slave address."]
    #[inline(always)]
    pub fn oar(&mut self) -> OarW<OarSpec> {
        OarW::new(self, 0)
    }
}
#[doc = "I2C slave own address This register consists of seven address bits that identify the CC2538 I2C device on the I2C bus.\n\nYou can [`read`](crate::Reg::read) this register and get [`oar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OarSpec;
impl crate::RegisterSpec for OarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar::R`](R) reader structure"]
impl crate::Readable for OarSpec {}
#[doc = "`write(|w| ..)` method takes [`oar::W`](W) writer structure"]
impl crate::Writable for OarSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR to value 0"]
impl crate::Resettable for OarSpec {
    const RESET_VALUE: u32 = 0;
}
