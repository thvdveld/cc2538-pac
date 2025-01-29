#[doc = "Register `AES_CTRL` reader"]
pub type R = crate::R<AesCtrlSpec>;
#[doc = "Register `AES_CTRL` writer"]
pub type W = crate::W<AesCtrlSpec>;
#[doc = "Field `output_ready` reader - If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
pub type OutputReadyR = crate::BitReader;
#[doc = "Field `output_ready` writer - If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
pub type OutputReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `input_ready` reader - If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
pub type InputReadyR = crate::BitReader;
#[doc = "Field `input_ready` writer - If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
pub type InputReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `direction` reader - If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub type DirectionR = crate::BitReader;
#[doc = "Field `direction` writer - If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub type DirectionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `key_size` reader - This read-only field specifies the key size. The key size is automatically configured when a new key is loaded through the key store module. 00 = N/A - Reserved 01 = 128-bit 10 = 192-bit 11 = 256-bit"]
pub type KeySizeR = crate::FieldReader;
#[doc = "Field `CBC` reader - If set to 1, cipher-block-chaining (CBC) mode is selected."]
pub type CbcR = crate::BitReader;
#[doc = "Field `CBC` writer - If set to 1, cipher-block-chaining (CBC) mode is selected."]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTR` reader - If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
pub type CtrR = crate::BitReader;
#[doc = "Field `CTR` writer - If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
pub type CtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ctr_width` reader - Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
pub type CtrWidthR = crate::FieldReader;
#[doc = "Field `ctr_width` writer - Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
pub type CtrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CBC_MAC` reader - Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
pub type CbcMacR = crate::BitReader;
#[doc = "Field `CBC_MAC` writer - Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
pub type CbcMacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCM` reader - Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
pub type GcmR = crate::FieldReader;
#[doc = "Field `GCM` writer - Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
pub type GcmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CCM` reader - If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub type CcmR = crate::BitReader;
#[doc = "Field `CCM` writer - If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub type CcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCM_L` reader - Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
pub type CcmLR = crate::FieldReader;
#[doc = "Field `CCM_L` writer - Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
pub type CcmLW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CCM_M` reader - Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub type CcmMR = crate::FieldReader;
#[doc = "Field `CCM_M` writer - Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub type CcmMW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `save_context` reader - This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
pub type SaveContextR = crate::BitReader;
#[doc = "Field `save_context` writer - This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
pub type SaveContextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `saved_context_ready` reader - If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
pub type SavedContextReadyR = crate::BitReader;
#[doc = "Field `saved_context_ready` writer - If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
pub type SavedContextReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `context_ready` reader - If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context."]
pub type ContextReadyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
    #[inline(always)]
    pub fn output_ready(&self) -> OutputReadyR {
        OutputReadyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
    #[inline(always)]
    pub fn input_ready(&self) -> InputReadyR {
        InputReadyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    pub fn direction(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - This read-only field specifies the key size. The key size is automatically configured when a new key is loaded through the key store module. 00 = N/A - Reserved 01 = 128-bit 10 = 192-bit 11 = 256-bit"]
    #[inline(always)]
    pub fn key_size(&self) -> KeySizeR {
        KeySizeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - If set to 1, cipher-block-chaining (CBC) mode is selected."]
    #[inline(always)]
    pub fn cbc(&self) -> CbcR {
        CbcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub fn ctr(&self) -> CtrR {
        CtrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
    #[inline(always)]
    pub fn ctr_width(&self) -> CtrWidthR {
        CtrWidthR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 15 - Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
    #[inline(always)]
    pub fn cbc_mac(&self) -> CbcMacR {
        CbcMacR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
    #[inline(always)]
    pub fn gcm(&self) -> GcmR {
        GcmR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    pub fn ccm(&self) -> CcmR {
        CcmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:21 - Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
    #[inline(always)]
    pub fn ccm_l(&self) -> CcmLR {
        CcmLR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccm_m(&self) -> CcmMR {
        CcmMR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 29 - This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    pub fn save_context(&self) -> SaveContextR {
        SaveContextR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
    #[inline(always)]
    pub fn saved_context_ready(&self) -> SavedContextReadyR {
        SavedContextReadyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context."]
    #[inline(always)]
    pub fn context_ready(&self) -> ContextReadyR {
        ContextReadyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
    #[inline(always)]
    pub fn output_ready(&mut self) -> OutputReadyW<AesCtrlSpec> {
        OutputReadyW::new(self, 0)
    }
    #[doc = "Bit 1 - If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
    #[inline(always)]
    pub fn input_ready(&mut self) -> InputReadyW<AesCtrlSpec> {
        InputReadyW::new(self, 1)
    }
    #[doc = "Bit 2 - If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    pub fn direction(&mut self) -> DirectionW<AesCtrlSpec> {
        DirectionW::new(self, 2)
    }
    #[doc = "Bit 5 - If set to 1, cipher-block-chaining (CBC) mode is selected."]
    #[inline(always)]
    pub fn cbc(&mut self) -> CbcW<AesCtrlSpec> {
        CbcW::new(self, 5)
    }
    #[doc = "Bit 6 - If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub fn ctr(&mut self) -> CtrW<AesCtrlSpec> {
        CtrW::new(self, 6)
    }
    #[doc = "Bits 7:8 - Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
    #[inline(always)]
    pub fn ctr_width(&mut self) -> CtrWidthW<AesCtrlSpec> {
        CtrWidthW::new(self, 7)
    }
    #[doc = "Bit 15 - Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
    #[inline(always)]
    pub fn cbc_mac(&mut self) -> CbcMacW<AesCtrlSpec> {
        CbcMacW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
    #[inline(always)]
    pub fn gcm(&mut self) -> GcmW<AesCtrlSpec> {
        GcmW::new(self, 16)
    }
    #[doc = "Bit 18 - If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    pub fn ccm(&mut self) -> CcmW<AesCtrlSpec> {
        CcmW::new(self, 18)
    }
    #[doc = "Bits 19:21 - Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
    #[inline(always)]
    pub fn ccm_l(&mut self) -> CcmLW<AesCtrlSpec> {
        CcmLW::new(self, 19)
    }
    #[doc = "Bits 22:24 - Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccm_m(&mut self) -> CcmMW<AesCtrlSpec> {
        CcmMW::new(self, 22)
    }
    #[doc = "Bit 29 - This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    pub fn save_context(&mut self) -> SaveContextW<AesCtrlSpec> {
        SaveContextW::new(self, 29)
    }
    #[doc = "Bit 30 - If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
    #[inline(always)]
    pub fn saved_context_ready(&mut self) -> SavedContextReadyW<AesCtrlSpec> {
        SavedContextReadyW::new(self, 30)
    }
}
#[doc = "AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aes_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesCtrlSpec;
impl crate::RegisterSpec for AesCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_ctrl::R`](R) reader structure"]
impl crate::Readable for AesCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`aes_ctrl::W`](W) writer structure"]
impl crate::Writable for AesCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AES_CTRL to value 0"]
impl crate::Resettable for AesCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
