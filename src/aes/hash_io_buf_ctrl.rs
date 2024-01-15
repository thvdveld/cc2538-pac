#[doc = "Register `HASH_IO_BUF_CTRL` reader"]
pub type R = crate::R<HASH_IO_BUF_CTRL_SPEC>;
#[doc = "Register `HASH_IO_BUF_CTRL` writer"]
pub type W = crate::W<HASH_IO_BUF_CTRL_SPEC>;
#[doc = "Field `OUTPUT_FULL` reader - Indicates that the output buffer registers (HASH_DIGEST_n) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
pub type OUTPUT_FULL_R = crate::BitReader;
#[doc = "Field `OUTPUT_FULL` writer - Indicates that the output buffer registers (HASH_DIGEST_n) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
pub type OUTPUT_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_IN_AV` reader - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASH_DATA_IN; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASH_DATA_IN contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
pub type DATA_IN_AV_R = crate::BitReader;
#[doc = "Field `DATA_IN_AV` writer - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASH_DATA_IN; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASH_DATA_IN contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
pub type DATA_IN_AV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFD_IN` reader - Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASH_DATA_IN registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASH_DATA_IN; writing new data to these registers is not allowed."]
pub type RFD_IN_R = crate::BitReader;
#[doc = "Field `RFD_IN` writer - Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASH_DATA_IN registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASH_DATA_IN; writing new data to these registers is not allowed."]
pub type RFD_IN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_MESSAGE` reader - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASH_DATA_IN register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASH_DATA_IN register. When the last message block is smaller than 512 bits, the pad_message bit must be set to 1 together with the data_in_av bit. When the last message block is equal to 512 bits, pad_message may be set together with data_in_av. In this case the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become 1 again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
pub type PAD_MESSAGE_R = crate::BitReader;
#[doc = "Field `PAD_MESSAGE` writer - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASH_DATA_IN register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASH_DATA_IN register. When the last message block is smaller than 512 bits, the pad_message bit must be set to 1 together with the data_in_av bit. When the last message block is equal to 512 bits, pad_message may be set together with data_in_av. In this case the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become 1 again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
pub type PAD_MESSAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GET_DIGEST` reader - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASH_DATA_IN register. When provided without data_in_av, the current internal digest buffer value is copied to the HASH_DIGEST_n registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASH_DATA_IN register. In the period between this bit is set by the host and the actual HASH_DATA_IN processing, this bit reads 1."]
pub type GET_DIGEST_R = crate::BitReader;
#[doc = "Field `GET_DIGEST` writer - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASH_DATA_IN register. When provided without data_in_av, the current internal digest buffer value is copied to the HASH_DIGEST_n registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASH_DATA_IN register. In the period between this bit is set by the host and the actual HASH_DATA_IN processing, this bit reads 1."]
pub type GET_DIGEST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAD_DMA_MESSAGE` reader - Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
pub type PAD_DMA_MESSAGE_R = crate::BitReader;
#[doc = "Field `PAD_DMA_MESSAGE` writer - Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
pub type PAD_DMA_MESSAGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates that the output buffer registers (HASH_DIGEST_n) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
    #[inline(always)]
    pub fn output_full(&self) -> OUTPUT_FULL_R {
        OUTPUT_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASH_DATA_IN; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASH_DATA_IN contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
    #[inline(always)]
    pub fn data_in_av(&self) -> DATA_IN_AV_R {
        DATA_IN_AV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASH_DATA_IN registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASH_DATA_IN; writing new data to these registers is not allowed."]
    #[inline(always)]
    pub fn rfd_in(&self) -> RFD_IN_R {
        RFD_IN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASH_DATA_IN register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASH_DATA_IN register. When the last message block is smaller than 512 bits, the pad_message bit must be set to 1 together with the data_in_av bit. When the last message block is equal to 512 bits, pad_message may be set together with data_in_av. In this case the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become 1 again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
    #[inline(always)]
    pub fn pad_message(&self) -> PAD_MESSAGE_R {
        PAD_MESSAGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASH_DATA_IN register. When provided without data_in_av, the current internal digest buffer value is copied to the HASH_DIGEST_n registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASH_DATA_IN register. In the period between this bit is set by the host and the actual HASH_DATA_IN processing, this bit reads 1."]
    #[inline(always)]
    pub fn get_digest(&self) -> GET_DIGEST_R {
        GET_DIGEST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
    #[inline(always)]
    pub fn pad_dma_message(&self) -> PAD_DMA_MESSAGE_R {
        PAD_DMA_MESSAGE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that the output buffer registers (HASH_DIGEST_n) are available for reading by the host. When this bit reads 0, the output buffer registers are released; the hash engine is allowed to write new data to it. In this case, the registers should not be read by the host. When this bit reads 1, the hash engine has stored the result of the latest hash operation in the output buffer registers. As long as this bit reads 1, the host may read output buffer registers and the hash engine is prevented from writing new data to the output buffer. After retrieving the hash result data from the output buffer, the host must write a 1 to this bit to clear it. This makes the digest output buffer available for the hash engine to store new hash results. Writing 0 to this bit has no effect. Note: If this bit is asserted (1) no new operation should be started before the digest is retrieved from the hash engine and this bit is cleared (0)."]
    #[inline(always)]
    #[must_use]
    pub fn output_full(&mut self) -> OUTPUT_FULL_W<HASH_IO_BUF_CTRL_SPEC> {
        OUTPUT_FULL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers contain new input data for processing. The host must write a 1 to this bit to start processing the data in HASH_DATA_IN; the hash engine will process the new data as soon as it is ready for it (rfd_in bit is 1). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads as 0) when the hash engine starts processing the HASH_DATA_IN contents. This bit reads 1 between the time it was set by the host and the hash engine actually starts processing the input data block."]
    #[inline(always)]
    #[must_use]
    pub fn data_in_av(&mut self) -> DATA_IN_AV_W<HASH_IO_BUF_CTRL_SPEC> {
        DATA_IN_AV_W::new(self, 1)
    }
    #[doc = "Bit 2 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit can be ignored when data is received through the DMA. Read-only status of the input buffer of the hash engine. When 1, the input buffer of the hash engine can accept new data; the HASH_DATA_IN registers can safely be populated with new data. When 0, the input buffer of the hash engine is processing the data that is currently in HASH_DATA_IN; writing new data to these registers is not allowed."]
    #[inline(always)]
    #[must_use]
    pub fn rfd_in(&mut self) -> RFD_IN_W<HASH_IO_BUF_CTRL_SPEC> {
        RFD_IN_W::new(self, 2)
    }
    #[doc = "Bit 5 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates that the HASH_DATA_IN registers hold the last data of the message and hash padding must be applied. The host must write this bit to 1 in order to indicate to the hash engine that the HASH_DATA_IN register currently holds the last data of the message. When pad_message is set to 1, the hash engine will add padding bits to the data currently in the HASH_DATA_IN register. When the last message block is smaller than 512 bits, the pad_message bit must be set to 1 together with the data_in_av bit. When the last message block is equal to 512 bits, pad_message may be set together with data_in_av. In this case the pad_message bit may also be set after the last data block has been written to the hash engine (so when the rfd_in bit has become 1 again after writing the last data block). Writing 0 to this bit has no effect. This bit is automatically cleared (i.e. reads 0) by the hash engine. This bit reads 1 between the time it was set by the host and the hash engine interpreted its value."]
    #[inline(always)]
    #[must_use]
    pub fn pad_message(&mut self) -> PAD_MESSAGE_W<HASH_IO_BUF_CTRL_SPEC> {
        PAD_MESSAGE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Note: The bit description below is only applicable when data is sent through the slave interface. This bit must be set to 0 when data is received through the DMA. This bit indicates whether the hash engine should provide the hash digest. When provided simultaneously with data_in_av, the hash digest is provided after processing the data that is currently in the HASH_DATA_IN register. When provided without data_in_av, the current internal digest buffer value is copied to the HASH_DIGEST_n registers. The host must write a 1 to this bit to make the intermediate hash digest available. Writing 0 to this bit has no effect. This bit is automatically cleared (that is, reads 0) when the hash engine has processed the contents of the HASH_DATA_IN register. In the period between this bit is set by the host and the actual HASH_DATA_IN processing, this bit reads 1."]
    #[inline(always)]
    #[must_use]
    pub fn get_digest(&mut self) -> GET_DIGEST_W<HASH_IO_BUF_CTRL_SPEC> {
        GET_DIGEST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Note: This bit must only be used when data is supplied through the DMA. It should not be used when data is supplied through the slave interface. This bit indicates whether the hash engine has to pad the message, received through the DMA and finalize the hash. When set to 1, the hash engine pads the last block using the programmed length. After padding, the final hash result is calculated. When set to 0, the hash engine treats the last written block as block-size aligned and calculates the intermediate digest. This bit is automatically cleared when the last DMA data block is arrived in the hash engine."]
    #[inline(always)]
    #[must_use]
    pub fn pad_dma_message(&mut self) -> PAD_DMA_MESSAGE_W<HASH_IO_BUF_CTRL_SPEC> {
        PAD_DMA_MESSAGE_W::new(self, 7)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Input/output buffer control and status register This register pair shares a single address location and contains bits that control and monitor the data flow between the host and the hash engine.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_io_buf_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_io_buf_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_IO_BUF_CTRL_SPEC;
impl crate::RegisterSpec for HASH_IO_BUF_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_io_buf_ctrl::R`](R) reader structure"]
impl crate::Readable for HASH_IO_BUF_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hash_io_buf_ctrl::W`](W) writer structure"]
impl crate::Writable for HASH_IO_BUF_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_IO_BUF_CTRL to value 0"]
impl crate::Resettable for HASH_IO_BUF_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
