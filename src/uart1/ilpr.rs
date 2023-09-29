#[doc = "Register `ILPR` reader"]
pub type R = crate::R<ILPR_SPEC>;
#[doc = "Register `ILPR` writer"]
pub type W = crate::W<ILPR_SPEC>;
#[doc = "Field `ILPDVSR` reader - IrDA low-power divisor This field contains the 8-bit low-power divisor value."]
pub type ILPDVSR_R = crate::FieldReader;
#[doc = "Field `ILPDVSR` writer - IrDA low-power divisor This field contains the 8-bit low-power divisor value."]
pub type ILPDVSR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - IrDA low-power divisor This field contains the 8-bit low-power divisor value."]
    #[inline(always)]
    pub fn ilpdvsr(&self) -> ILPDVSR_R {
        ILPDVSR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - IrDA low-power divisor This field contains the 8-bit low-power divisor value."]
    #[inline(always)]
    #[must_use]
    pub fn ilpdvsr(&mut self) -> ILPDVSR_W<ILPR_SPEC, 0> {
        ILPDVSR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "UART IrDA low-power register The ILPR register stores the 8-bit low-power counter divisor value used to derive the low-power SIR pulse width clock by dividing down the system clock (SysClk). All the bits are cleared when reset. The internal IrLPBaud16 clock is generated by dividing down SysClk according to the low-power divisor value written to ILPR. The duration of SIR pulses generated when low-power mode is enabled is three times the period of the IrLPBaud16 clock. The low-power divisor value is calculated as follows: ILPDVSR = SysClk / FIrLPBaud16 where FIrLPBaud16 is nominally 1.8432 MHz The divisor must be programmed such that FIrLPBaud16 is in the range 1.42 MHz to 2.12 MHz, resulting in a low-power pulse duration of 1.41-2.11 us (three times the period of IrLPBaud16). The minimum frequency of IrLPBaud16 ensures that pulses less than one period of IrLPBaud16 are rejected, but pulses greater than 1.4 us are accepted as valid pulses. Note: Zero is an illegal value. Programming a zero value results in no IrLPBaud16 pulses being generated.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ilpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ilpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ILPR_SPEC;
impl crate::RegisterSpec for ILPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ilpr::R`](R) reader structure"]
impl crate::Readable for ILPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ilpr::W`](W) writer structure"]
impl crate::Writable for ILPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ILPR to value 0"]
impl crate::Resettable for ILPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
