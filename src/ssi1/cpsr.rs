#[doc = "Register `CPSR` reader"]
pub type R = crate::R<CpsrSpec>;
#[doc = "Register `CPSR` writer"]
pub type W = crate::W<CpsrSpec>;
#[doc = "Field `CPSDVSR` reader - SSI clock prescale divisor (R/W) Reset value: 0x0 This value must be an even number from 2 to 254, depending on the frequency of SSICLK. The LSB always returns zero on reads."]
pub type CpsdvsrR = crate::FieldReader;
#[doc = "Field `CPSDVSR` writer - SSI clock prescale divisor (R/W) Reset value: 0x0 This value must be an even number from 2 to 254, depending on the frequency of SSICLK. The LSB always returns zero on reads."]
pub type CpsdvsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SSI clock prescale divisor (R/W) Reset value: 0x0 This value must be an even number from 2 to 254, depending on the frequency of SSICLK. The LSB always returns zero on reads."]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CpsdvsrR {
        CpsdvsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SSI clock prescale divisor (R/W) Reset value: 0x0 This value must be an even number from 2 to 254, depending on the frequency of SSICLK. The LSB always returns zero on reads."]
    #[inline(always)]
    pub fn cpsdvsr(&mut self) -> CpsdvsrW<CpsrSpec> {
        CpsdvsrW::new(self, 0)
    }
}
#[doc = "The CPSR register specifies the division factor which is used to derive the SSIClk from the system clock. The clock is further divided by a value from 1 to 256, which is 1 + SCR. SCR is programmed in the SSICR0 register. The frequency of the SSIClk is defined by: SSIClk = SysClk / (CPSDVSR x (1 + SCR)) The value programmed into this register must be an even number between 2 and 254. The least-significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least-significant bit as zero.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpsrSpec;
impl crate::RegisterSpec for CpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsr::R`](R) reader structure"]
impl crate::Readable for CpsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsr::W`](W) writer structure"]
impl crate::Writable for CpsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSR to value 0"]
impl crate::Resettable for CpsrSpec {
    const RESET_VALUE: u32 = 0;
}
