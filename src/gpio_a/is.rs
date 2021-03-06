#[doc = "Register `IS` reader"]
pub struct R(crate::R<IS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IS` writer"]
pub struct W(crate::W<IS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IS_SPEC>;
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
impl From<crate::W<IS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IS` reader - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
pub type IS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IS` writer - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
pub type IS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bits set: Level on corresponding pin is detected Bits cleared: Edge on corresponding pin is detected"]
    #[inline(always)]
    pub fn is(&mut self) -> IS_W<0> {
        IS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The IS register is the interrupt sense register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [is](index.html) module"]
pub struct IS_SPEC;
impl crate::RegisterSpec for IS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [is::R](R) reader structure"]
impl crate::Readable for IS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [is::W](W) writer structure"]
impl crate::Writable for IS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IS to value 0"]
impl crate::Resettable for IS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
