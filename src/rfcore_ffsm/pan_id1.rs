#[doc = "Register `PAN_ID1` reader"]
pub struct R(crate::R<PAN_ID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAN_ID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAN_ID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAN_ID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAN_ID1` writer"]
pub struct W(crate::W<PAN_ID1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAN_ID1_SPEC>;
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
impl From<crate::W<PAN_ID1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAN_ID1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAN_ID1` reader - PAN_ID\\[15:8\\]
The PAN ID used during destination address filtering"]
pub type PAN_ID1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PAN_ID1` writer - PAN_ID\\[15:8\\]
The PAN ID used during destination address filtering"]
pub type PAN_ID1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PAN_ID1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - PAN_ID\\[15:8\\]
The PAN ID used during destination address filtering"]
    #[inline(always)]
    pub fn pan_id1(&self) -> PAN_ID1_R {
        PAN_ID1_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PAN_ID\\[15:8\\]
The PAN ID used during destination address filtering"]
    #[inline(always)]
    #[must_use]
    pub fn pan_id1(&mut self) -> PAN_ID1_W<0> {
        PAN_ID1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pan_id1](index.html) module"]
pub struct PAN_ID1_SPEC;
impl crate::RegisterSpec for PAN_ID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pan_id1::R](R) reader structure"]
impl crate::Readable for PAN_ID1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pan_id1::W](W) writer structure"]
impl crate::Writable for PAN_ID1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAN_ID1 to value 0"]
impl crate::Resettable for PAN_ID1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
