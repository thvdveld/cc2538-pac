#[doc = "Register `HASH_IO_BUF_CTRL` reader"]
pub struct R(crate::R<HASH_IO_BUF_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_IO_BUF_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_IO_BUF_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_IO_BUF_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_IO_BUF_CTRL` writer"]
pub struct W(crate::W<HASH_IO_BUF_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_IO_BUF_CTRL_SPEC>;
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
impl From<crate::W<HASH_IO_BUF_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_IO_BUF_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAD_DMA_MESSAGE` reader - Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
pub struct PAD_DMA_MESSAGE_R(crate::FieldReader<bool, bool>);
impl PAD_DMA_MESSAGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD_DMA_MESSAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_DMA_MESSAGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_DMA_MESSAGE` writer - Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
pub struct PAD_DMA_MESSAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_DMA_MESSAGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `GET_DIGEST` reader - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASH_DATA_IN register. When provided without data_in_av, the current internal digest buffer value is copied to the HASH_DIGEST_n registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASH_DATA_IN register. In the period between this bit is set by the host and the actual HASH_DATA_IN processing, this bit reads 1."]
pub struct GET_DIGEST_R(crate::FieldReader<bool, bool>);
impl GET_DIGEST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GET_DIGEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GET_DIGEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GET_DIGEST` writer - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASH_DATA_IN register. When provided without data_in_av, the current internal digest buffer value is copied to the HASH_DIGEST_n registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASH_DATA_IN register. In the period between this bit is set by the host and the actual HASH_DATA_IN processing, this bit reads 1."]
pub struct GET_DIGEST_W<'a> {
    w: &'a mut W,
}
impl<'a> GET_DIGEST_W<'a> {
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
#[doc = "Field `PAD_MESSAGE` reader - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASH_DATA_IN register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASH_DATA_IN register. When the last message block is smaller than 512 bits, the pad_message bit must be set to 1 together with the data_in_av bit. When the last message block is equal to 512 bits, pad_message may be set together with data_in_av. In this case the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become 1 again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
pub struct PAD_MESSAGE_R(crate::FieldReader<bool, bool>);
impl PAD_MESSAGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD_MESSAGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_MESSAGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAD_MESSAGE` writer - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASH_DATA_IN register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASH_DATA_IN register. When the last message block is smaller than 512 bits, the pad_message bit must be set to 1 together with the data_in_av bit. When the last message block is equal to 512 bits, pad_message may be set together with data_in_av. In this case the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become 1 again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
pub struct PAD_MESSAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_MESSAGE_W<'a> {
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
#[doc = "Field `RFD_IN` reader - Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASH_DATA_IN registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASH_DATA_IN; writing new data to these registers is not allowed."]
pub struct RFD_IN_R(crate::FieldReader<bool, bool>);
impl RFD_IN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFD_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFD_IN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFD_IN` writer - Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASH_DATA_IN registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASH_DATA_IN; writing new data to these registers is not allowed."]
pub struct RFD_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> RFD_IN_W<'a> {
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
#[doc = "Field `DATA_IN_AV` reader - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASH_DATA_IN; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASH_DATA_IN contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
pub struct DATA_IN_AV_R(crate::FieldReader<bool, bool>);
impl DATA_IN_AV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATA_IN_AV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_IN_AV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA_IN_AV` writer - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASH_DATA_IN; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASH_DATA_IN contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
pub struct DATA_IN_AV_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_IN_AV_W<'a> {
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
#[doc = "Field `OUTPUT_FULL` reader - Indicates that the output buffer registers (HASH_DIGEST_n) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
pub struct OUTPUT_FULL_R(crate::FieldReader<bool, bool>);
impl OUTPUT_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTPUT_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTPUT_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTPUT_FULL` writer - Indicates that the output buffer registers (HASH_DIGEST_n) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
pub struct OUTPUT_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_FULL_W<'a> {
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
    #[doc = "Bit 7 - Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
    #[inline(always)]
    pub fn pad_dma_message(&self) -> PAD_DMA_MESSAGE_R {
        PAD_DMA_MESSAGE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASH_DATA_IN register. When provided without data_in_av, the current internal digest buffer value is copied to the HASH_DIGEST_n registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASH_DATA_IN register. In the period between this bit is set by the host and the actual HASH_DATA_IN processing, this bit reads 1."]
    #[inline(always)]
    pub fn get_digest(&self) -> GET_DIGEST_R {
        GET_DIGEST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASH_DATA_IN register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASH_DATA_IN register. When the last message block is smaller than 512 bits, the pad_message bit must be set to 1 together with the data_in_av bit. When the last message block is equal to 512 bits, pad_message may be set together with data_in_av. In this case the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become 1 again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
    #[inline(always)]
    pub fn pad_message(&self) -> PAD_MESSAGE_R {
        PAD_MESSAGE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASH_DATA_IN registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASH_DATA_IN; writing new data to these registers is not allowed."]
    #[inline(always)]
    pub fn rfd_in(&self) -> RFD_IN_R {
        RFD_IN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASH_DATA_IN; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASH_DATA_IN contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
    #[inline(always)]
    pub fn data_in_av(&self) -> DATA_IN_AV_R {
        DATA_IN_AV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Indicates that the output buffer registers (HASH_DIGEST_n) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
    #[inline(always)]
    pub fn output_full(&self) -> OUTPUT_FULL_R {
        OUTPUT_FULL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
    #[inline(always)]
    pub fn pad_dma_message(&mut self) -> PAD_DMA_MESSAGE_W {
        PAD_DMA_MESSAGE_W { w: self }
    }
    #[doc = "Bit 6 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASH_DATA_IN register. When provided without data_in_av, the current internal digest buffer value is copied to the HASH_DIGEST_n registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASH_DATA_IN register. In the period between this bit is set by the host and the actual HASH_DATA_IN processing, this bit reads 1."]
    #[inline(always)]
    pub fn get_digest(&mut self) -> GET_DIGEST_W {
        GET_DIGEST_W { w: self }
    }
    #[doc = "Bit 5 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASH_DATA_IN register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASH_DATA_IN register. When the last message block is smaller than 512 bits, the pad_message bit must be set to 1 together with the data_in_av bit. When the last message block is equal to 512 bits, pad_message may be set together with data_in_av. In this case the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become 1 again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
    #[inline(always)]
    pub fn pad_message(&mut self) -> PAD_MESSAGE_W {
        PAD_MESSAGE_W { w: self }
    }
    #[doc = "Bit 2 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASH_DATA_IN registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASH_DATA_IN; writing new data to these registers is not allowed."]
    #[inline(always)]
    pub fn rfd_in(&mut self) -> RFD_IN_W {
        RFD_IN_W { w: self }
    }
    #[doc = "Bit 1 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASH_DATA_IN; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASH_DATA_IN contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
    #[inline(always)]
    pub fn data_in_av(&mut self) -> DATA_IN_AV_W {
        DATA_IN_AV_W { w: self }
    }
    #[doc = "Bit 0 - Indicates that the output buffer registers (HASH_DIGEST_n) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
    #[inline(always)]
    pub fn output_full(&mut self) -> OUTPUT_FULL_W {
        OUTPUT_FULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input/output buffer control and status register This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_io_buf_ctrl](index.html) module"]
pub struct HASH_IO_BUF_CTRL_SPEC;
impl crate::RegisterSpec for HASH_IO_BUF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_io_buf_ctrl::R](R) reader structure"]
impl crate::Readable for HASH_IO_BUF_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_io_buf_ctrl::W](W) writer structure"]
impl crate::Writable for HASH_IO_BUF_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_IO_BUF_CTRL to value 0"]
impl crate::Resettable for HASH_IO_BUF_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}