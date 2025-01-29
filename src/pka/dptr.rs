#[doc = "Register `DPTR` reader"]
pub type R = crate::R<DptrSpec>;
#[doc = "Register `DPTR` writer"]
pub type W = crate::W<DptrSpec>;
#[doc = "Field `DPTR` reader - This register specifies the location of vector D within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
pub type DptrR = crate::FieldReader<u16>;
#[doc = "Field `DPTR` writer - This register specifies the location of vector D within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
pub type DptrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - This register specifies the location of vector D within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn dptr(&self) -> DptrR {
        DptrR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - This register specifies the location of vector D within the PKA RAM. Vectors are identified through the location of their least-significant 32-bit word. Note that bit \\[0\\]
must be zero to ensure that the vector starts at an 8-byte boundary."]
    #[inline(always)]
    pub fn dptr(&mut self) -> DptrW<DptrSpec> {
        DptrW::new(self, 0)
    }
}
#[doc = "PKA vector D address During execution of basic PKCP operations, this register is double buffered and can be written with a new value for the next operation; when not written, the value remains intact. During the execution of sequencer-controlled complex operations, this register may not be written and its value is undefined at the conclusion of the operation. The driver software cannot rely on the written value to remain intact.\n\nYou can [`read`](crate::Reg::read) this register and get [`dptr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dptr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DptrSpec;
impl crate::RegisterSpec for DptrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dptr::R`](R) reader structure"]
impl crate::Readable for DptrSpec {}
#[doc = "`write(|w| ..)` method takes [`dptr::W`](W) writer structure"]
impl crate::Writable for DptrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPTR to value 0"]
impl crate::Resettable for DptrSpec {
    const RESET_VALUE: u32 = 0;
}
