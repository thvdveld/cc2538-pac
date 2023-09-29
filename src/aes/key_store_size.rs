#[doc = "Register `KEY_STORE_SIZE` reader"]
pub type R = crate::R<KEY_STORE_SIZE_SPEC>;
#[doc = "Register `KEY_STORE_SIZE` writer"]
pub type W = crate::W<KEY_STORE_SIZE_SPEC>;
#[doc = "Field `KEY_SIZE` reader - Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
pub type KEY_SIZE_R = crate::FieldReader;
#[doc = "Field `KEY_SIZE` writer - Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
pub type KEY_SIZE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
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
    #[must_use]
    pub fn key_size(&mut self) -> KEY_SIZE_W<KEY_STORE_SIZE_SPEC, 0> {
        KEY_SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Key store size register This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_store_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_store_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_STORE_SIZE_SPEC;
impl crate::RegisterSpec for KEY_STORE_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_store_size::R`](R) reader structure"]
impl crate::Readable for KEY_STORE_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_store_size::W`](W) writer structure"]
impl crate::Writable for KEY_STORE_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY_STORE_SIZE to value 0"]
impl crate::Resettable for KEY_STORE_SIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
