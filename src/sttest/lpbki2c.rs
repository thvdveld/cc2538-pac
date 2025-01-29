#[doc = "Register `LPBKI2C` reader"]
pub type R = crate::R<Lpbki2cSpec>;
#[doc = "Register `LPBKI2C` writer"]
pub type W = crate::W<Lpbki2cSpec>;
#[doc = "Field `LPBKI2C` reader - I2C0 Master/slave loopback mode 0: Normal mode"]
pub type Lpbki2cR = crate::BitReader;
#[doc = "Field `LPBKI2C` writer - I2C0 Master/slave loopback mode 0: Normal mode"]
pub type Lpbki2cW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C0 Master/slave loopback mode 0: Normal mode"]
    #[inline(always)]
    pub fn lpbki2c(&self) -> Lpbki2cR {
        Lpbki2cR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0 Master/slave loopback mode 0: Normal mode"]
    #[inline(always)]
    pub fn lpbki2c(&mut self) -> Lpbki2cW<Lpbki2cSpec> {
        Lpbki2cW::new(self, 0)
    }
}
#[doc = "I2C internal loopback\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbki2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbki2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lpbki2cSpec;
impl crate::RegisterSpec for Lpbki2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpbki2c::R`](R) reader structure"]
impl crate::Readable for Lpbki2cSpec {}
#[doc = "`write(|w| ..)` method takes [`lpbki2c::W`](W) writer structure"]
impl crate::Writable for Lpbki2cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPBKI2C to value 0"]
impl crate::Resettable for Lpbki2cSpec {
    const RESET_VALUE: u32 = 0;
}
