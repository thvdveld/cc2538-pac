#[doc = "Register `SRI2C` reader"]
pub type R = crate::R<Sri2cSpec>;
#[doc = "Register `SRI2C` writer"]
pub type W = crate::W<Sri2cSpec>;
#[doc = "Field `I2C0` reader - 0: I2C0 module is not reset 1: I2C0 module is reset"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C0` writer - 0: I2C0 module is not reset 1: I2C0 module is reset"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: I2C0 module is not reset 1: I2C0 module is reset"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: I2C0 module is not reset 1: I2C0 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2c0W<Sri2cSpec> {
        I2c0W::new(self, 0)
    }
}
#[doc = "This register controls the reset for I2C.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sri2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sri2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sri2cSpec;
impl crate::RegisterSpec for Sri2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sri2c::R`](R) reader structure"]
impl crate::Readable for Sri2cSpec {}
#[doc = "`write(|w| ..)` method takes [`sri2c::W`](W) writer structure"]
impl crate::Writable for Sri2cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRI2C to value 0"]
impl crate::Resettable for Sri2cSpec {
    const RESET_VALUE: u32 = 0;
}
