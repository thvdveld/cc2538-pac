#[doc = "Register `AES_CTRL` reader"]
pub struct R(crate::R<AES_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_CTRL` writer"]
pub struct W(crate::W<AES_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_CTRL_SPEC>;
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
impl From<crate::W<AES_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `context_ready` reader - If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context."]
pub struct CONTEXT_READY_R(crate::FieldReader<bool, bool>);
impl CONTEXT_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONTEXT_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONTEXT_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `saved_context_ready` reader - If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
pub struct SAVED_CONTEXT_READY_R(crate::FieldReader<bool, bool>);
impl SAVED_CONTEXT_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAVED_CONTEXT_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAVED_CONTEXT_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `saved_context_ready` writer - If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
pub struct SAVED_CONTEXT_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> SAVED_CONTEXT_READY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `save_context` reader - This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
pub struct SAVE_CONTEXT_R(crate::FieldReader<bool, bool>);
impl SAVE_CONTEXT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SAVE_CONTEXT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAVE_CONTEXT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `save_context` writer - This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
pub struct SAVE_CONTEXT_W<'a> {
    w: &'a mut W,
}
impl<'a> SAVE_CONTEXT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `CCM_M` reader - Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub struct CCM_M_R(crate::FieldReader<u8, u8>);
impl CCM_M_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCM_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCM_M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCM_M` writer - Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
pub struct CCM_M_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Field `CCM_L` reader - Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
pub struct CCM_L_R(crate::FieldReader<u8, u8>);
impl CCM_L_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CCM_L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCM_L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCM_L` writer - Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
pub struct CCM_L_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | ((value as u32 & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `CCM` reader - If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub struct CCM_R(crate::FieldReader<bool, bool>);
impl CCM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCM` writer - If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
pub struct CCM_W<'a> {
    w: &'a mut W,
}
impl<'a> CCM_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `GCM` reader - Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
pub struct GCM_R(crate::FieldReader<u8, u8>);
impl GCM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCM` writer - Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
pub struct GCM_W<'a> {
    w: &'a mut W,
}
impl<'a> GCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `CBC_MAC` reader - Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
pub struct CBC_MAC_R(crate::FieldReader<bool, bool>);
impl CBC_MAC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBC_MAC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBC_MAC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBC_MAC` writer - Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
pub struct CBC_MAC_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC_MAC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `ctr_width` reader - Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
pub struct CTR_WIDTH_R(crate::FieldReader<u8, u8>);
impl CTR_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CTR_WIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR_WIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ctr_width` writer - Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
pub struct CTR_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `CTR` reader - If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
pub struct CTR_R(crate::FieldReader<bool, bool>);
impl CTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTR` writer - If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
pub struct CTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CBC` reader - If set to 1, cipher-block-chaining (CBC) mode is selected."]
pub struct CBC_R(crate::FieldReader<bool, bool>);
impl CBC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CBC` writer - If set to 1, cipher-block-chaining (CBC) mode is selected."]
pub struct CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> CBC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `key_size` reader - This read-only field specifies the key size. The key size is automatically configured when a new key is loaded through the key store module. 00 = N/A - Reserved 01 = 128-bit 10 = 192-bit 11 = 256-bit"]
pub struct KEY_SIZE_R(crate::FieldReader<u8, u8>);
impl KEY_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `direction` reader - If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub struct DIRECTION_R(crate::FieldReader<bool, bool>);
impl DIRECTION_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIRECTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRECTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `direction` writer - If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
pub struct DIRECTION_W<'a> {
    w: &'a mut W,
}
impl<'a> DIRECTION_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `input_ready` reader - If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
pub struct INPUT_READY_R(crate::FieldReader<bool, bool>);
impl INPUT_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INPUT_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INPUT_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `input_ready` writer - If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
pub struct INPUT_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_READY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `output_ready` reader - If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
pub struct OUTPUT_READY_R(crate::FieldReader<bool, bool>);
impl OUTPUT_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OUTPUT_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPUT_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `output_ready` writer - If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
pub struct OUTPUT_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_READY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - If 1, this read-only status bit indicates that the context data registers can be overwritten and the host is permitted to write the next context."]
    #[inline(always)]
    pub fn context_ready(&self) -> CONTEXT_READY_R {
        CONTEXT_READY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
    #[inline(always)]
    pub fn saved_context_ready(&self) -> SAVED_CONTEXT_READY_R {
        SAVED_CONTEXT_READY_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    pub fn save_context(&self) -> SAVE_CONTEXT_R {
        SAVE_CONTEXT_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccm_m(&self) -> CCM_M_R {
        CCM_M_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21 - Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
    #[inline(always)]
    pub fn ccm_l(&self) -> CCM_L_R {
        CCM_L_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18 - If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    pub fn ccm(&self) -> CCM_R {
        CCM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
    #[inline(always)]
    pub fn gcm(&self) -> GCM_R {
        GCM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
    #[inline(always)]
    pub fn cbc_mac(&self) -> CBC_MAC_R {
        CBC_MAC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
    #[inline(always)]
    pub fn ctr_width(&self) -> CTR_WIDTH_R {
        CTR_WIDTH_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 6 - If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If set to 1, cipher-block-chaining (CBC) mode is selected."]
    #[inline(always)]
    pub fn cbc(&self) -> CBC_R {
        CBC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - This read-only field specifies the key size. The key size is automatically configured when a new key is loaded through the key store module. 00 = N/A - Reserved 01 = 128-bit 10 = 192-bit 11 = 256-bit"]
    #[inline(always)]
    pub fn key_size(&self) -> KEY_SIZE_R {
        KEY_SIZE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 2 - If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
    #[inline(always)]
    pub fn input_ready(&self) -> INPUT_READY_R {
        INPUT_READY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
    #[inline(always)]
    pub fn output_ready(&self) -> OUTPUT_READY_R {
        OUTPUT_READY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - If 1, this status bit indicates that an AES authentication TAG and/or IV block(s) is/are available for the host to retrieve. This bit is only asserted if the save_context bit is set to 1. The bit is mutual exclusive with the context_ready bit. Writing one clears the bit to 0, indicating the AES core can start its next operation. This bit is also cleared when the 4th word of the output TAG and/or IV is read. Note: All other mode bit writes are ignored when this mode bit is written with 1. Note: This bit is controlled automatically by the EIP-120t for TAG read DMA operations."]
    #[inline(always)]
    pub fn saved_context_ready(&mut self) -> SAVED_CONTEXT_READY_W {
        SAVED_CONTEXT_READY_W { w: self }
    }
    #[doc = "Bit 29 - This bit indicates that an authentication TAG or result IV needs to be stored as a result context. Typically this bit must be set for authentication modes returning a TAG (CBC-MAC, GCM and CCM), or for basic encryption modes that require future continuation with the current result IV. If this bit is set, the engine retains its full context until the TAG and/or IV registers are read. The TAG or IV must be read before the AES engine can start a new operation."]
    #[inline(always)]
    pub fn save_context(&mut self) -> SAVE_CONTEXT_W {
        SAVE_CONTEXT_W { w: self }
    }
    #[doc = "Bits 22:24 - Defines M, which indicates the length of the authentication field for CCM operations; the authentication field length equals two times (the value of CCM-M plus one). Note: The EIP-120t always returns a 128-bit authentication field, of which the M least significant bytes are valid. All values are supported."]
    #[inline(always)]
    pub fn ccm_m(&mut self) -> CCM_M_W {
        CCM_M_W { w: self }
    }
    #[doc = "Bits 19:21 - Defines L, which indicates the width of the length field for CCM operations; the length field in bytes equals the value of CMM-L plus one. All values are supported."]
    #[inline(always)]
    pub fn ccm_l(&mut self) -> CCM_L_W {
        CCM_L_W { w: self }
    }
    #[doc = "Bit 18 - If set to 1, AES-CCM is selected AES-CCM is a combined mode, using AES for authentication and encryption. Note: Selecting AES-CCM mode requires writing of the AAD length register after all other registers. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR; selecting other AES modes than CTR mode is invalid."]
    #[inline(always)]
    pub fn ccm(&mut self) -> CCM_W {
        CCM_W { w: self }
    }
    #[doc = "Bits 16:17 - Set these bits to 11 to select AES-GCM mode. AES-GCM is a combined mode, using the Galois field multiplier GF(2 to the power of 128) for authentication and AES-CTR mode for encryption. Note: The CTR mode bit in this register must also be set to 1 to enable AES-CTR Bit combination description: 00 = No GCM mode 01 = Reserved, do not select 10 = Reserved, do not select 11 = Autonomous GHASH (both H- and Y0-encrypted calculated internally) Note: The EIP-120t-1 configuration only supports mode 11 (autonomous GHASH), other GCM modes are not allowed."]
    #[inline(always)]
    pub fn gcm(&mut self) -> GCM_W {
        GCM_W { w: self }
    }
    #[doc = "Bit 15 - Set to 1 to select AES-CBC MAC mode. The direction bit must be set to 1 for this mode. Selecting this mode requires writing the length register after all other registers."]
    #[inline(always)]
    pub fn cbc_mac(&mut self) -> CBC_MAC_W {
        CBC_MAC_W { w: self }
    }
    #[doc = "Bits 7:8 - Specifies the counter width for AES-CTR mode 00 = 32-bit counter 01 = 64-bit counter 10 = 96-bit counter 11 = 128-bit counter"]
    #[inline(always)]
    pub fn ctr_width(&mut self) -> CTR_WIDTH_W {
        CTR_WIDTH_W { w: self }
    }
    #[doc = "Bit 6 - If set to 1, AES counter mode (CTR) is selected. Note: This bit must also be set for GCM and CCM, when encryption/decryption is required."]
    #[inline(always)]
    pub fn ctr(&mut self) -> CTR_W {
        CTR_W { w: self }
    }
    #[doc = "Bit 5 - If set to 1, cipher-block-chaining (CBC) mode is selected."]
    #[inline(always)]
    pub fn cbc(&mut self) -> CBC_W {
        CBC_W { w: self }
    }
    #[doc = "Bit 2 - If set to 1 an encrypt operation is performed. If set to 0 a decrypt operation is performed. This bit must be written with a 1 when CBC-MAC is selected."]
    #[inline(always)]
    pub fn direction(&mut self) -> DIRECTION_W {
        DIRECTION_W { w: self }
    }
    #[doc = "Bit 1 - If 1, this status bit indicates that the 16-byte AES input buffer is empty. The host is permitted to write the next block of data. Writing 0 clears the bit to 0 and indicates that the AES core can use the provided input data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t. After reset, this bit is 0. After writing a context, this bit becomes 1."]
    #[inline(always)]
    pub fn input_ready(&mut self) -> INPUT_READY_W {
        INPUT_READY_W { w: self }
    }
    #[doc = "Bit 0 - If 1, this status bit indicates that an AES output block is available to be retrieved by the host. Writing 0 clears the bit to 0 and indicates that output data is read by the host. The AES core can provide a next output data block. Writing 1 to this bit is ignored. Note: For DMA operations, this bit is automatically controlled by the EIP-120t."]
    #[inline(always)]
    pub fn output_ready(&mut self) -> OUTPUT_READY_W {
        OUTPUT_READY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES input/output buffer control and mode register This register specifies the AES mode of operation for the EIP-120t. Electronic codebook (ECB) mode is automatically selected if bits \\[28:5\\]
of this register are all 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_ctrl](index.html) module"]
pub struct AES_CTRL_SPEC;
impl crate::RegisterSpec for AES_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_ctrl::R](R) reader structure"]
impl crate::Readable for AES_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_ctrl::W](W) writer structure"]
impl crate::Writable for AES_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AES_CTRL to value 0"]
impl crate::Resettable for AES_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
