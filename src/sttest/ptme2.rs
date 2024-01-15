#[doc = "Register `PTME2` reader"]
pub type R = crate::R<PTME2_SPEC>;
#[doc = "Register `PTME2` writer"]
pub type W = crate::W<PTME2_SPEC>;
#[doc = "Field `I2C0TME` reader - I2C 0 test mode enable"]
pub type I2C0TME_R = crate::BitReader;
#[doc = "Field `I2C0TME` writer - I2C 0 test mode enable"]
pub type I2C0TME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T0TME` reader - Timer0 test mode enable"]
pub type T0TME_R = crate::BitReader;
#[doc = "Field `T0TME` writer - Timer0 test mode enable"]
pub type T0TME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T1TME` reader - Timer1 test mode enable"]
pub type T1TME_R = crate::BitReader;
#[doc = "Field `T1TME` writer - Timer1 test mode enable"]
pub type T1TME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTTME` reader - MacTimer test mode enable"]
pub type MTTME_R = crate::BitReader;
#[doc = "Field `MTTME` writer - MacTimer test mode enable"]
pub type MTTME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `T3TME` reader - Timer3 test mode enable"]
pub type T3TME_R = crate::BitReader;
#[doc = "Field `T3TME` writer - Timer3 test mode enable"]
pub type T3TME_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C 0 test mode enable"]
    #[inline(always)]
    pub fn i2c0tme(&self) -> I2C0TME_R {
        I2C0TME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Timer0 test mode enable"]
    #[inline(always)]
    pub fn t0tme(&self) -> T0TME_R {
        T0TME_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer1 test mode enable"]
    #[inline(always)]
    pub fn t1tme(&self) -> T1TME_R {
        T1TME_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MacTimer test mode enable"]
    #[inline(always)]
    pub fn mttme(&self) -> MTTME_R {
        MTTME_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer3 test mode enable"]
    #[inline(always)]
    pub fn t3tme(&self) -> T3TME_R {
        T3TME_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C 0 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0tme(&mut self) -> I2C0TME_W<PTME2_SPEC> {
        I2C0TME_W::new(self, 0)
    }
    #[doc = "Bit 16 - Timer0 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn t0tme(&mut self) -> T0TME_W<PTME2_SPEC> {
        T0TME_W::new(self, 16)
    }
    #[doc = "Bit 17 - Timer1 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn t1tme(&mut self) -> T1TME_W<PTME2_SPEC> {
        T1TME_W::new(self, 17)
    }
    #[doc = "Bit 18 - MacTimer test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mttme(&mut self) -> MTTME_W<PTME2_SPEC> {
        MTTME_W::new(self, 18)
    }
    #[doc = "Bit 19 - Timer3 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn t3tme(&mut self) -> T3TME_W<PTME2_SPEC> {
        T3TME_W::new(self, 19)
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
#[doc = "Peripheral test mode enable 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptme2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptme2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PTME2_SPEC;
impl crate::RegisterSpec for PTME2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptme2::R`](R) reader structure"]
impl crate::Readable for PTME2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ptme2::W`](W) writer structure"]
impl crate::Writable for PTME2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTME2 to value 0"]
impl crate::Resettable for PTME2_SPEC {
    const RESET_VALUE: u32 = 0;
}
