#[doc = "Register `TBPS` reader"]
pub type R = crate::R<TbpsSpec>;
#[doc = "Field `PSS` reader - GPTM Timer B prescaler"]
pub type PssR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B prescaler"]
    #[inline(always)]
    pub fn pss(&self) -> PssR {
        PssR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "GPTM Timer B prescale snapshot For the 32-bit wide GPTM, this register shows the current value of the Timer B prescaler in the 32-bit modes. This register is ununsed in 16-bit GPTM mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbps::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbpsSpec;
impl crate::RegisterSpec for TbpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbps::R`](R) reader structure"]
impl crate::Readable for TbpsSpec {}
#[doc = "`reset()` method sets TBPS to value 0"]
impl crate::Resettable for TbpsSpec {
    const RESET_VALUE: u32 = 0;
}
