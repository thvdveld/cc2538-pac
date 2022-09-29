#[doc = "Register `MTIRQM` reader"]
pub struct R(crate::R<MTIRQM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIRQM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIRQM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIRQM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTIRQM` writer"]
pub struct W(crate::W<MTIRQM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTIRQM_SPEC>;
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
impl From<crate::W<MTIRQM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTIRQM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACTIMER_PERM` reader - Enables the MACTIMER_PER interrupt"]
pub type MACTIMER_PERM_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_PERM` writer - Enables the MACTIMER_PER interrupt"]
pub type MACTIMER_PERM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTIRQM_SPEC, bool, O>;
#[doc = "Field `MACTIMER_COMPARE1M` reader - Enables the MACTIMER_COMPARE1 interrupt"]
pub type MACTIMER_COMPARE1M_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_COMPARE1M` writer - Enables the MACTIMER_COMPARE1 interrupt"]
pub type MACTIMER_COMPARE1M_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTIRQM_SPEC, bool, O>;
#[doc = "Field `MACTIMER_COMPARE2M` reader - Enables the MACTIMER_COMPARE2 interrupt"]
pub type MACTIMER_COMPARE2M_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_COMPARE2M` writer - Enables the MACTIMER_COMPARE2 interrupt"]
pub type MACTIMER_COMPARE2M_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTIRQM_SPEC, bool, O>;
#[doc = "Field `MACTIMER_OVF_PERM` reader - Enables the MACTIMER_OVF_PER interrupt"]
pub type MACTIMER_OVF_PERM_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_OVF_PERM` writer - Enables the MACTIMER_OVF_PER interrupt"]
pub type MACTIMER_OVF_PERM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTIRQM_SPEC, bool, O>;
#[doc = "Field `MACTIMER_OVF_COMPARE1M` reader - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
pub type MACTIMER_OVF_COMPARE1M_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_OVF_COMPARE1M` writer - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
pub type MACTIMER_OVF_COMPARE1M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MTIRQM_SPEC, bool, O>;
#[doc = "Field `MACTIMER_OVF_COMPARE2M` reader - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
pub type MACTIMER_OVF_COMPARE2M_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_OVF_COMPARE2M` writer - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
pub type MACTIMER_OVF_COMPARE2M_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MTIRQM_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enables the MACTIMER_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_perm(&self) -> MACTIMER_PERM_R {
        MACTIMER_PERM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the MACTIMER_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare1m(&self) -> MACTIMER_COMPARE1M_R {
        MACTIMER_COMPARE1M_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables the MACTIMER_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare2m(&self) -> MACTIMER_COMPARE2M_R {
        MACTIMER_COMPARE2M_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables the MACTIMER_OVF_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_perm(&self) -> MACTIMER_OVF_PERM_R {
        MACTIMER_OVF_PERM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1m(&self) -> MACTIMER_OVF_COMPARE1M_R {
        MACTIMER_OVF_COMPARE1M_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2m(&self) -> MACTIMER_OVF_COMPARE2M_R {
        MACTIMER_OVF_COMPARE2M_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the MACTIMER_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_perm(&mut self) -> MACTIMER_PERM_W<0> {
        MACTIMER_PERM_W::new(self)
    }
    #[doc = "Bit 1 - Enables the MACTIMER_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare1m(&mut self) -> MACTIMER_COMPARE1M_W<1> {
        MACTIMER_COMPARE1M_W::new(self)
    }
    #[doc = "Bit 2 - Enables the MACTIMER_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_compare2m(&mut self) -> MACTIMER_COMPARE2M_W<2> {
        MACTIMER_COMPARE2M_W::new(self)
    }
    #[doc = "Bit 3 - Enables the MACTIMER_OVF_PER interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_perm(&mut self) -> MACTIMER_OVF_PERM_W<3> {
        MACTIMER_OVF_PERM_W::new(self)
    }
    #[doc = "Bit 4 - Enables the MACTIMER_OVF_COMPARE1 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1m(&mut self) -> MACTIMER_OVF_COMPARE1M_W<4> {
        MACTIMER_OVF_COMPARE1M_W::new(self)
    }
    #[doc = "Bit 5 - Enables the MACTIMER_OVF_COMPARE2 interrupt"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2m(&mut self) -> MACTIMER_OVF_COMPARE2M_W<5> {
        MACTIMER_OVF_COMPARE2M_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtirqm](index.html) module"]
pub struct MTIRQM_SPEC;
impl crate::RegisterSpec for MTIRQM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtirqm::R](R) reader structure"]
impl crate::Readable for MTIRQM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtirqm::W](W) writer structure"]
impl crate::Writable for MTIRQM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTIRQM to value 0"]
impl crate::Resettable for MTIRQM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
