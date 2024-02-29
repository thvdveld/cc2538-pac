#[doc = "Register `SCGCI2C` reader"]
pub type R = crate::R<Scgci2cSpec>;
#[doc = "Register `SCGCI2C` writer"]
pub type W = crate::W<Scgci2cSpec>;
#[doc = "Field `I2C0` reader - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C0` writer - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2c0W<Scgci2cSpec> {
        I2c0W::new(self, 0)
    }
}
#[doc = "This register defines the module clocks for I2C when the CPU is in sleep mode. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scgci2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scgci2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgci2cSpec;
impl crate::RegisterSpec for Scgci2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgci2c::R`](R) reader structure"]
impl crate::Readable for Scgci2cSpec {}
#[doc = "`write(|w| ..)` method takes [`scgci2c::W`](W) writer structure"]
impl crate::Writable for Scgci2cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGCI2C to value 0"]
impl crate::Resettable for Scgci2cSpec {
    const RESET_VALUE: u32 = 0;
}
