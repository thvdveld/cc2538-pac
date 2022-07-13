#[doc = "Register `STCS` reader"]
pub struct R(crate::R<STCS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCS` writer"]
pub struct W(crate::W<STCS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCS_SPEC>;
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
impl From<crate::W<STCS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VALID` reader - Capture valid flag Set to 1 when capture value in STCV has been updated Clear explicitly to allow new capture"]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `VALID` writer - Capture valid flag Set to 1 when capture value in STCV has been updated Clear explicitly to allow new capture"]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, STCS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Capture valid flag Set to 1 when capture value in STCV has been updated Clear explicitly to allow new capture"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture valid flag Set to 1 when capture value in STCV has been updated Clear explicitly to allow new capture"]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W<0> {
        VALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sleep Timer Capture status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcs](index.html) module"]
pub struct STCS_SPEC;
impl crate::RegisterSpec for STCS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcs::R](R) reader structure"]
impl crate::Readable for STCS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcs::W](W) writer structure"]
impl crate::Writable for STCS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STCS to value 0"]
impl crate::Resettable for STCS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
