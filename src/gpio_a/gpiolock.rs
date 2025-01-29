#[doc = "Register `GPIOLOCK` reader"]
pub type R = crate::R<GpiolockSpec>;
#[doc = "Register `GPIOLOCK` writer"]
pub type W = crate::W<GpiolockSpec>;
#[doc = "Field `LOCK` reader - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
pub type LockR = crate::FieldReader<u32>;
#[doc = "Field `LOCK` writer - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<GpiolockSpec> {
        LockW::new(self, 0)
    }
}
#[doc = "A write of the value 0x4C4F434B to the GPIOLOCK register unlocks the GPIO commit register (GPIOCR) for write access. A write of any other value reapplies the lock, preventing any register updates. Any write to the commit register (GPIOCR) causes the lock register to be locked.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiolock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiolock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiolockSpec;
impl crate::RegisterSpec for GpiolockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiolock::R`](R) reader structure"]
impl crate::Readable for GpiolockSpec {}
#[doc = "`write(|w| ..)` method takes [`gpiolock::W`](W) writer structure"]
impl crate::Writable for GpiolockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOLOCK to value 0x01"]
impl crate::Resettable for GpiolockSpec {
    const RESET_VALUE: u32 = 0x01;
}
