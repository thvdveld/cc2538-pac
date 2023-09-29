#[doc = "Register `CIE` reader"]
pub type R = crate::R<CIE_SPEC>;
#[doc = "Register `CIE` writer"]
pub type W = crate::W<CIE_SPEC>;
#[doc = "Field `SUSPENDIE` reader - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type SUSPENDIE_R = crate::BitReader;
#[doc = "Field `SUSPENDIE` writer - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type SUSPENDIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESUMEIE` reader - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type RESUMEIE_R = crate::BitReader;
#[doc = "Field `RESUMEIE` writer - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type RESUMEIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RSTIE` reader - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type RSTIE_R = crate::BitReader;
#[doc = "Field `RSTIE` writer - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type RSTIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SOFIE` reader - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type SOFIE_R = crate::BitReader;
#[doc = "Field `SOFIE` writer - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
pub type SOFIE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn suspendie(&self) -> SUSPENDIE_R {
        SUSPENDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn resumeie(&self) -> RESUMEIE_R {
        RESUMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn sofie(&self) -> SOFIE_R {
        SOFIE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Suspend interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn suspendie(&mut self) -> SUSPENDIE_W<CIE_SPEC, 0> {
        SUSPENDIE_W::new(self)
    }
    #[doc = "Bit 1 - Resume interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn resumeie(&mut self) -> RESUMEIE_W<CIE_SPEC, 1> {
        RESUMEIE_W::new(self)
    }
    #[doc = "Bit 2 - Reset interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn rstie(&mut self) -> RSTIE_W<CIE_SPEC, 2> {
        RSTIE_W::new(self)
    }
    #[doc = "Bit 3 - Start-of-frame interrupt enable 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn sofie(&mut self) -> SOFIE_W<CIE_SPEC, 3> {
        SOFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Common USB interrupt enable mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CIE_SPEC;
impl crate::RegisterSpec for CIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cie::R`](R) reader structure"]
impl crate::Readable for CIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cie::W`](W) writer structure"]
impl crate::Writable for CIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIE to value 0"]
impl crate::Resettable for CIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
