#[doc = "Register `TBILR` reader"]
pub type R = crate::R<TBILR_SPEC>;
#[doc = "Register `TBILR` writer"]
pub type W = crate::W<TBILR_SPEC>;
#[doc = "Field `TBILR` reader - GPTM B interval load register"]
pub type TBILR_R = crate::FieldReader<u16>;
#[doc = "Field `TBILR` writer - GPTM B interval load register"]
pub type TBILR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM B interval load register"]
    #[inline(always)]
    pub fn tbilr(&self) -> TBILR_R {
        TBILR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM B interval load register"]
    #[inline(always)]
    #[must_use]
    pub fn tbilr(&mut self) -> TBILR_W<TBILR_SPEC, 0> {
        TBILR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPTM Timer B interval load When the Timer is counting down, this register is used to load the starting count value into the Timer. When the Timer is counting up, this register sets the upper bound for the time-out event. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAILR register. Reads from this register return the current value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the load value. Bits \\[31:16\\]
are reserved in both cases.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbilr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbilr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBILR_SPEC;
impl crate::RegisterSpec for TBILR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbilr::R`](R) reader structure"]
impl crate::Readable for TBILR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbilr::W`](W) writer structure"]
impl crate::Writable for TBILR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBILR to value 0"]
impl crate::Resettable for TBILR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
