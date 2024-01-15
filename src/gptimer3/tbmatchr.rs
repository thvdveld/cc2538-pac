#[doc = "Register `TBMATCHR` reader"]
pub type R = crate::R<TBMATCHR_SPEC>;
#[doc = "Register `TBMATCHR` writer"]
pub type W = crate::W<TBMATCHR_SPEC>;
#[doc = "Field `TBMR` reader - GPTM Timer B match register"]
pub type TBMR_R = crate::FieldReader<u16>;
#[doc = "Field `TBMR` writer - GPTM Timer B match register"]
pub type TBMR_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B match register"]
    #[inline(always)]
    pub fn tbmr(&self) -> TBMR_R {
        TBMR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer B match register"]
    #[inline(always)]
    #[must_use]
    pub fn tbmr(&mut self) -> TBMR_W<TBMATCHR_SPEC> {
        TBMR_W::new(self, 0)
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
#[doc = "PTM Timer B match This register is loaded with a match value. Interrupts can be generated when the Timer value is equal to the value in this register in one-shot or periodic mode. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAMATCHR register. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits \\[15:0\\]
are used for the match value. Bits \\[31:16\\]
are reserved in both cases.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tbmatchr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tbmatchr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TBMATCHR_SPEC;
impl crate::RegisterSpec for TBMATCHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbmatchr::R`](R) reader structure"]
impl crate::Readable for TBMATCHR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tbmatchr::W`](W) writer structure"]
impl crate::Writable for TBMATCHR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBMATCHR to value 0"]
impl crate::Resettable for TBMATCHR_SPEC {
    const RESET_VALUE: u32 = 0;
}
