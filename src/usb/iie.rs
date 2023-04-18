#[doc = "Register `IIE` reader"]
pub struct R(crate::R<IIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIE` writer"]
pub struct W(crate::W<IIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIE_SPEC>;
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
impl From<crate::W<IIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP0IE` reader - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
pub type EP0IE_R = crate::BitReader<bool>;
#[doc = "Field `EP0IE` writer - Interrupt enable for endpoint 0 0: Interrupt disabled 1: Interrupt enabled"]
pub type EP0IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIE_SPEC, bool, O>;
#[doc = "Field `INEP1IE` reader - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP1IE_R = crate::BitReader<bool>;
#[doc = "Field `INEP1IE` writer - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIE_SPEC, bool, O>;
#[doc = "Field `INEP2IE` reader - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP2IE_R = crate::BitReader<bool>;
#[doc = "Field `INEP2IE` writer - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIE_SPEC, bool, O>;
#[doc = "Field `INEP3IE` reader - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP3IE_R = crate::BitReader<bool>;
#[doc = "Field `INEP3IE` writer - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIE_SPEC, bool, O>;
#[doc = "Field `INEP4IE` reader - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP4IE_R = crate::BitReader<bool>;
#[doc = "Field `INEP4IE` writer - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIE_SPEC, bool, O>;
#[doc = "Field `INEP5IE` reader - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP5IE_R = crate::BitReader<bool>;
#[doc = "Field `INEP5IE` writer - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type INEP5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IIE_SPEC, bool, O>;
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
    pub fn ep0ie(&mut self) -> EP0IE_W<0> {
        EP0IE_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable for IN endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn inep1ie(&mut self) -> INEP1IE_W<1> {
        INEP1IE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable for IN endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn inep2ie(&mut self) -> INEP2IE_W<2> {
        INEP2IE_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt enable for IN endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn inep3ie(&mut self) -> INEP3IE_W<3> {
        INEP3IE_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt enable for IN endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn inep4ie(&mut self) -> INEP4IE_W<4> {
        INEP4IE_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt enable for IN endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn inep5ie(&mut self) -> INEP5IE_W<5> {
        INEP5IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable mask for IN endpoints 1-5 and endpoint 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iie](index.html) module"]
pub struct IIE_SPEC;
impl crate::RegisterSpec for IIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iie::R](R) reader structure"]
impl crate::Readable for IIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iie::W](W) writer structure"]
impl crate::Writable for IIE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIE to value 0"]
impl crate::Resettable for IIE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
