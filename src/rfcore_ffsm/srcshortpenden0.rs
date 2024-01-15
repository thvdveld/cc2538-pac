#[doc = "Register `SRCSHORTPENDEN0` reader"]
pub type R = crate::R<SRCSHORTPENDEN0_SPEC>;
#[doc = "Register `SRCSHORTPENDEN0` writer"]
pub type W = crate::W<SRCSHORTPENDEN0_SPEC>;
#[doc = "Field `SRCSHORTPENDEN0` reader - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type SRCSHORTPENDEN0_R = crate::FieldReader;
#[doc = "Field `SRCSHORTPENDEN0` writer - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
pub type SRCSHORTPENDEN0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    pub fn srcshortpenden0(&self) -> SRCSHORTPENDEN0_R {
        SRCSHORTPENDEN0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 8 LSBs of the 24-bit mask that enables or disables automatic pending for each of the 24 short addresses"]
    #[inline(always)]
    #[must_use]
    pub fn srcshortpenden0(&mut self) -> SRCSHORTPENDEN0_W<SRCSHORTPENDEN0_SPEC> {
        SRCSHORTPENDEN0_W::new(self, 0)
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
#[doc = "Source address matching control This register is stored in RAM; the reset value is undefined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srcshortpenden0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srcshortpenden0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRCSHORTPENDEN0_SPEC;
impl crate::RegisterSpec for SRCSHORTPENDEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcshortpenden0::R`](R) reader structure"]
impl crate::Readable for SRCSHORTPENDEN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`srcshortpenden0::W`](W) writer structure"]
impl crate::Writable for SRCSHORTPENDEN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRCSHORTPENDEN0 to value 0"]
impl crate::Resettable for SRCSHORTPENDEN0_SPEC {
    const RESET_VALUE: u32 = 0;
}
