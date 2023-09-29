#[doc = "Register `SYNC` reader"]
pub type R = crate::R<SYNC_SPEC>;
#[doc = "Register `SYNC` writer"]
pub type W = crate::W<SYNC_SPEC>;
#[doc = "Field `SYNC0` reader - Synchronize GPTM0 0x0: GPTM0 is not affected. 0x1: A time-out event for Timer A of GPTM0 is triggered. 0x2: A time-out event for Timer B of GPTM0 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM0 is triggered."]
pub type SYNC0_R = crate::FieldReader;
#[doc = "Field `SYNC0` writer - Synchronize GPTM0 0x0: GPTM0 is not affected. 0x1: A time-out event for Timer A of GPTM0 is triggered. 0x2: A time-out event for Timer B of GPTM0 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM0 is triggered."]
pub type SYNC0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SYNC1` reader - Synchronize GPTM1 0x0: GPTM1 is not affected. 0x1: A time-out event for Timer A of GPTM1 is triggered. 0x2: A time-out event for Timer B of GPTM1 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM1 is triggered."]
pub type SYNC1_R = crate::FieldReader;
#[doc = "Field `SYNC1` writer - Synchronize GPTM1 0x0: GPTM1 is not affected. 0x1: A time-out event for Timer A of GPTM1 is triggered. 0x2: A time-out event for Timer B of GPTM1 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM1 is triggered."]
pub type SYNC1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SYNC2` reader - Synchronize GPTM2 0x0: GPTM2 is not affected. 0x1: A time-out event for Timer A of GPTM2 is triggered. 0x2: A time-out event for Timer B of GPTM2 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM2 is triggered."]
pub type SYNC2_R = crate::FieldReader;
#[doc = "Field `SYNC2` writer - Synchronize GPTM2 0x0: GPTM2 is not affected. 0x1: A time-out event for Timer A of GPTM2 is triggered. 0x2: A time-out event for Timer B of GPTM2 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM2 is triggered."]
pub type SYNC2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `SYNC3` reader - Synchronize GPTM3 0x0: GPTM3 is not affected. 0x1: A time-out event for Timer A of GPTM3 is triggered. 0x2: A time-out event for Timer B of GPTM3 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM3 is triggered."]
pub type SYNC3_R = crate::FieldReader;
#[doc = "Field `SYNC3` writer - Synchronize GPTM3 0x0: GPTM3 is not affected. 0x1: A time-out event for Timer A of GPTM3 is triggered. 0x2: A time-out event for Timer B of GPTM3 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM3 is triggered."]
pub type SYNC3_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Synchronize GPTM0 0x0: GPTM0 is not affected. 0x1: A time-out event for Timer A of GPTM0 is triggered. 0x2: A time-out event for Timer B of GPTM0 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM0 is triggered."]
    #[inline(always)]
    pub fn sync0(&self) -> SYNC0_R {
        SYNC0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Synchronize GPTM1 0x0: GPTM1 is not affected. 0x1: A time-out event for Timer A of GPTM1 is triggered. 0x2: A time-out event for Timer B of GPTM1 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM1 is triggered."]
    #[inline(always)]
    pub fn sync1(&self) -> SYNC1_R {
        SYNC1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronize GPTM2 0x0: GPTM2 is not affected. 0x1: A time-out event for Timer A of GPTM2 is triggered. 0x2: A time-out event for Timer B of GPTM2 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM2 is triggered."]
    #[inline(always)]
    pub fn sync2(&self) -> SYNC2_R {
        SYNC2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Synchronize GPTM3 0x0: GPTM3 is not affected. 0x1: A time-out event for Timer A of GPTM3 is triggered. 0x2: A time-out event for Timer B of GPTM3 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM3 is triggered."]
    #[inline(always)]
    pub fn sync3(&self) -> SYNC3_R {
        SYNC3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Synchronize GPTM0 0x0: GPTM0 is not affected. 0x1: A time-out event for Timer A of GPTM0 is triggered. 0x2: A time-out event for Timer B of GPTM0 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM0 is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sync0(&mut self) -> SYNC0_W<SYNC_SPEC, 0> {
        SYNC0_W::new(self)
    }
    #[doc = "Bits 2:3 - Synchronize GPTM1 0x0: GPTM1 is not affected. 0x1: A time-out event for Timer A of GPTM1 is triggered. 0x2: A time-out event for Timer B of GPTM1 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM1 is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sync1(&mut self) -> SYNC1_W<SYNC_SPEC, 2> {
        SYNC1_W::new(self)
    }
    #[doc = "Bits 4:5 - Synchronize GPTM2 0x0: GPTM2 is not affected. 0x1: A time-out event for Timer A of GPTM2 is triggered. 0x2: A time-out event for Timer B of GPTM2 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM2 is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sync2(&mut self) -> SYNC2_W<SYNC_SPEC, 4> {
        SYNC2_W::new(self)
    }
    #[doc = "Bits 6:7 - Synchronize GPTM3 0x0: GPTM3 is not affected. 0x1: A time-out event for Timer A of GPTM3 is triggered. 0x2: A time-out event for Timer B of GPTM3 is triggered. 0x3: A time-out event for Timer A and Timer B of GPTM3 is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn sync3(&mut self) -> SYNC3_W<SYNC_SPEC, 6> {
        SYNC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "GPTM synchronize Note: This register is implemented on GPTM 0 base address only. This register does however, allow software to synchronize a number of timers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYNC_SPEC;
impl crate::RegisterSpec for SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sync::R`](R) reader structure"]
impl crate::Readable for SYNC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sync::W`](W) writer structure"]
impl crate::Writable for SYNC_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
