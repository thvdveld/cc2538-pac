#[doc = "Register `HASH_DIGEST_F` reader"]
pub struct R(crate::R<HASH_DIGEST_F_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_DIGEST_F_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_DIGEST_F_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_DIGEST_F_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_DIGEST_F` writer"]
pub struct W(crate::W<HASH_DIGEST_F_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_DIGEST_F_SPEC>;
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
impl From<crate::W<HASH_DIGEST_F_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_DIGEST_F_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HASH_DIGEST` reader - HASH_DIGEST\\[191:160\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the new_hash bit in the HASH_MODE register is 0 when starting a hash session). New hash: When initiating a new hash session (the new_hash bit in the HASH_MODE register is high), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
pub struct HASH_DIGEST_R(crate::FieldReader<u32, u32>);
impl HASH_DIGEST_R {
    pub(crate) fn new(bits: u32) -> Self {
        HASH_DIGEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HASH_DIGEST_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HASH_DIGEST` writer - HASH_DIGEST\\[191:160\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the new_hash bit in the HASH_MODE register is 0 when starting a hash session). New hash: When initiating a new hash session (the new_hash bit in the HASH_MODE register is high), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
pub struct HASH_DIGEST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH_DIGEST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits |= value as u32;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - HASH_DIGEST\\[191:160\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the new_hash bit in the HASH_MODE register is 0 when starting a hash session). New hash: When initiating a new hash session (the new_hash bit in the HASH_MODE register is high), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline(always)]
    pub fn hash_digest(&self) -> HASH_DIGEST_R {
        HASH_DIGEST_R::new(self.bits as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - HASH_DIGEST\\[191:160\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the new_hash bit in the HASH_MODE register is 0 when starting a hash session). New hash: When initiating a new hash session (the new_hash bit in the HASH_MODE register is high), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline(always)]
    pub fn hash_digest(&mut self) -> HASH_DIGEST_W {
        HASH_DIGEST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_digest_f](index.html) module"]
pub struct HASH_DIGEST_F_SPEC;
impl crate::RegisterSpec for HASH_DIGEST_F_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_digest_f::R](R) reader structure"]
impl crate::Readable for HASH_DIGEST_F_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_digest_f::W](W) writer structure"]
impl crate::Writable for HASH_DIGEST_F_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_DIGEST_F to value 0"]
impl crate::Resettable for HASH_DIGEST_F_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
