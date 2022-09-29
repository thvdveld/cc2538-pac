#[doc = "Register `CMPCTL` reader"]
pub struct R(crate::R<CMPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPCTL` writer"]
pub struct W(crate::W<CMPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPCTL_SPEC>;
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
impl From<crate::W<CMPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTPUT` reader - Comparator output"]
pub type OUTPUT_R = crate::BitReader<bool>;
#[doc = "Field `EN` reader - Comparator enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Comparator enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Comparator output"]
    #[inline(always)]
    pub fn output(&self) -> OUTPUT_R {
        OUTPUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Comparator enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<1> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog comparator control and status register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpctl](index.html) module"]
pub struct CMPCTL_SPEC;
impl crate::RegisterSpec for CMPCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpctl::R](R) reader structure"]
impl crate::Readable for CMPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpctl::W](W) writer structure"]
impl crate::Writable for CMPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPCTL to value 0"]
impl crate::Resettable for CMPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
