#[doc = "Register `TBR` reader"]
pub type R = crate::R<TbrSpec>;
#[doc = "Field `TBR` reader - GPTM Timer B register"]
pub type TbrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B register"]
    #[inline(always)]
    pub fn tbr(&self) -> TbrR {
        TbrR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "GPTM Timer B This register shows the current value of the Timer B counter. When a GPTM is configured to one of the 32-bit modes, the contents of bits \\[15:0\\]
in this register are loaded into the upper 16 bits of the TAR register. Reads from this register return the current value of Timer B. In a 16-bit mode, bits 15:0 contain the value of the counter and bits \\[23:16\\]
contain the value of the prescaler in Input edge count, input edge time, and PWM modes, which is the upper 8 bits of the count. Bits \\[31:24\\]
always read as 0. To read the value of the prescaler in 16-bit, one-shot and periodic modes, read bits \\[23:16\\]
in the TBV register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbrSpec;
impl crate::RegisterSpec for TbrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbr::R`](R) reader structure"]
impl crate::Readable for TbrSpec {}
#[doc = "`reset()` method sets TBR to value 0"]
impl crate::Resettable for TbrSpec {
    const RESET_VALUE: u32 = 0;
}
