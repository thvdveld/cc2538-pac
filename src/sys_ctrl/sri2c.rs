#[doc = "Register `SRI2C` reader"]
pub type R = crate::R<SRI2C_SPEC>;
#[doc = "Register `SRI2C` writer"]
pub type W = crate::W<SRI2C_SPEC>;
#[doc = "Field `I2C0` reader - 0: I2C0 module is not reset 1: I2C0 module is reset"]
pub type I2C0_R = crate::BitReader;
#[doc = "Field `I2C0` writer - 0: I2C0 module is not reset 1: I2C0 module is reset"]
pub type I2C0_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 0: I2C0 module is not reset 1: I2C0 module is reset"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: I2C0 module is not reset 1: I2C0 module is reset"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<SRI2C_SPEC, 0> {
        I2C0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register controls the reset for I2C.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sri2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sri2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRI2C_SPEC;
impl crate::RegisterSpec for SRI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sri2c::R`](R) reader structure"]
impl crate::Readable for SRI2C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sri2c::W`](W) writer structure"]
impl crate::Writable for SRI2C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRI2C to value 0"]
impl crate::Resettable for SRI2C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
