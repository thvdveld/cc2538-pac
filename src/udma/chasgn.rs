#[doc = "Register `CHASGN` reader"]
pub struct R(crate::R<CHASGN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHASGN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHASGN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHASGN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHASGN` writer"]
pub struct W(crate::W<CHASGN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHASGN_SPEC>;
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
impl From<crate::W<CHASGN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHASGN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHASGN` reader - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
pub type CHASGN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CHASGN` writer - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
pub type CHASGN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHASGN_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
    #[inline(always)]
    pub fn chasgn(&self) -> CHASGN_R {
        CHASGN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
    #[inline(always)]
    #[must_use]
    pub fn chasgn(&mut self) -> CHASGN_W<0> {
        CHASGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel assignment Each bit of the CHASGN register represents the corresponding uDMA channel. Setting a bit selects the secondary channel assignment as specified in the section \"Channel Assignments\"\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chasgn](index.html) module"]
pub struct CHASGN_SPEC;
impl crate::RegisterSpec for CHASGN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chasgn::R](R) reader structure"]
impl crate::Readable for CHASGN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chasgn::W](W) writer structure"]
impl crate::Writable for CHASGN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHASGN to value 0"]
impl crate::Resettable for CHASGN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
