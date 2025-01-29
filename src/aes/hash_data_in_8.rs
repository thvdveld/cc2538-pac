#[doc = "Register `HASH_DATA_IN_8` writer"]
pub type W = crate::W<HashDataIn8Spec>;
#[doc = "Field `HASH_DATA_IN` writer - HASH_DATA_IN\\[287:256\\]
These registers must be written with the 512-bit input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when the rfd_in bit of the HASH_IO_BUF_CTRL register is high. If the rfd_in bit is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than 64 bytes, multiple blocks of data are written to this input buffer using a handshake through flags of the HASH_IO_BUF_CTRL register. All blocks except the last are required to be 512 bits in size. If the last block is not 512 bits long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
pub type HashDataInW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - HASH_DATA_IN\\[287:256\\]
These registers must be written with the 512-bit input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when the rfd_in bit of the HASH_IO_BUF_CTRL register is high. If the rfd_in bit is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than 64 bytes, multiple blocks of data are written to this input buffer using a handshake through flags of the HASH_IO_BUF_CTRL register. All blocks except the last are required to be 512 bits in size. If the last block is not 512 bits long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
    #[inline(always)]
    pub fn hash_data_in(&mut self) -> HashDataInW<HashDataIn8Spec> {
        HashDataInW::new(self, 0)
    }
}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_data_in_8::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashDataIn8Spec;
impl crate::RegisterSpec for HashDataIn8Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hash_data_in_8::W`](W) writer structure"]
impl crate::Writable for HashDataIn8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_DATA_IN_8 to value 0"]
impl crate::Resettable for HashDataIn8Spec {
    const RESET_VALUE: u32 = 0;
}
