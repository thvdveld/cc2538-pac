#[doc = "Register `AES_TAG_OUT_1` reader"]
pub type R = crate::R<AesTagOut1Spec>;
#[doc = "Field `AES_TAG` reader - AES_TAG\\[63:32\\]
For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register contains valid data only if the TAG is available and when the store_ready bit from AES_CTRL register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
pub type AesTagR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - AES_TAG\\[63:32\\]
For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register contains valid data only if the TAG is available and when the store_ready bit from AES_CTRL register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
    #[inline(always)]
    pub fn aes_tag(&self) -> AesTagR {
        AesTagR::new(self.bits)
    }
}
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice.\n\nYou can [`read`](crate::Reg::read) this register and get [`aes_tag_out_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AesTagOut1Spec;
impl crate::RegisterSpec for AesTagOut1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aes_tag_out_1::R`](R) reader structure"]
impl crate::Readable for AesTagOut1Spec {}
#[doc = "`reset()` method sets AES_TAG_OUT_1 to value 0"]
impl crate::Resettable for AesTagOut1Spec {
    const RESET_VALUE: u32 = 0;
}
