#[doc = "Register `TBV` reader"]
pub type R = crate::R<TbvSpec>;
#[doc = "Register `TBV` writer"]
pub type W = crate::W<TbvSpec>;
#[doc = "Field `TBV` reader - GPTM Timer B register"]
pub type TbvR = crate::FieldReader<u16>;
#[doc = "Field `TBV` writer - GPTM Timer B register"]
pub type TbvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRE` reader - GPTM Timer B prescale register (16-bit mode)"]
pub type PreR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - GPTM Timer B register"]
    #[inline(always)]
    pub fn tbv(&self) -> TbvR {
        TbvR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - GPTM Timer B prescale register (16-bit mode)"]
    #[inline(always)]
    pub fn pre(&self) -> PreR {
        PreR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - GPTM Timer B register"]
    #[inline(always)]
    pub fn tbv(&mut self) -> TbvW<TbvSpec> {
        TbvW::new(self, 0)
    }
}
#[doc = "GPTM Timer B value When read, this register shows the current, free-running value of Timer B in all modes. Software can use this value to determine the time elapsed between an interrupt and the ISR entry. When written, the value written into this register is loaded into the TBR register on the next clock cycle. When a GPTM is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of the TAV register. Reads from this register return the current free-running value of Timer B. In a 16-bit mode, bits \\[15:0\\]
contain the value of the counter and bits \\[23:16\\]
contain the current, free-running value of the prescaler, which is the upper 8 bits of the count in input edge count, input edge time, PWM and one-shot or periodic up count modes. In one-shot or periodic down count modes, the prescaler stored in \\[23:16\\]
is a true prescaler, meaning bits \\[23:16\\]
count down before decrementing the value in bits \\[15:0\\]. The prescaler its \\[31:24\\]
always read as 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`tbv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbvSpec;
impl crate::RegisterSpec for TbvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbv::R`](R) reader structure"]
impl crate::Readable for TbvSpec {}
#[doc = "`write(|w| ..)` method takes [`tbv::W`](W) writer structure"]
impl crate::Writable for TbvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBV to value 0"]
impl crate::Resettable for TbvSpec {
    const RESET_VALUE: u32 = 0;
}
