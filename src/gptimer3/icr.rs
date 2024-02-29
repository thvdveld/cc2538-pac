#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `TATOCINT` reader - GPTM Timer A time-out interrupt clear"]
pub type TatocintR = crate::BitReader;
#[doc = "Field `TATOCINT` writer - GPTM Timer A time-out interrupt clear"]
pub type TatocintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAMCINT` reader - GPTM Timer A capture match interrupt clear"]
pub type CamcintR = crate::BitReader;
#[doc = "Field `CAMCINT` writer - GPTM Timer A capture match interrupt clear"]
pub type CamcintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAECINT` reader - GPTM Timer A capture event Interrupt clear"]
pub type CaecintR = crate::BitReader;
#[doc = "Field `CAECINT` writer - GPTM Timer A capture event Interrupt clear"]
pub type CaecintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMCINT` reader - GPTM Timer A match interrupt clear"]
pub type TamcintR = crate::BitReader;
#[doc = "Field `TAMCINT` writer - GPTM Timer A match interrupt clear"]
pub type TamcintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBTOCINT` reader - GPTM Timer B time-out interrupt clear"]
pub type TbtocintR = crate::BitReader;
#[doc = "Field `TBTOCINT` writer - GPTM Timer B time-out interrupt clear"]
pub type TbtocintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBMCINT` reader - GPTM Timer B capture match interrupt clear"]
pub type CbmcintR = crate::BitReader;
#[doc = "Field `CBMCINT` writer - GPTM Timer B capture match interrupt clear"]
pub type CbmcintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBECINT` reader - GPTM Timer B capture event Interrupt clear"]
pub type CbecintR = crate::BitReader;
#[doc = "Field `CBECINT` writer - GPTM Timer B capture event Interrupt clear"]
pub type CbecintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBMCINT` reader - GPTM Timer B match interrupt clear"]
pub type TbmcintR = crate::BitReader;
#[doc = "Field `TBMCINT` writer - GPTM Timer B match interrupt clear"]
pub type TbmcintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUECINT` reader - GPTM write update error interrupt clear"]
pub type WuecintR = crate::BitReader;
#[doc = "Field `WUECINT` writer - GPTM write update error interrupt clear"]
pub type WuecintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt clear"]
    #[inline(always)]
    pub fn tatocint(&self) -> TatocintR {
        TatocintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt clear"]
    #[inline(always)]
    pub fn camcint(&self) -> CamcintR {
        CamcintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event Interrupt clear"]
    #[inline(always)]
    pub fn caecint(&self) -> CaecintR {
        CaecintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt clear"]
    #[inline(always)]
    pub fn tamcint(&self) -> TamcintR {
        TamcintR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt clear"]
    #[inline(always)]
    pub fn tbtocint(&self) -> TbtocintR {
        TbtocintR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt clear"]
    #[inline(always)]
    pub fn cbmcint(&self) -> CbmcintR {
        CbmcintR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event Interrupt clear"]
    #[inline(always)]
    pub fn cbecint(&self) -> CbecintR {
        CbecintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPTM Timer B match interrupt clear"]
    #[inline(always)]
    pub fn tbmcint(&self) -> TbmcintR {
        TbmcintR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - GPTM write update error interrupt clear"]
    #[inline(always)]
    pub fn wuecint(&self) -> WuecintR {
        WuecintR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPTM Timer A time-out interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn tatocint(&mut self) -> TatocintW<IcrSpec> {
        TatocintW::new(self, 0)
    }
    #[doc = "Bit 1 - GPTM Timer A capture match interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn camcint(&mut self) -> CamcintW<IcrSpec> {
        CamcintW::new(self, 1)
    }
    #[doc = "Bit 2 - GPTM Timer A capture event Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn caecint(&mut self) -> CaecintW<IcrSpec> {
        CaecintW::new(self, 2)
    }
    #[doc = "Bit 4 - GPTM Timer A match interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn tamcint(&mut self) -> TamcintW<IcrSpec> {
        TamcintW::new(self, 4)
    }
    #[doc = "Bit 8 - GPTM Timer B time-out interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn tbtocint(&mut self) -> TbtocintW<IcrSpec> {
        TbtocintW::new(self, 8)
    }
    #[doc = "Bit 9 - GPTM Timer B capture match interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cbmcint(&mut self) -> CbmcintW<IcrSpec> {
        CbmcintW::new(self, 9)
    }
    #[doc = "Bit 10 - GPTM Timer B capture event Interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn cbecint(&mut self) -> CbecintW<IcrSpec> {
        CbecintW::new(self, 10)
    }
    #[doc = "Bit 11 - GPTM Timer B match interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn tbmcint(&mut self) -> TbmcintW<IcrSpec> {
        TbmcintW::new(self, 11)
    }
    #[doc = "Bit 16 - GPTM write update error interrupt clear"]
    #[inline(always)]
    #[must_use]
    pub fn wuecint(&mut self) -> WuecintW<IcrSpec> {
        WuecintW::new(self, 16)
    }
}
#[doc = "GPTM interrupt clear This register is used to clear the status bits in the RIS and MIS registers. Writing 1 to a bit clears the corresponding bit in the RIS and MIS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
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
