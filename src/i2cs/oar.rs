#[doc = "Register `OAR` reader"]
pub type R = crate::R<OAR_SPEC>;
#[doc = "Register `OAR` writer"]
pub type W = crate::W<OAR_SPEC>;
#[doc = "Field `OAR` reader - I2C slave own address This field specifies bits A6 through A0 of the slave address."]
pub type OAR_R = crate::FieldReader;
#[doc = "Field `OAR` writer - I2C slave own address This field specifies bits A6 through A0 of the slave address."]
pub type OAR_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - I2C slave own address This field specifies bits A6 through A0 of the slave address."]
    #[inline(always)]
    pub fn oar(&self) -> OAR_R {
        OAR_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - I2C slave own address This field specifies bits A6 through A0 of the slave address."]
    #[inline(always)]
    #[must_use]
    pub fn oar(&mut self) -> OAR_W<OAR_SPEC> {
        OAR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2C slave own address This register consists of seven address bits that identify the CC2538 I2C device on the I2C bus.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OAR_SPEC;
impl crate::RegisterSpec for OAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar::R`](R) reader structure"]
impl crate::Readable for OAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oar::W`](W) writer structure"]
impl crate::Writable for OAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OAR to value 0"]
impl crate::Resettable for OAR_SPEC {
    const RESET_VALUE: u32 = 0;
}
