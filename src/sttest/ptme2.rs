#[doc = "Register `PTME2` reader"]
pub type R = crate::R<Ptme2Spec>;
#[doc = "Register `PTME2` writer"]
pub type W = crate::W<Ptme2Spec>;
#[doc = "Field `I2C0TME` reader - I2C 0 test mode enable"]
pub type I2c0tmeR = crate::BitReader;
#[doc = "Field `I2C0TME` writer - I2C 0 test mode enable"]
pub type I2c0tmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T0TME` reader - Timer0 test mode enable"]
pub type T0tmeR = crate::BitReader;
#[doc = "Field `T0TME` writer - Timer0 test mode enable"]
pub type T0tmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T1TME` reader - Timer1 test mode enable"]
pub type T1tmeR = crate::BitReader;
#[doc = "Field `T1TME` writer - Timer1 test mode enable"]
pub type T1tmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTTME` reader - MacTimer test mode enable"]
pub type MttmeR = crate::BitReader;
#[doc = "Field `MTTME` writer - MacTimer test mode enable"]
pub type MttmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T3TME` reader - Timer3 test mode enable"]
pub type T3tmeR = crate::BitReader;
#[doc = "Field `T3TME` writer - Timer3 test mode enable"]
pub type T3tmeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C 0 test mode enable"]
    #[inline(always)]
    pub fn i2c0tme(&self) -> I2c0tmeR {
        I2c0tmeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Timer0 test mode enable"]
    #[inline(always)]
    pub fn t0tme(&self) -> T0tmeR {
        T0tmeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer1 test mode enable"]
    #[inline(always)]
    pub fn t1tme(&self) -> T1tmeR {
        T1tmeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MacTimer test mode enable"]
    #[inline(always)]
    pub fn mttme(&self) -> MttmeR {
        MttmeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer3 test mode enable"]
    #[inline(always)]
    pub fn t3tme(&self) -> T3tmeR {
        T3tmeR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C 0 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0tme(&mut self) -> I2c0tmeW<Ptme2Spec> {
        I2c0tmeW::new(self, 0)
    }
    #[doc = "Bit 16 - Timer0 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn t0tme(&mut self) -> T0tmeW<Ptme2Spec> {
        T0tmeW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer1 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn t1tme(&mut self) -> T1tmeW<Ptme2Spec> {
        T1tmeW::new(self, 17)
    }
    #[doc = "Bit 18 - MacTimer test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mttme(&mut self) -> MttmeW<Ptme2Spec> {
        MttmeW::new(self, 18)
    }
    #[doc = "Bit 19 - Timer3 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn t3tme(&mut self) -> T3tmeW<Ptme2Spec> {
        T3tmeW::new(self, 19)
    }
}
#[doc = "Peripheral test mode enable 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptme2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptme2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ptme2Spec;
impl crate::RegisterSpec for Ptme2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptme2::R`](R) reader structure"]
impl crate::Readable for Ptme2Spec {}
#[doc = "`write(|w| ..)` method takes [`ptme2::W`](W) writer structure"]
impl crate::Writable for Ptme2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTME2 to value 0"]
impl crate::Resettable for Ptme2Spec {
    const RESET_VALUE: u32 = 0;
}
