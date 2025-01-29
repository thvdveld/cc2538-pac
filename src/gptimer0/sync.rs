#[doc = "Register `SYNC` reader"]
pub type R = crate::R<SyncSpec>;
#[doc = "Register `SYNC` writer"]
pub type W = crate::W<SyncSpec>;
#[doc = "Field `SYNC0` reader - Synchronize GPTM0 0x0: GPTM0 is not affected. 0x1: A time-out event for Timer A of GPTM0 is triggered. 0x2: A time-out event for Timer B of GPTM0 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM0 is triggered."]
pub type Sync0R = crate::FieldReader;
#[doc = "Field `SYNC0` writer - Synchronize GPTM0 0x0: GPTM0 is not affected. 0x1: A time-out event for Timer A of GPTM0 is triggered. 0x2: A time-out event for Timer B of GPTM0 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM0 is triggered."]
pub type Sync0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNC1` reader - Synchronize GPTM1 0x0: GPTM1 is not affected. 0x1: A time-out event for Timer A of GPTM1 is triggered. 0x2: A time-out event for Timer B of GPTM1 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM1 is triggered."]
pub type Sync1R = crate::FieldReader;
#[doc = "Field `SYNC1` writer - Synchronize GPTM1 0x0: GPTM1 is not affected. 0x1: A time-out event for Timer A of GPTM1 is triggered. 0x2: A time-out event for Timer B of GPTM1 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM1 is triggered."]
pub type Sync1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNC2` reader - Synchronize GPTM2 0x0: GPTM2 is not affected. 0x1: A time-out event for Timer A of GPTM2 is triggered. 0x2: A time-out event for Timer B of GPTM2 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM2 is triggered."]
pub type Sync2R = crate::FieldReader;
#[doc = "Field `SYNC2` writer - Synchronize GPTM2 0x0: GPTM2 is not affected. 0x1: A time-out event for Timer A of GPTM2 is triggered. 0x2: A time-out event for Timer B of GPTM2 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM2 is triggered."]
pub type Sync2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYNC3` reader - Synchronize GPTM3 0x0: GPTM3 is not affected. 0x1: A time-out event for Timer A of GPTM3 is triggered. 0x2: A time-out event for Timer B of GPTM3 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM3 is triggered."]
pub type Sync3R = crate::FieldReader;
#[doc = "Field `SYNC3` writer - Synchronize GPTM3 0x0: GPTM3 is not affected. 0x1: A time-out event for Timer A of GPTM3 is triggered. 0x2: A time-out event for Timer B of GPTM3 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM3 is triggered."]
pub type Sync3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Synchronize GPTM0 0x0: GPTM0 is not affected. 0x1: A time-out event for Timer A of GPTM0 is triggered. 0x2: A time-out event for Timer B of GPTM0 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM0 is triggered."]
    #[inline(always)]
    pub fn sync0(&self) -> Sync0R {
        Sync0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Synchronize GPTM1 0x0: GPTM1 is not affected. 0x1: A time-out event for Timer A of GPTM1 is triggered. 0x2: A time-out event for Timer B of GPTM1 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM1 is triggered."]
    #[inline(always)]
    pub fn sync1(&self) -> Sync1R {
        Sync1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronize GPTM2 0x0: GPTM2 is not affected. 0x1: A time-out event for Timer A of GPTM2 is triggered. 0x2: A time-out event for Timer B of GPTM2 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM2 is triggered."]
    #[inline(always)]
    pub fn sync2(&self) -> Sync2R {
        Sync2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Synchronize GPTM3 0x0: GPTM3 is not affected. 0x1: A time-out event for Timer A of GPTM3 is triggered. 0x2: A time-out event for Timer B of GPTM3 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM3 is triggered."]
    #[inline(always)]
    pub fn sync3(&self) -> Sync3R {
        Sync3R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Synchronize GPTM0 0x0: GPTM0 is not affected. 0x1: A time-out event for Timer A of GPTM0 is triggered. 0x2: A time-out event for Timer B of GPTM0 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM0 is triggered."]
    #[inline(always)]
    pub fn sync0(&mut self) -> Sync0W<SyncSpec> {
        Sync0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Synchronize GPTM1 0x0: GPTM1 is not affected. 0x1: A time-out event for Timer A of GPTM1 is triggered. 0x2: A time-out event for Timer B of GPTM1 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM1 is triggered."]
    #[inline(always)]
    pub fn sync1(&mut self) -> Sync1W<SyncSpec> {
        Sync1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Synchronize GPTM2 0x0: GPTM2 is not affected. 0x1: A time-out event for Timer A of GPTM2 is triggered. 0x2: A time-out event for Timer B of GPTM2 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM2 is triggered."]
    #[inline(always)]
    pub fn sync2(&mut self) -> Sync2W<SyncSpec> {
        Sync2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Synchronize GPTM3 0x0: GPTM3 is not affected. 0x1: A time-out event for Timer A of GPTM3 is triggered. 0x2: A time-out event for Timer B of GPTM3 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM3 is triggered."]
    #[inline(always)]
    pub fn sync3(&mut self) -> Sync3W<SyncSpec> {
        Sync3W::new(self, 6)
    }
}
#[doc = "GPTM synchronize Note: This register is implemented on GPTM 0 base address only. This register does however, allow software to synchronize a number of timers.\n\nYou can [`read`](crate::Reg::read) this register and get [`sync::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncSpec;
impl crate::RegisterSpec for SyncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SyncSpec {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SyncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SyncSpec {
    const RESET_VALUE: u32 = 0;
}
