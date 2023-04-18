#[doc = "Register `MTIRQF` reader"]
pub struct R(crate::R<MTIRQF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTIRQF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTIRQF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTIRQF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTIRQF` writer"]
pub struct W(crate::W<MTIRQF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTIRQF_SPEC>;
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
impl From<crate::W<MTIRQF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTIRQF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACTIMER_PERF` reader - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
pub type MACTIMER_PERF_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_PERF` writer - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
pub type MACTIMER_PERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTIRQF_SPEC, bool, O>;
#[doc = "Field `MACTIMER_COMPARE1F` reader - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
pub type MACTIMER_COMPARE1F_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_COMPARE1F` writer - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
pub type MACTIMER_COMPARE1F_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTIRQF_SPEC, bool, O>;
#[doc = "Field `MACTIMER_COMPARE2F` reader - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
pub type MACTIMER_COMPARE2F_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_COMPARE2F` writer - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
pub type MACTIMER_COMPARE2F_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTIRQF_SPEC, bool, O>;
#[doc = "Field `MACTIMER_OVF_PERF` reader - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
pub type MACTIMER_OVF_PERF_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_OVF_PERF` writer - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
pub type MACTIMER_OVF_PERF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MTIRQF_SPEC, bool, O>;
#[doc = "Field `MACTIMER_OVF_COMPARE1F` reader - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
pub type MACTIMER_OVF_COMPARE1F_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_OVF_COMPARE1F` writer - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
pub type MACTIMER_OVF_COMPARE1F_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MTIRQF_SPEC, bool, O>;
#[doc = "Field `MACTIMER_OVF_COMPARE2F` reader - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
pub type MACTIMER_OVF_COMPARE2F_R = crate::BitReader<bool>;
#[doc = "Field `MACTIMER_OVF_COMPARE2F` writer - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
pub type MACTIMER_OVF_COMPARE2F_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MTIRQF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
    #[inline(always)]
    pub fn mactimer_perf(&self) -> MACTIMER_PERF_R {
        MACTIMER_PERF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
    #[inline(always)]
    pub fn mactimer_compare1f(&self) -> MACTIMER_COMPARE1F_R {
        MACTIMER_COMPARE1F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
    #[inline(always)]
    pub fn mactimer_compare2f(&self) -> MACTIMER_COMPARE2F_R {
        MACTIMER_COMPARE2F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
    #[inline(always)]
    pub fn mactimer_ovf_perf(&self) -> MACTIMER_OVF_PERF_R {
        MACTIMER_OVF_PERF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
    #[inline(always)]
    pub fn mactimer_ovf_compare1f(&self) -> MACTIMER_OVF_COMPARE1F_R {
        MACTIMER_OVF_COMPARE1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
    #[inline(always)]
    pub fn mactimer_ovf_compare2f(&self) -> MACTIMER_OVF_COMPARE2F_R {
        MACTIMER_OVF_COMPARE2F_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_perf(&mut self) -> MACTIMER_PERF_W<0> {
        MACTIMER_PERF_W::new(self)
    }
    #[doc = "Bit 1 - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_compare1f(&mut self) -> MACTIMER_COMPARE1F_W<1> {
        MACTIMER_COMPARE1F_W::new(self)
    }
    #[doc = "Bit 2 - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_compare2f(&mut self) -> MACTIMER_COMPARE2F_W<2> {
        MACTIMER_COMPARE2F_W::new(self)
    }
    #[doc = "Bit 3 - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_perf(&mut self) -> MACTIMER_OVF_PERF_W<3> {
        MACTIMER_OVF_PERF_W::new(self)
    }
    #[doc = "Bit 4 - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_compare1f(&mut self) -> MACTIMER_OVF_COMPARE1F_W<4> {
        MACTIMER_OVF_COMPARE1F_W::new(self)
    }
    #[doc = "Bit 5 - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_compare2f(&mut self) -> MACTIMER_OVF_COMPARE2F_W<5> {
        MACTIMER_OVF_COMPARE2F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC Timer interrupt flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtirqf](index.html) module"]
pub struct MTIRQF_SPEC;
impl crate::RegisterSpec for MTIRQF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtirqf::R](R) reader structure"]
impl crate::Readable for MTIRQF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtirqf::W](W) writer structure"]
impl crate::Writable for MTIRQF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTIRQF to value 0"]
impl crate::Resettable for MTIRQF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
