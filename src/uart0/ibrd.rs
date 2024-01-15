#[doc = "Register `IBRD` reader"]
pub type R = crate::R<IBRD_SPEC>;
#[doc = "Register `IBRD` writer"]
pub type W = crate::W<IBRD_SPEC>;
#[doc = "Field `DIVINT` reader - Integer baud-rate divisor"]
pub type DIVINT_R = crate::FieldReader<u16>;
#[doc = "Field `DIVINT` writer - Integer baud-rate divisor"]
pub type DIVINT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Integer baud-rate divisor"]
    #[inline(always)]
    pub fn divint(&self) -> DIVINT_R {
        DIVINT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Integer baud-rate divisor"]
    #[inline(always)]
    #[must_use]
    pub fn divint(&mut self) -> DIVINT_W<IBRD_SPEC> {
        DIVINT_W::new(self, 0)
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
#[doc = "UART integer baud-rate divisor The IBRD register is the integer part of the baud-rate divisor value. All the bits are cleared on reset. The minimum possible divide ratio is 1 (when IBRD = 0), in which case the FBRD register is ignored. When changing the IBRD register, the new value does not take effect until transmission or reception of the current character is complete. Any changes to the baud-rate divisor must be followed by a write to the LCRH register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBRD_SPEC;
impl crate::RegisterSpec for IBRD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibrd::R`](R) reader structure"]
impl crate::Readable for IBRD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ibrd::W`](W) writer structure"]
impl crate::Writable for IBRD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IBRD to value 0"]
impl crate::Resettable for IBRD_SPEC {
    const RESET_VALUE: u32 = 0;
}
