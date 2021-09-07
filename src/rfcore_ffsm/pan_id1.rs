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
pub struct PAN_ID1_R(crate::FieldReader<u8, u8>);
impl PAN_ID1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PAN_ID1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAN_ID1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAN_ID1` writer - PAN_ID\\[15:8\\]
The PAN ID used during destination address filtering"]
pub struct PAN_ID1_W<'a> {
    w: &'a mut W,
}
impl<'a> PAN_ID1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
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
    pub fn pan_id1(&mut self) -> PAN_ID1_W {
        PAN_ID1_W { w: self }
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
}
#[doc = "`reset()` method sets PAN_ID1 to value 0"]
impl crate::Resettable for PAN_ID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
