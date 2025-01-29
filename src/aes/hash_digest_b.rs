#[doc = "Register `HASH_DIGEST_B` reader"]
pub type R = crate::R<HashDigestBSpec>;
#[doc = "Register `HASH_DIGEST_B` writer"]
pub type W = crate::W<HashDigestBSpec>;
#[doc = "Field `HASH_DIGEST` reader - HASH_DIGEST\\[63:32\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the new_hash bit in the HASH_MODE register is 0 when starting a hash session). New hash: When initiating a new hash session (the new_hash bit in the HASH_MODE register is high), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
pub type HashDigestR = crate::FieldReader<u32>;
#[doc = "Field `HASH_DIGEST` writer - HASH_DIGEST\\[63:32\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the new_hash bit in the HASH_MODE register is 0 when starting a hash session). New hash: When initiating a new hash session (the new_hash bit in the HASH_MODE register is high), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
pub type HashDigestW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HASH_DIGEST\\[63:32\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the new_hash bit in the HASH_MODE register is 0 when starting a hash session). New hash: When initiating a new hash session (the new_hash bit in the HASH_MODE register is high), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline(always)]
    pub fn hash_digest(&self) -> HashDigestR {
        HashDigestR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HASH_DIGEST\\[63:32\\]
Hash digest registers Write operation: Continued hash: These registers should be written with the context data, before the start of a resumed hash session (the new_hash bit in the HASH_MODE register is 0 when starting a hash session). New hash: When initiating a new hash session (the new_hash bit in the HASH_MODE register is high), the internal digest registers are automatically set to the SHA-256 algorithm constant and these register should not be written. Reading from these registers provides the intermediate hash result (non-final hash operation) or the final hash result (final hash operation) after data processing."]
    #[inline(always)]
    pub fn hash_digest(&mut self) -> HashDigestW<HashDigestBSpec> {
        HashDigestW::new(self, 0)
    }
}
#[doc = "Hash digest registers The hash digest registers consist of eight 32-bit registers, named HASH_DIGEST_A to HASH_DIGEST_H. After processing a message, the output digest can be read from these registers. These registers can be written with an intermediate hash result for continued hash operations.\n\nYou can [`read`](crate::Reg::read) this register and get [`hash_digest_b::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_digest_b::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashDigestBSpec;
impl crate::RegisterSpec for HashDigestBSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_digest_b::R`](R) reader structure"]
impl crate::Readable for HashDigestBSpec {}
#[doc = "`write(|w| ..)` method takes [`hash_digest_b::W`](W) writer structure"]
impl crate::Writable for HashDigestBSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_DIGEST_B to value 0"]
impl crate::Resettable for HashDigestBSpec {
    const RESET_VALUE: u32 = 0;
}
