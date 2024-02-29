#[doc = "Register `ALTBASE` reader"]
pub type R = crate::R<AltbaseSpec>;
#[doc = "Field `ADDR` reader - Alternate channel address pointer This field provides the base address of the alternate channel control structures."]
pub type AddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Alternate channel address pointer This field provides the base address of the alternate channel control structures."]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(self.bits)
    }
}
#[doc = "DMA alternate channel control base pointer The ALTBASE register returns the base address of the alternate channel control data. This register removes the necessity for application software to calculate the base address of the alternate channel control structures. This register cannot be read when the uDMA controller is in the reset state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altbase::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltbaseSpec;
impl crate::RegisterSpec for AltbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`altbase::R`](R) reader structure"]
impl crate::Readable for AltbaseSpec {}
#[doc = "`reset()` method sets ALTBASE to value 0"]
impl crate::Resettable for AltbaseSpec {
    const RESET_VALUE: u32 = 0;
}
