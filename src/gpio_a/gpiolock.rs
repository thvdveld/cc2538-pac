#[doc = "Register `GPIOLOCK` reader"]
pub type R = crate::R<GPIOLOCK_SPEC>;
#[doc = "Register `GPIOLOCK` writer"]
pub type W = crate::W<GPIOLOCK_SPEC>;
#[doc = "Field `LOCK` reader - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
pub type LOCK_R = crate::FieldReader<u32>;
#[doc = "Field `LOCK` writer - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<GPIOLOCK_SPEC> {
        LOCK_W::new(self, 0)
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
#[doc = "A write of the value 0x4C4F434B to the GPIOLOCK register unlocks the GPIO commit register (GPIOCR) for write access. A write of any other value reapplies the lock, preventing any register updates. Any write to the commit register (GPIOCR) causes the lock register to be locked.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiolock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiolock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOLOCK_SPEC;
impl crate::RegisterSpec for GPIOLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiolock::R`](R) reader structure"]
impl crate::Readable for GPIOLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpiolock::W`](W) writer structure"]
impl crate::Writable for GPIOLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOLOCK to value 0x01"]
impl crate::Resettable for GPIOLOCK_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
