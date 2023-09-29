#[doc = "Register `CLK_SSIIN_SSI1` reader"]
pub type R = crate::R<CLK_SSIIN_SSI1_SPEC>;
#[doc = "Register `CLK_SSIIN_SSI1` writer"]
pub type W = crate::W<CLK_SSIIN_SSI1_SPEC>;
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as SSI1 CLK_SSIN 1: PA1 selected as SSI1 CLK_SSIN ... 31: PD7 selected as SSI1 CLK_SSIN"]
pub type INPUT_SEL_R = crate::FieldReader;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as SSI1 CLK_SSIN 1: PA1 selected as SSI1 CLK_SSIN ... 31: PD7 selected as SSI1 CLK_SSIN"]
pub type INPUT_SEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI1 CLK_SSIN 1: PA1 selected as SSI1 CLK_SSIN ... 31: PD7 selected as SSI1 CLK_SSIN"]
    #[inline(always)]
    pub fn input_sel(&self) -> INPUT_SEL_R {
        INPUT_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI1 CLK_SSIN 1: PA1 selected as SSI1 CLK_SSIN ... 31: PD7 selected as SSI1 CLK_SSIN"]
    #[inline(always)]
    #[must_use]
    pub fn input_sel(&mut self) -> INPUT_SEL_W<CLK_SSIIN_SSI1_SPEC, 0> {
        INPUT_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI1 CLK_SSIN.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ssiin_ssi1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ssiin_ssi1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SSIIN_SSI1_SPEC;
impl crate::RegisterSpec for CLK_SSIIN_SSI1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_ssiin_ssi1::R`](R) reader structure"]
impl crate::Readable for CLK_SSIIN_SSI1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_ssiin_ssi1::W`](W) writer structure"]
impl crate::Writable for CLK_SSIIN_SSI1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_SSIIN_SSI1 to value 0"]
impl crate::Resettable for CLK_SSIIN_SSI1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
