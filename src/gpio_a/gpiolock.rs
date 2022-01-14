#[doc = "Register `GPIOLOCK` reader"]
pub struct R(crate::R<GPIOLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIOLOCK` writer"]
pub struct W(crate::W<GPIOLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIOLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPIOLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIOLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
pub struct LOCK_R(crate::FieldReader<u32, u32>);
impl LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - A read of this register returns the following values: Locked: 0x00000001 Unlocked: 0x00000000"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "A write of the value 0x4C4F434B to the GPIOLOCK register unlocks the GPIO commit register (GPIOCR) for write access. A write of any other value reapplies the lock, preventing any register updates. Any write to the commit register (GPIOCR) causes the lock register to be locked.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiolock](index.html) module"]
pub struct GPIOLOCK_SPEC;
impl crate::RegisterSpec for GPIOLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpiolock::R](R) reader structure"]
impl crate::Readable for GPIOLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpiolock::W](W) writer structure"]
impl crate::Writable for GPIOLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIOLOCK to value 0x01"]
impl crate::Resettable for GPIOLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
