#[doc = "Register `SSIFSSIN_SSI0` reader"]
pub type R = crate::R<SsifssinSsi0Spec>;
#[doc = "Register `SSIFSSIN_SSI0` writer"]
pub type W = crate::W<SsifssinSsi0Spec>;
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
pub type InputSelR = crate::FieldReader;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
pub type InputSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
    #[inline(always)]
    pub fn input_sel(&self) -> InputSelR {
        InputSelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as SSI0 FSSIN 1: PA1 selected as SSI0 FSSIN ... 31: PD7 selected as SSI0 FSSIN"]
    #[inline(always)]
    #[must_use]
    pub fn input_sel(&mut self) -> InputSelW<SsifssinSsi0Spec> {
        InputSelW::new(self, 0)
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the SSI0 FSSIN.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssifssin_ssi0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssifssin_ssi0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsifssinSsi0Spec;
impl crate::RegisterSpec for SsifssinSsi0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssifssin_ssi0::R`](R) reader structure"]
impl crate::Readable for SsifssinSsi0Spec {}
#[doc = "`write(|w| ..)` method takes [`ssifssin_ssi0::W`](W) writer structure"]
impl crate::Writable for SsifssinSsi0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SSIFSSIN_SSI0 to value 0"]
impl crate::Resettable for SsifssinSsi0Spec {
    const RESET_VALUE: u32 = 0;
}
