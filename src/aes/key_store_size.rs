#[doc = "Register `KEY_STORE_SIZE` reader"]
pub struct R(crate::R<KEY_STORE_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_STORE_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_STORE_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_STORE_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_STORE_SIZE` writer"]
pub struct W(crate::W<KEY_STORE_SIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_STORE_SIZE_SPEC>;
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
impl From<crate::W<KEY_STORE_SIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_STORE_SIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_SIZE` reader - Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
pub type KEY_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_SIZE` writer - Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
pub type KEY_SIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, KEY_STORE_SIZE_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
    #[inline(always)]
    pub fn key_size(&mut self) -> KEY_SIZE_W<0> {
        KEY_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key store size register This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_store_size](index.html) module"]
pub struct KEY_STORE_SIZE_SPEC;
impl crate::RegisterSpec for KEY_STORE_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_store_size::R](R) reader structure"]
impl crate::Readable for KEY_STORE_SIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_store_size::W](W) writer structure"]
impl crate::Writable for KEY_STORE_SIZE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEY_STORE_SIZE to value 0"]
impl crate::Resettable for KEY_STORE_SIZE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
