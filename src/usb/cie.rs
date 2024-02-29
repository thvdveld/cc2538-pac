#[doc = "Register `CIE` reader"]
pub type R = crate::R<CieSpec>;
#[doc = "Register `CIE` writer"]
pub type W = crate::W<CieSpec>;
#[doc = "Field `SUSPENDIE` reader - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type SuspendieR = crate::BitReader;
#[doc = "Field `SUSPENDIE` writer - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type SuspendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUMEIE` reader - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type ResumeieR = crate::BitReader;
#[doc = "Field `RESUMEIE` writer - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type ResumeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIE` reader - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type RstieR = crate::BitReader;
#[doc = "Field `RSTIE` writer - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type RstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFIE` reader - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type SofieR = crate::BitReader;
#[doc = "Field `SOFIE` writer - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type SofieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn suspendie(&self) -> SuspendieR {
        SuspendieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn resumeie(&self) -> ResumeieR {
        ResumeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn rstie(&self) -> RstieR {
        RstieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn sofie(&self) -> SofieR {
        SofieR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn suspendie(&mut self) -> SuspendieW<CieSpec> {
        SuspendieW::new(self, 0)
    }
    #[doc = "Bit 1 - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn resumeie(&mut self) -> ResumeieW<CieSpec> {
        ResumeieW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rstie(&mut self) -> RstieW<CieSpec> {
        RstieW::new(self, 2)
    }
    #[doc = "Bit 3 - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SofieW<CieSpec> {
        SofieW::new(self, 3)
    }
}
#[doc = "Common USB interrupt enable mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CieSpec;
impl crate::RegisterSpec for CieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cie::R`](R) reader structure"]
impl crate::Readable for CieSpec {}
#[doc = "`write(|w| ..)` method takes [`cie::W`](W) writer structure"]
impl crate::Writable for CieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CIE to value 0"]
impl crate::Resettable for CieSpec {
    const RESET_VALUE: u32 = 0;
}
