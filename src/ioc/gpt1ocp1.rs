#[doc = "Register `GPT1OCP1` reader"]
pub type R = crate::R<Gpt1ocp1Spec>;
#[doc = "Register `GPT1OCP1` writer"]
pub type W = crate::W<Gpt1ocp1Spec>;
#[doc = "Field `INPUT_SEL` reader - 0: PA0 selected as GPT1OCP1 1: PA1 selected as GPT1OCP1 ... 31: PD7 selected as GPT1OCP1"]
pub type InputSelR = crate::FieldReader;
#[doc = "Field `INPUT_SEL` writer - 0: PA0 selected as GPT1OCP1 1: PA1 selected as GPT1OCP1 ... 31: PD7 selected as GPT1OCP1"]
pub type InputSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - 0: PA0 selected as GPT1OCP1 1: PA1 selected as GPT1OCP1 ... 31: PD7 selected as GPT1OCP1"]
    #[inline(always)]
    pub fn input_sel(&self) -> InputSelR {
        InputSelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 0: PA0 selected as GPT1OCP1 1: PA1 selected as GPT1OCP1 ... 31: PD7 selected as GPT1OCP1"]
    #[inline(always)]
    pub fn input_sel(&mut self) -> InputSelW<Gpt1ocp1Spec> {
        InputSelW::new(self, 0)
    }
}
#[doc = "Selects one of the 32 pins on the four 8-pin I/O-ports (port A, port B, port C, and port D) to be the GPT1OCP1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpt1ocp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpt1ocp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpt1ocp1Spec;
impl crate::RegisterSpec for Gpt1ocp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpt1ocp1::R`](R) reader structure"]
impl crate::Readable for Gpt1ocp1Spec {}
#[doc = "`write(|w| ..)` method takes [`gpt1ocp1::W`](W) writer structure"]
impl crate::Writable for Gpt1ocp1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPT1OCP1 to value 0"]
impl crate::Resettable for Gpt1ocp1Spec {
    const RESET_VALUE: u32 = 0;
}
