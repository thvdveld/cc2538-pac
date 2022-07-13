#[doc = "Register `ALENGTH` reader"]
pub struct R(crate::R<ALENGTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALENGTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALENGTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALENGTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ALENGTH` writer"]
pub struct W(crate::W<ALENGTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALENGTH_SPEC>;
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
impl From<crate::W<ALENGTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALENGTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALENGTH` reader - This register specifies the length (in 32-bit words) of Vector A."]
pub type ALENGTH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ALENGTH` writer - This register specifies the length (in 32-bit words) of Vector A."]
pub type ALENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ALENGTH_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - This register specifies the length (in 32-bit words) of Vector A."]
    #[inline(always)]
    pub fn alength(&self) -> ALENGTH_R {
        ALENGTH_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register specifies the length (in 32-bit words) of Vector A."]
    #[inline(always)]
    pub fn alength(&mut self) -> ALENGTH_W<0> {
        ALENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PKA vector A length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alength](index.html) module"]
pub struct ALENGTH_SPEC;
impl crate::RegisterSpec for ALENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [alength::R](R) reader structure"]
impl crate::Readable for ALENGTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [alength::W](W) writer structure"]
impl crate::Writable for ALENGTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALENGTH to value 0"]
impl crate::Resettable for ALENGTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
