#[doc = "Register `DCGCI2C` reader"]
pub type R = crate::R<DCGCI2C_SPEC>;
#[doc = "Register `DCGCI2C` writer"]
pub type W = crate::W<DCGCI2C_SPEC>;
#[doc = "Field `I2C0` reader - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
pub type I2C0_R = crate::BitReader;
#[doc = "Field `I2C0` writer - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
pub type I2C0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Clock for I2C0 is gated. 1: Clock for I2C0 is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<DCGCI2C_SPEC, 0> {
        I2C0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register defines the module clocks for I2C when the CPU is in PM0. This register setting is don't care for PM1-3, because the system clock is powered down in these modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcgci2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcgci2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCGCI2C_SPEC;
impl crate::RegisterSpec for DCGCI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcgci2c::R`](R) reader structure"]
impl crate::Readable for DCGCI2C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcgci2c::W`](W) writer structure"]
impl crate::Writable for DCGCI2C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCGCI2C to value 0"]
impl crate::Resettable for DCGCI2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
