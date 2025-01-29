#[doc = "Register `RFST` reader"]
pub type R = crate::R<RfstSpec>;
#[doc = "Register `RFST` writer"]
pub type W = crate::W<RfstSpec>;
#[doc = "Field `INSTR` reader - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
pub type InstrR = crate::FieldReader;
#[doc = "Field `INSTR` writer - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
pub type InstrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
    #[inline(always)]
    pub fn instr(&self) -> InstrR {
        InstrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
    #[inline(always)]
    pub fn instr(&mut self) -> InstrW<RfstSpec> {
        InstrW::new(self, 0)
    }
}
#[doc = "RF CSMA-CA/strobe processor\n\nYou can [`read`](crate::Reg::read) this register and get [`rfst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfstSpec;
impl crate::RegisterSpec for RfstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfst::R`](R) reader structure"]
impl crate::Readable for RfstSpec {}
#[doc = "`write(|w| ..)` method takes [`rfst::W`](W) writer structure"]
impl crate::Writable for RfstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFST to value 0"]
impl crate::Resettable for RfstSpec {
    const RESET_VALUE: u32 = 0;
}
