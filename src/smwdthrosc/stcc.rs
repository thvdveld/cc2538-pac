#[doc = "Register `STCC` reader"]
pub type R = crate::R<STCC_SPEC>;
#[doc = "Register `STCC` writer"]
pub type W = crate::W<STCC_SPEC>;
#[doc = "Field `PIN` reader - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
pub type PIN_R = crate::FieldReader;
#[doc = "Field `PIN` writer - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
pub type PIN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `PORT` reader - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
pub type PORT_R = crate::FieldReader;
#[doc = "Field `PORT` writer - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
pub type PORT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pin select Valid settings are 1-7 when either port A, B, C, or D is selected."]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PIN_W<STCC_SPEC, 0> {
        PIN_W::new(self)
    }
    #[doc = "Bits 3:5 - Port select Valid settings are 0-3, all others inhibit any capture from occurring 000: Port A selected 001: Port B selected 010: Port C selected 011: Port D selected"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PORT_W<STCC_SPEC, 3> {
        PORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Sleep Timer Capture control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stcc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stcc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STCC_SPEC;
impl crate::RegisterSpec for STCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stcc::R`](R) reader structure"]
impl crate::Readable for STCC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`stcc::W`](W) writer structure"]
impl crate::Writable for STCC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCC to value 0"]
impl crate::Resettable for STCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
