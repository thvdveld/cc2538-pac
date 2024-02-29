#[doc = "Register `CSPT` reader"]
pub type R = crate::R<CsptSpec>;
#[doc = "Field `CSPT` reader - Content is decremented each time the MAC Timer overflows while the CSP program is running. The SCP program stops when decremented to 0. Setting CSPT = 0xFF prevents the register from being decremented."]
pub type CsptR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Content is decremented each time the MAC Timer overflows while the CSP program is running. The SCP program stops when decremented to 0. Setting CSPT = 0xFF prevents the register from being decremented."]
    #[inline(always)]
    pub fn cspt(&self) -> CsptR {
        CsptR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CSP T data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsptSpec;
impl crate::RegisterSpec for CsptSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cspt::R`](R) reader structure"]
impl crate::Readable for CsptSpec {}
#[doc = "`reset()` method sets CSPT to value 0"]
impl crate::Resettable for CsptSpec {
    const RESET_VALUE: u32 = 0;
}
