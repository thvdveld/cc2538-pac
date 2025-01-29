#[doc = "Register `FRMH` reader"]
pub type R = crate::R<FrmhSpec>;
#[doc = "Field `FRAMEH` reader - Bits 10:8 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
pub type FramehR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Bits 10:8 of the 11-bit frame number The frame number is only updated upon successful reception of SOF tokens"]
    #[inline(always)]
    pub fn frameh(&self) -> FramehR {
        FramehR::new((self.bits & 7) as u8)
    }
}
#[doc = "Frame number (high byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`frmh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrmhSpec;
impl crate::RegisterSpec for FrmhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frmh::R`](R) reader structure"]
impl crate::Readable for FrmhSpec {}
#[doc = "`reset()` method sets FRMH to value 0"]
impl crate::Resettable for FrmhSpec {
    const RESET_VALUE: u32 = 0;
}
