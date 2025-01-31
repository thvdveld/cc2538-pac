#[doc = "Register `TAV` reader"]
pub type R = crate::R<TavSpec>;
#[doc = "Register `TAV` writer"]
pub type W = crate::W<TavSpec>;
#[doc = "Field `TAV` reader - GPTM Timer A register"]
pub type TavR = crate::FieldReader<u32>;
#[doc = "Field `TAV` writer - GPTM Timer A register"]
pub type TavW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPTM Timer A register"]
    #[inline(always)]
    pub fn tav(&self) -> TavR {
        TavR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPTM Timer A register"]
    #[inline(always)]
    pub fn tav(&mut self) -> TavW<TavSpec> {
        TavW::new(self, 0)
    }
}
#[doc = "GPTM Timer A value When read, this register shows the current, free-running value of Timer A in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry when using the snapshot feature with the periodic operating mode. When written, the value written into this register is loaded into the TAR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, TAV appears as a 32-bit register (the upper 16-bits correspond to the contents of the GPTM Timer B Value (TBV) register). In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tav::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tav::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TavSpec;
impl crate::RegisterSpec for TavSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tav::R`](R) reader structure"]
impl crate::Readable for TavSpec {}
#[doc = "`write(|w| ..)` method takes [`tav::W`](W) writer structure"]
impl crate::Writable for TavSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TAV to value 0"]
impl crate::Resettable for TavSpec {
    const RESET_VALUE: u32 = 0;
}
