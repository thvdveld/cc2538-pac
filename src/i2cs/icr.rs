#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICR_SPEC>;
#[doc = "Field `DATAIC` writer - Data interrupt clear Writing 1 to this bit clears the DATARIS bit in the I2CSRIS register and the DATAMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
pub type DATAIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTIC` writer - Start condition interrupt vlear Writing 1 to this bit clears the STARTRIS bit in the I2CSRIS register and the STARTMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
pub type STARTIC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPIC` writer - Stop condition interrupt clear Writing 1 to this bit clears the STOPRIS bit in the I2CSRIS register and the STOPMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
pub type STOPIC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Data interrupt clear Writing 1 to this bit clears the DATARIS bit in the I2CSRIS register and the DATAMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
    #[inline(always)]
    #[must_use]
    pub fn dataic(&mut self) -> DATAIC_W<ICR_SPEC> {
        DATAIC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Start condition interrupt vlear Writing 1 to this bit clears the STARTRIS bit in the I2CSRIS register and the STARTMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
    #[inline(always)]
    #[must_use]
    pub fn startic(&mut self) -> STARTIC_W<ICR_SPEC> {
        STARTIC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Stop condition interrupt clear Writing 1 to this bit clears the STOPRIS bit in the I2CSRIS register and the STOPMIS bit in the I2CSMIS register. A read of this register returns no meaningful data."]
    #[inline(always)]
    #[must_use]
    pub fn stopic(&mut self) -> STOPIC_W<ICR_SPEC> {
        STOPIC_W::new(self, 2)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "I2C slave interrupt clear This register clears the raw interrupt. A read of this register returns no meaningful data.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: u32 = 0;
}
