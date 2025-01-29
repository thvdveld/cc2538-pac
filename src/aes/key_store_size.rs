#[doc = "Register `KEY_STORE_SIZE` reader"]
pub type R = crate::R<KeyStoreSizeSpec>;
#[doc = "Register `KEY_STORE_SIZE` writer"]
pub type W = crate::W<KeyStoreSizeSpec>;
#[doc = "Field `KEY_SIZE` reader - Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
pub type KeySizeR = crate::FieldReader;
#[doc = "Field `KEY_SIZE` writer - Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
pub type KeySizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
    #[inline(always)]
    pub fn key_size(&self) -> KeySizeR {
        KeySizeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Key size: 00: Reserved 01: 128 bits 10: 192 bits 11: 256 bits When writing this to this register, the KEY_STORE_WRITTEN_AREA register is reset."]
    #[inline(always)]
    pub fn key_size(&mut self) -> KeySizeW<KeyStoreSizeSpec> {
        KeySizeW::new(self, 0)
    }
}
#[doc = "Key store size register This register defines the size of the keys that are written with DMA. This register should be configured before writing to the KEY_STORE_WRITE_AREA register.\n\nYou can [`read`](crate::Reg::read) this register and get [`key_store_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key_store_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyStoreSizeSpec;
impl crate::RegisterSpec for KeyStoreSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_store_size::R`](R) reader structure"]
impl crate::Readable for KeyStoreSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`key_store_size::W`](W) writer structure"]
impl crate::Writable for KeyStoreSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_STORE_SIZE to value 0"]
impl crate::Resettable for KeyStoreSizeSpec {
    const RESET_VALUE: u32 = 0;
}
