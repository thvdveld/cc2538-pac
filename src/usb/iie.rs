#[doc = "Register `IIE` reader"]
pub type R = crate::R<IIE_SPEC>;
#[doc = "Register `IIE` writer"]
pub type W = crate::W<IIE_SPEC>;
#[doc = "Field `EP0IE` reader - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
pub type EP0IE_R = crate::BitReader;
#[doc = "Field `EP0IE` writer - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
pub type EP0IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEP1IE` reader - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP1IE_R = crate::BitReader;
#[doc = "Field `INEP1IE` writer - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP1IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEP2IE` reader - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP2IE_R = crate::BitReader;
#[doc = "Field `INEP2IE` writer - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP2IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEP3IE` reader - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP3IE_R = crate::BitReader;
#[doc = "Field `INEP3IE` writer - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP3IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEP4IE` reader - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP4IE_R = crate::BitReader;
#[doc = "Field `INEP4IE` writer - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP4IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `INEP5IE` reader - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP5IE_R = crate::BitReader;
#[doc = "Field `INEP5IE` writer - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP5IE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn ep0ie(&self) -> EP0IE_R {
        EP0IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep1ie(&self) -> INEP1IE_R {
        INEP1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep2ie(&self) -> INEP2IE_R {
        INEP2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep3ie(&self) -> INEP3IE_R {
        INEP3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep4ie(&self) -> INEP4IE_R {
        INEP4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn inep5ie(&self) -> INEP5IE_R {
        INEP5IE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ep0ie(&mut self) -> EP0IE_W<IIE_SPEC, 0> {
        EP0IE_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn inep1ie(&mut self) -> INEP1IE_W<IIE_SPEC, 1> {
        INEP1IE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn inep2ie(&mut self) -> INEP2IE_W<IIE_SPEC, 2> {
        INEP2IE_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn inep3ie(&mut self) -> INEP3IE_W<IIE_SPEC, 3> {
        INEP3IE_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn inep4ie(&mut self) -> INEP4IE_W<IIE_SPEC, 4> {
        INEP4IE_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn inep5ie(&mut self) -> INEP5IE_W<IIE_SPEC, 5> {
        INEP5IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt enable mask for IN endpoints 1-5 and endpoint 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IIE_SPEC;
impl crate::RegisterSpec for IIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iie::R`](R) reader structure"]
impl crate::Readable for IIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iie::W`](W) writer structure"]
impl crate::Writable for IIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIE to value 0"]
impl crate::Resettable for IIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
