#[doc = "Register `CPTR` reader"]
pub type R = crate::R<CptrSpec>;
#[doc = "Register `CPTR` writer"]
pub type W = crate::W<CptrSpec>;
#[doc = "Field `CPTR` reader - This register specifies the location of vector C within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
pub type CptrR = crate::FieldReader<u16>;
#[doc = "Field `CPTR` writer - This register specifies the location of vector C within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
pub type CptrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - This register specifies the location of vector C within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn cptr(&self) -> CptrR {
        CptrR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - This register specifies the location of vector C within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn cptr(&mut self) -> CptrW<CptrSpec> {
        CptrW::new(self, 0)
    }
}
#[doc = "PKA vector C address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::Reg::read) this register and get [`cptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CptrSpec;
impl crate::RegisterSpec for CptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cptr::R`](R) reader structure"]
impl crate::Readable for CptrSpec {}
#[doc = "`write(|w| ..)` method takes [`cptr::W`](W) writer structure"]
impl crate::Writable for CptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPTR to value 0"]
impl crate::Resettable for CptrSpec {
    const RESET_VALUE: u32 = 0;
}
