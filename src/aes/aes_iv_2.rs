#[doc = "Register `AES_IV_2` reader"]
pub type R = crate::R<AesIv2Spec>;
#[doc = "Register `AES_IV_2` writer"]
pub type W = crate::W<AesIv2Spec>;
#[doc = "Field `AES_IV` reader - AES_IV\\[95:64\\]
Initialization vector Used for regular non-ECB modes (CBC/CTR): -\\[127:0\\]
- AES_IV - For regular AES operations (CBC and CTR) these registers must be written with a new 128-bit IV. After an operation, these registers contain the latest 128-bit result IV, generated by the EIP-120t. If CTR mode is selected, this value is incremented with 0x1: After first use - When a new data block is submitted to the engine For GCM: -\\[127:0\\]
- AES_IV - For GCM operations, these registers must be written with a new 128-bit IV. After an operation, these registers contain the updated 128-bit result IV, generated by the EIP-120t. Note that bits \\[127:96\\]
of the IV represent the initial counter value (which is 1 for GCM) and must therefore be initialized to 0x01000000. This value is incremented with 0x1: After first use - When a new data block is submitted to the engine. For CCM: -\\[127:0\\]
- A0: For CCM this field must be written with value A0, this value is the concatenation of: A0-flags (5-bits of 0 and 3-bits 'L'), Nonce and counter value. 'L' must be a copy from the 'L' value of the AES_CTRL register. This 'L' indicates the width of the Nonce and counter. The loaded counter must be initialized to 0. The total width of A0 is 128-bit. For CBC-MAC: -\\[127:0\\]
- Zeroes - For CBC-MAC this register must be written with 0s at the start of each operation. After an operation, these registers contain the 128-bit TAG output, generated by the EIP-120t."]
pub type AesIvR = crate::FieldReader<u32>;
#[doc = "Field `AES_IV` writer - AES_IV\\[95:64\\]
Initialization vector Used for regular non-ECB modes (CBC/CTR): -\\[127:0\\]
- AES_IV - For regular AES operations (CBC and CTR) these registers must be written with a new 128-bit IV. After an operation, these registers contain the latest 128-bit result IV, generated by the EIP-120t. If CTR mode is selected, this value is incremented with 0x1: After first use - When a new data block is submitted to the engine For GCM: -\\[127:0\\]
- AES_IV - For GCM operations, these registers must be written with a new 128-bit IV. After an operation, these registers contain the updated 128-bit result IV, generated by the EIP-120t. Note that bits \\[127:96\\]
of the IV represent the initial counter value (which is 1 for GCM) and must therefore be initialized to 0x01000000. This value is incremented with 0x1: After first use - When a new data block is submitted to the engine. For CCM: -\\[127:0\\]
- A0: For CCM this field must be written with value A0, this value is the concatenation of: A0-flags (5-bits of 0 and 3-bits 'L'), Nonce and counter value. 'L' must be a copy from the 'L' value of the AES_CTRL register. This 'L' indicates the width of the Nonce and counter. The loaded counter must be initialized to 0. The total width of A0 is 128-bit. For CBC-MAC: -\\[127:0\\]
- Zeroes - For CBC-MAC this register must be written with 0s at the start of each operation. After an operation, these registers contain the 128-bit TAG output, generated by the EIP-120t."]
pub type AesIvW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - AES_IV\\[95:64\\]
Initialization vector Used for regular non-ECB modes (CBC/CTR): -\\[127:0\\]
- AES_IV - For regular AES operations (CBC and CTR) these registers must be written with a new 128-bit IV. After an operation, these registers contain the latest 128-bit result IV, generated by the EIP-120t. If CTR mode is selected, this value is incremented with 0x1: After first use - When a new data block is submitted to the engine For GCM: -\\[127:0\\]
- AES_IV - For GCM operations, these registers must be written with a new 128-bit IV. After an operation, these registers contain the updated 128-bit result IV, generated by the EIP-120t. Note that bits \\[127:96\\]
of the IV represent the initial counter value (which is 1 for GCM) and must therefore be initialized to 0x01000000. This value is incremented with 0x1: After first use - When a new data block is submitted to the engine. For CCM: -\\[127:0\\]
- A0: For CCM this field must be written with value A0, this value is the concatenation of: A0-flags (5-bits of 0 and 3-bits 'L'), Nonce and counter value. 'L' must be a copy from the 'L' value of the AES_CTRL register. This 'L' indicates the width of the Nonce and counter. The loaded counter must be initialized to 0. The total width of A0 is 128-bit. For CBC-MAC: -\\[127:0\\]
- Zeroes - For CBC-MAC this register must be written with 0s at the start of each operation. After an operation, these registers contain the 128-bit TAG output, generated by the EIP-120t."]
    #[inline(always)]
    pub fn aes_iv(&self) -> AesIvR {
        AesIvR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - AES_IV\\[95:64\\]
Initialization vector Used for regular non-ECB modes (CBC/CTR): -\\[127:0\\]
- AES_IV - For regular AES operations (CBC and CTR) these registers must be written with a new 128-bit IV. After an operation, these registers contain the latest 128-bit result IV, generated by the EIP-120t. If CTR mode is selected, this value is incremented with 0x1: After first use - When a new data block is submitted to the engine For GCM: -\\[127:0\\]
- AES_IV - For GCM operations, these registers must be written with a new 128-bit IV. After an operation, these registers contain the updated 128-bit result IV, generated by the EIP-120t. Note that bits \\[127:96\\]
of the IV represent the initial counter value (which is 1 for GCM) and must therefore be initialized to 0x01000000. This value is incremented with 0x1: After first use - When a new data block is submitted to the engine. For CCM: -\\[127:0\\]
- A0: For CCM this field must be written with value A0, this value is the concatenation of: A0-flags (5-bits of 0 and 3-bits 'L'), Nonce and counter value. 'L' must be a copy from the 'L' value of the AES_CTRL register. This 'L' indicates the width of the Nonce and counter. The loaded counter must be initialized to 0. The total width of A0 is 128-bit. For CBC-MAC: -\\[127:0\\]
- Zeroes - For CBC-MAC this register must be written with 0s at the start of each operation. After an operation, these registers contain the 128-bit TAG output, generated by the EIP-120t."]
    #[inline(always)]
    pub fn aes_iv(&mut self) -> AesIvW<AesIv2Spec> {
        AesIvW::new(self, 0)
    }
}
#[doc = "AES initialization vector registers These registers are used to provide and read the IV from the AES engine.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_iv_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_iv_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesIv2Spec;
impl crate::RegisterSpec for AesIv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_iv_2::R`](R) reader structure"]
impl crate::Readable for AesIv2Spec {}
#[doc = "`write(|w| ..)` method takes [`aes_iv_2::W`](W) writer structure"]
impl crate::Writable for AesIv2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_IV_2 to value 0"]
impl crate::Resettable for AesIv2Spec {
    const RESET_VALUE: u32 = 0;
}
