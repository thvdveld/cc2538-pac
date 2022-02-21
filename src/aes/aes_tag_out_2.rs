#[doc = "Register `AES_TAG_OUT_2` reader"]
pub struct R(crate::R<AES_TAG_OUT_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_TAG_OUT_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_TAG_OUT_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_TAG_OUT_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AES_TAG` reader - AES_TAG\\[95:64\\]
For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register contains valid data only if the TAG is available and when the store_ready bit from AES_CTRL register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
pub struct AES_TAG_R(crate::FieldReader<u32, u32>);
impl AES_TAG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        AES_TAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AES_TAG_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - AES_TAG\\[95:64\\]
For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register contains valid data only if the TAG is available and when the store_ready bit from AES_CTRL register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
    #[inline(always)]
    pub fn aes_tag(&self) -> AES_TAG_R {
        AES_TAG_R::new(self.bits)
    }
}
#[doc = "TAG registers The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order returns the IV twice.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_tag_out_2](index.html) module"]
pub struct AES_TAG_OUT_2_SPEC;
impl crate::RegisterSpec for AES_TAG_OUT_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_tag_out_2::R](R) reader structure"]
impl crate::Readable for AES_TAG_OUT_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AES_TAG_OUT_2 to value 0"]
impl crate::Resettable for AES_TAG_OUT_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
