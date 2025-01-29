#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `IC` writer - Interrupt clear Writing 1 to this bit clears the RIS bit in the I2CMRIS register and the MIS bit in the I2CMMIS register. Reading this register returns no meaningful data."]
pub type IcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Interrupt clear Writing 1 to this bit clears the RIS bit in the I2CMRIS register and the MIS bit in the I2CMMIS register. Reading this register returns no meaningful data."]
    #[inline(always)]
    pub fn ic(&mut self) -> IcW<IcrSpec> {
        IcW::new(self, 0)
    }
}
#[doc = "I2C master interrupt clear This register clears the raw and masked interrupts.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {
    const RESET_VALUE: u32 = 0;
}
