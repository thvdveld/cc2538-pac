#[doc = "Register `ICR` reader"]
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICR` writer"]
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TATOCINT` reader - GPTM Timer A time-out interrupt clear"]
pub type TATOCINT_R = crate::BitReader<bool>;
#[doc = "Field `TATOCINT` writer - GPTM Timer A time-out interrupt clear"]
pub type TATOCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CAMCINT` reader - GPTM Timer A capture match interrupt clear"]
pub type CAMCINT_R = crate::BitReader<bool>;
#[doc = "Field `CAMCINT` writer - GPTM Timer A capture match interrupt clear"]
pub type CAMCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CAECINT` reader - GPTM Timer A capture event Interrupt clear"]
pub type CAECINT_R = crate::BitReader<bool>;
#[doc = "Field `CAECINT` writer - GPTM Timer A capture event Interrupt clear"]
pub type CAECINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TAMCINT` reader - GPTM Timer A match interrupt clear"]
pub type TAMCINT_R = crate::BitReader<bool>;
#[doc = "Field `TAMCINT` writer - GPTM Timer A match interrupt clear"]
pub type TAMCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TBTOCINT` reader - GPTM Timer B time-out interrupt clear"]
pub type TBTOCINT_R = crate::BitReader<bool>;
#[doc = "Field `TBTOCINT` writer - GPTM Timer B time-out interrupt clear"]
pub type TBTOCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CBMCINT` reader - GPTM Timer B capture match interrupt clear"]
pub type CBMCINT_R = crate::BitReader<bool>;
#[doc = "Field `CBMCINT` writer - GPTM Timer B capture match interrupt clear"]
pub type CBMCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `CBECINT` reader - GPTM Timer B capture event Interrupt clear"]
pub type CBECINT_R = crate::BitReader<bool>;
#[doc = "Field `CBECINT` writer - GPTM Timer B capture event Interrupt clear"]
pub type CBECINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `TBMCINT` reader - GPTM Timer B match interrupt clear"]
pub type TBMCINT_R = crate::BitReader<bool>;
#[doc = "Field `TBMCINT` writer - GPTM Timer B match interrupt clear"]
pub type TBMCINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
#[doc = "Field `WUECINT` reader - GPTM write update error interrupt clear"]
pub type WUECINT_R = crate::BitReader<bool>;
#[doc = "Field `WUECINT` writer - GPTM write update error interrupt clear"]
pub type WUECINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt clear"]
    #[inline(always)]
    pub fn tatocint(&self) -> TATOCINT_R {
        TATOCINT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt clear"]
    #[inline(always)]
    pub fn camcint(&self) -> CAMCINT_R {
        CAMCINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event Interrupt clear"]
    #[inline(always)]
    pub fn caecint(&self) -> CAECINT_R {
        CAECINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt clear"]
    #[inline(always)]
    pub fn tamcint(&self) -> TAMCINT_R {
        TAMCINT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt clear"]
    #[inline(always)]
    pub fn tbtocint(&self) -> TBTOCINT_R {
        TBTOCINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt clear"]
    #[inline(always)]
    pub fn cbmcint(&self) -> CBMCINT_R {
        CBMCINT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event Interrupt clear"]
    #[inline(always)]
    pub fn cbecint(&self) -> CBECINT_R {
        CBECINT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B match interrupt clear"]
    #[inline(always)]
    pub fn tbmcint(&self) -> TBMCINT_R {
        TBMCINT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - GPTM write update error interrupt clear"]
    #[inline(always)]
    pub fn wuecint(&self) -> WUECINT_R {
        WUECINT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt clear"]
    #[inline(always)]
    pub fn tatocint(&mut self) -> TATOCINT_W<0> {
        TATOCINT_W::new(self)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt clear"]
    #[inline(always)]
    pub fn camcint(&mut self) -> CAMCINT_W<1> {
        CAMCINT_W::new(self)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event Interrupt clear"]
    #[inline(always)]
    pub fn caecint(&mut self) -> CAECINT_W<2> {
        CAECINT_W::new(self)
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt clear"]
    #[inline(always)]
    pub fn tamcint(&mut self) -> TAMCINT_W<4> {
        TAMCINT_W::new(self)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt clear"]
    #[inline(always)]
    pub fn tbtocint(&mut self) -> TBTOCINT_W<8> {
        TBTOCINT_W::new(self)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt clear"]
    #[inline(always)]
    pub fn cbmcint(&mut self) -> CBMCINT_W<9> {
        CBMCINT_W::new(self)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event Interrupt clear"]
    #[inline(always)]
    pub fn cbecint(&mut self) -> CBECINT_W<10> {
        CBECINT_W::new(self)
    }
    #[doc = "Bit 11 - GPTM Timer B match interrupt clear"]
    #[inline(always)]
    pub fn tbmcint(&mut self) -> TBMCINT_W<11> {
        TBMCINT_W::new(self)
    }
    #[doc = "Bit 16 - GPTM write update error interrupt clear"]
    #[inline(always)]
    pub fn wuecint(&mut self) -> WUECINT_W<16> {
        WUECINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTM interrupt clear This register is used to clear the status bits in the RIS and MIS registers. Writing 1 to a bit clears the corresponding bit in the RIS and MIS registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](index.html) module"]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icr::R](R) reader structure"]
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icr::W](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
