#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `DATAIC` writer - Data interrupt clear Writing 1 to this bit clears the DATARIS bit in the I2CSRIS register and the DATAMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
pub type DataicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTIC` writer - Start condition interrupt vlear Writing 1 to this bit clears the STARTRIS bit in the I2CSRIS register and the STARTMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
pub type StarticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPIC` writer - Stop condition interrupt clear Writing 1 to this bit clears the STOPRIS bit in the I2CSRIS register and the STOPMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
pub type StopicW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Data interrupt clear Writing 1 to this bit clears the DATARIS bit in the I2CSRIS register and the DATAMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
    #[inline(always)]
    pub fn dataic(&mut self) -> DataicW<IcrSpec> {
        DataicW::new(self, 0)
    }
    #[doc = "Bit 1 - Start condition interrupt vlear Writing 1 to this bit clears the STARTRIS bit in the I2CSRIS register and the STARTMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
    #[inline(always)]
    pub fn startic(&mut self) -> StarticW<IcrSpec> {
        StarticW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop condition interrupt clear Writing 1 to this bit clears the STOPRIS bit in the I2CSRIS register and the STOPMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
    #[inline(always)]
    pub fn stopic(&mut self) -> StopicW<IcrSpec> {
        StopicW::new(self, 2)
    }
}
#[doc = "I2C slave interrupt clear This register clears the raw interrupt. A read of this register returns no meaningful data.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
