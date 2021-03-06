#[doc = "Register `HASH_DATA_IN_2` writer"]
pub struct W(crate::W<HASH_DATA_IN_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_DATA_IN_2_SPEC>;
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
impl From<crate::W<HASH_DATA_IN_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_DATA_IN_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HASH_DATA_IN` writer - HASH_DATA_IN\\[95:64\\]
These registers must be written with the 512-bit input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when the rfd_in bit of the HASH_IO_BUF_CTRL register is high. If the rfd_in bit is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than 64 bytes, multiple blocks of data are written to this input buffer using a handshake through flags of the HASH_IO_BUF_CTRL register. All blocks except the last are required to be 512 bits in size. If the last block is not 512 bits long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
pub type HASH_DATA_IN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HASH_DATA_IN_2_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - HASH_DATA_IN\\[95:64\\]
These registers must be written with the 512-bit input data. The data lines are connected directly to the data input of the hash module and hence into the engine's internal data buffer. Writing to each of the registers triggers a corresponding 32-bit write enable to the internal buffer. Note: The host may only write the input data buffer when the rfd_in bit of the HASH_IO_BUF_CTRL register is high. If the rfd_in bit is 0, the engine is busy with processing. During processing, it is not allowed to write new input data. For message lengths larger than 64 bytes, multiple blocks of data are written to this input buffer using a handshake through flags of the HASH_IO_BUF_CTRL register. All blocks except the last are required to be 512 bits in size. If the last block is not 512 bits long, only the least significant bits of data must be written, but they must be padded with 0s to the next 32-bit boundary. Host read operations from these register addresses return 0s."]
    #[inline(always)]
    pub fn hash_data_in(&mut self) -> HASH_DATA_IN_W<0> {
        HASH_DATA_IN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH data input registers The data input registers should be used to provide input data to the hash module through the slave interface.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_data_in_2](index.html) module"]
pub struct HASH_DATA_IN_2_SPEC;
impl crate::RegisterSpec for HASH_DATA_IN_2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hash_data_in_2::W](W) writer structure"]
impl crate::Writable for HASH_DATA_IN_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_DATA_IN_2 to value 0"]
impl crate::Resettable for HASH_DATA_IN_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
