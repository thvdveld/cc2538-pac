#[doc = "Register `FWDATA` reader"]
pub type R = crate::R<FwdataSpec>;
#[doc = "Register `FWDATA` writer"]
pub type W = crate::W<FwdataSpec>;
#[doc = "Field `FWDATA` reader - 32-bit flash write data Writes to this register are accepted only during a flash write sequence; that is, writes to this register after having written 1 to the FCTL.WRITE bit. New 32-bit data is written only if FCTL.FULL = 0."]
pub type FwdataR = crate::FieldReader<u32>;
#[doc = "Field `FWDATA` writer - 32-bit flash write data Writes to this register are accepted only during a flash write sequence; that is, writes to this register after having written 1 to the FCTL.WRITE bit. New 32-bit data is written only if FCTL.FULL = 0."]
pub type FwdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 32-bit flash write data Writes to this register are accepted only during a flash write sequence; that is, writes to this register after having written 1 to the FCTL.WRITE bit. New 32-bit data is written only if FCTL.FULL = 0."]
    #[inline(always)]
    pub fn fwdata(&self) -> FwdataR {
        FwdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit flash write data Writes to this register are accepted only during a flash write sequence; that is, writes to this register after having written 1 to the FCTL.WRITE bit. New 32-bit data is written only if FCTL.FULL = 0."]
    #[inline(always)]
    pub fn fwdata(&mut self) -> FwdataW<FwdataSpec> {
        FwdataW::new(self, 0)
    }
}
#[doc = "Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR.\n\nYou can [`read`](crate::Reg::read) this register and get [`fwdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FwdataSpec;
impl crate::RegisterSpec for FwdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fwdata::R`](R) reader structure"]
impl crate::Readable for FwdataSpec {}
#[doc = "`write(|w| ..)` method takes [`fwdata::W`](W) writer structure"]
impl crate::Writable for FwdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FWDATA to value 0"]
impl crate::Resettable for FwdataSpec {
    const RESET_VALUE: u32 = 0;
}
