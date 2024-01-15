#[doc = "Register `AFSEL` reader"]
pub type R = crate::R<AFSEL_SPEC>;
#[doc = "Register `AFSEL` writer"]
pub type W = crate::W<AFSEL_SPEC>;
#[doc = "Field `AFSEL` reader - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
pub type AFSEL_R = crate::FieldReader;
#[doc = "Field `AFSEL` writer - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
pub type AFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
    #[inline(always)]
    pub fn afsel(&self) -> AFSEL_R {
        AFSEL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit set: Enables hardware (peripheral) control mode Bit cleared: Enables software control mode"]
    #[inline(always)]
    #[must_use]
    pub fn afsel(&mut self) -> AFSEL_W<AFSEL_SPEC> {
        AFSEL_W::new(self, 0)
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
#[doc = "The AFSEL register is the mode control select register. Writing 1 to any bit in this register selects the hardware (peripheral) control for the corresponding GPIO line. All bits are cleared by a reset, therefore no GPIO line is set to hardware control by default.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFSEL_SPEC;
impl crate::RegisterSpec for AFSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afsel::R`](R) reader structure"]
impl crate::Readable for AFSEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`afsel::W`](W) writer structure"]
impl crate::Writable for AFSEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFSEL to value 0"]
impl crate::Resettable for AFSEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
