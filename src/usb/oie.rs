#[doc = "Register `OIE` reader"]
pub struct R(crate::R<OIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OIE` writer"]
pub struct W(crate::W<OIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OIE_SPEC>;
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
impl From<crate::W<OIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTEP5IE` reader - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP5IE_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP5IE` writer - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP5IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OIE_SPEC, bool, O>;
#[doc = "Field `OUTEP4IE` reader - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP4IE_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP4IE` writer - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP4IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OIE_SPEC, bool, O>;
#[doc = "Field `OUTEP3IE` reader - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP3IE_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP3IE` writer - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OIE_SPEC, bool, O>;
#[doc = "Field `OUTEP2IE` reader - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP2IE_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP2IE` writer - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OIE_SPEC, bool, O>;
#[doc = "Field `OUTEP1IE` reader - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP1IE_R = crate::BitReader<bool>;
#[doc = "Field `OUTEP1IE` writer - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OIE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep5ie(&self) -> OUTEP5IE_R {
        OUTEP5IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep4ie(&self) -> OUTEP4IE_R {
        OUTEP4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep3ie(&self) -> OUTEP3IE_R {
        OUTEP3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep2ie(&self) -> OUTEP2IE_R {
        OUTEP2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep1ie(&self) -> OUTEP1IE_R {
        OUTEP1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep5ie(&mut self) -> OUTEP5IE_W<5> {
        OUTEP5IE_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep4ie(&mut self) -> OUTEP4IE_W<4> {
        OUTEP4IE_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep3ie(&mut self) -> OUTEP3IE_W<3> {
        OUTEP3IE_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep2ie(&mut self) -> OUTEP2IE_W<2> {
        OUTEP2IE_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep1ie(&mut self) -> OUTEP1IE_W<1> {
        OUTEP1IE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable mask for OUT endpoints 1-5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oie](index.html) module"]
pub struct OIE_SPEC;
impl crate::RegisterSpec for OIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oie::R](R) reader structure"]
impl crate::Readable for OIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oie::W](W) writer structure"]
impl crate::Writable for OIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OIE to value 0"]
impl crate::Resettable for OIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
