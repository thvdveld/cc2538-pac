#[doc = "Register `MTIRQF` reader"]
pub type R = crate::R<MTIRQF_SPEC>;
#[doc = "Register `MTIRQF` writer"]
pub type W = crate::W<MTIRQF_SPEC>;
#[doc = "Field `MACTIMER_PERF` reader - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
pub type MACTIMER_PERF_R = crate::BitReader;
#[doc = "Field `MACTIMER_PERF` writer - Set when the MAC Timer counter would have counted to a value equal to MT_per, but instead wraps to 0"]
pub type MACTIMER_PERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MACTIMER_COMPARE1F` reader - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
pub type MACTIMER_COMPARE1F_R = crate::BitReader;
#[doc = "Field `MACTIMER_COMPARE1F` writer - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
pub type MACTIMER_COMPARE1F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MACTIMER_COMPARE2F` reader - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
pub type MACTIMER_COMPARE2F_R = crate::BitReader;
#[doc = "Field `MACTIMER_COMPARE2F` writer - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
pub type MACTIMER_COMPARE2F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MACTIMER_OVF_PERF` reader - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
pub type MACTIMER_OVF_PERF_R = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_PERF` writer - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
pub type MACTIMER_OVF_PERF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MACTIMER_OVF_COMPARE1F` reader - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
pub type MACTIMER_OVF_COMPARE1F_R = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_COMPARE1F` writer - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
pub type MACTIMER_OVF_COMPARE1F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MACTIMER_OVF_COMPARE2F` reader - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
pub type MACTIMER_OVF_COMPARE2F_R = crate::BitReader;
#[doc = "Field `MACTIMER_OVF_COMPARE2F` writer - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
pub type MACTIMER_OVF_COMPARE2F_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn mactimer_perf(&mut self) -> MACTIMER_PERF_W<MTIRQF_SPEC, 0> {
        MACTIMER_PERF_W::new(self)
    }
    #[doc = "Bit 1 - Set when the MAC Timer counter counts to the value set at MT_cmp1"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_compare1f(&mut self) -> MACTIMER_COMPARE1F_W<MTIRQF_SPEC, 1> {
        MACTIMER_COMPARE1F_W::new(self)
    }
    #[doc = "Bit 2 - Set when the MAC Timer counter counts to the value set at MT_cmp2"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_compare2f(&mut self) -> MACTIMER_COMPARE2F_W<MTIRQF_SPEC, 2> {
        MACTIMER_COMPARE2F_W::new(self)
    }
    #[doc = "Bit 3 - Set when the MAC Timer overflow counter would have counted to a value equal to MTovf_per, but instead wraps to 0"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_perf(&mut self) -> MACTIMER_OVF_PERF_W<MTIRQF_SPEC, 3> {
        MACTIMER_OVF_PERF_W::new(self)
    }
    #[doc = "Bit 4 - Set when the MAC Timer overflow counter counts to the value set at Timer 2 MTovf_cmp1"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_compare1f(&mut self) -> MACTIMER_OVF_COMPARE1F_W<MTIRQF_SPEC, 4> {
        MACTIMER_OVF_COMPARE1F_W::new(self)
    }
    #[doc = "Bit 5 - Set when the MAC Timer overflow counter counts to the value set at MTovf_cmp2"]
    #[inline(always)]
    #[must_use]
    pub fn mactimer_ovf_compare2f(&mut self) -> MACTIMER_OVF_COMPARE2F_W<MTIRQF_SPEC, 5> {
        MACTIMER_OVF_COMPARE2F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "MAC Timer interrupt flags\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtirqf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtirqf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIRQF_SPEC;
impl crate::RegisterSpec for MTIRQF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtirqf::R`](R) reader structure"]
impl crate::Readable for MTIRQF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mtirqf::W`](W) writer structure"]
impl crate::Writable for MTIRQF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTIRQF to value 0"]
impl crate::Resettable for MTIRQF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
