#[doc = "Register `RFST` reader"]
pub type R = crate::R<RFST_SPEC>;
#[doc = "Register `RFST` writer"]
pub type W = crate::W<RFST_SPEC>;
#[doc = "Field `INSTR` reader - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
pub type INSTR_R = crate::FieldReader;
#[doc = "Field `INSTR` writer - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
pub type INSTR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
    #[inline(always)]
    pub fn instr(&self) -> INSTR_R {
        INSTR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data written to this register is written to the CSP instruction memory. Reading this register returns the CSP instruction currently being executed."]
    #[inline(always)]
    #[must_use]
    pub fn instr(&mut self) -> INSTR_W<RFST_SPEC> {
        INSTR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RF CSMA-CA/strobe processor\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RFST_SPEC;
impl crate::RegisterSpec for RFST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfst::R`](R) reader structure"]
impl crate::Readable for RFST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rfst::W`](W) writer structure"]
impl crate::Writable for RFST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RFST to value 0"]
impl crate::Resettable for RFST_SPEC {
    const RESET_VALUE: u32 = 0;
}
