#[doc = "Register `LPBKI2C` reader"]
pub type R = crate::R<LPBKI2C_SPEC>;
#[doc = "Register `LPBKI2C` writer"]
pub type W = crate::W<LPBKI2C_SPEC>;
#[doc = "Field `LPBKI2C` reader - I2C0 Master/slave loopback mode 0: Normal mode"]
pub type LPBKI2C_R = crate::BitReader;
#[doc = "Field `LPBKI2C` writer - I2C0 Master/slave loopback mode 0: Normal mode"]
pub type LPBKI2C_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C0 Master/slave loopback mode 0: Normal mode"]
    #[inline(always)]
    pub fn lpbki2c(&self) -> LPBKI2C_R {
        LPBKI2C_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0 Master/slave loopback mode 0: Normal mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpbki2c(&mut self) -> LPBKI2C_W<LPBKI2C_SPEC> {
        LPBKI2C_W::new(self, 0)
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
#[doc = "I2C internal loopback\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpbki2c::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpbki2c::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPBKI2C_SPEC;
impl crate::RegisterSpec for LPBKI2C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpbki2c::R`](R) reader structure"]
impl crate::Readable for LPBKI2C_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpbki2c::W`](W) writer structure"]
impl crate::Writable for LPBKI2C_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPBKI2C to value 0"]
impl crate::Resettable for LPBKI2C_SPEC {
    const RESET_VALUE: u32 = 0;
}
