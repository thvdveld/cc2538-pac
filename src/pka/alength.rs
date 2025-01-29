#[doc = "Register `ALENGTH` reader"]
pub type R = crate::R<AlengthSpec>;
#[doc = "Register `ALENGTH` writer"]
pub type W = crate::W<AlengthSpec>;
#[doc = "Field `ALENGTH` reader - This register specifies the length (in 32-bit words) of Vector A."]
pub type AlengthR = crate::FieldReader<u16>;
#[doc = "Field `ALENGTH` writer - This register specifies the length (in 32-bit words) of Vector A."]
pub type AlengthW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - This register specifies the length (in 32-bit words) of Vector A."]
    #[inline(always)]
    pub fn alength(&self) -> AlengthR {
        AlengthR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - This register specifies the length (in 32-bit words) of Vector A."]
    #[inline(always)]
    pub fn alength(&mut self) -> AlengthW<AlengthSpec> {
        AlengthW::new(self, 0)
    }
}
#[doc = "PKA vector A length During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::Reg::read) this register and get [`alength::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alength::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AlengthSpec;
impl crate::RegisterSpec for AlengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alength::R`](R) reader structure"]
impl crate::Readable for AlengthSpec {}
#[doc = "`write(|w| ..)` method takes [`alength::W`](W) writer structure"]
impl crate::Writable for AlengthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALENGTH to value 0"]
impl crate::Resettable for AlengthSpec {
    const RESET_VALUE: u32 = 0;
}
