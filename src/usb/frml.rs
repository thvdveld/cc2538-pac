#[doc = "Register `FRML` reader"]
pub type R = crate::R<FrmlSpec>;
#[doc = "Field `FRAMEL` reader - Bits 7:0 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
pub type FramelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bits 7:0 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
    #[inline(always)]
    pub fn framel(&self) -> FramelR {
        FramelR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Frame number (low byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`frml::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrmlSpec;
impl crate::RegisterSpec for FrmlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frml::R`](R) reader structure"]
impl crate::Readable for FrmlSpec {}
#[doc = "`reset()` method sets FRML to value 0"]
impl crate::Resettable for FrmlSpec {
    const RESET_VALUE: u32 = 0;
}
