#[doc = "Register `OIE` reader"]
pub type R = crate::R<OIE_SPEC>;
#[doc = "Register `OIE` writer"]
pub type W = crate::W<OIE_SPEC>;
#[doc = "Field `OUTEP1IE` reader - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP1IE_R = crate::BitReader;
#[doc = "Field `OUTEP1IE` writer - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP1IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEP2IE` reader - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP2IE_R = crate::BitReader;
#[doc = "Field `OUTEP2IE` writer - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP2IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEP3IE` reader - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP3IE_R = crate::BitReader;
#[doc = "Field `OUTEP3IE` writer - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP3IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEP4IE` reader - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP4IE_R = crate::BitReader;
#[doc = "Field `OUTEP4IE` writer - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP4IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTEP5IE` reader - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP5IE_R = crate::BitReader;
#[doc = "Field `OUTEP5IE` writer - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
pub type OUTEP5IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep1ie(&self) -> OUTEP1IE_R {
        OUTEP1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep2ie(&self) -> OUTEP2IE_R {
        OUTEP2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep3ie(&self) -> OUTEP3IE_R {
        OUTEP3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep4ie(&self) -> OUTEP4IE_R {
        OUTEP4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    pub fn outep5ie(&self) -> OUTEP5IE_R {
        OUTEP5IE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt enable for OUT endpoint 1 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn outep1ie(&mut self) -> OUTEP1IE_W<OIE_SPEC> {
        OUTEP1IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable for OUT endpoint 2 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn outep2ie(&mut self) -> OUTEP2IE_W<OIE_SPEC> {
        OUTEP2IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable for OUT endpoint 3 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn outep3ie(&mut self) -> OUTEP3IE_W<OIE_SPEC> {
        OUTEP3IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable for OUT endpoint 4 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn outep4ie(&mut self) -> OUTEP4IE_W<OIE_SPEC> {
        OUTEP4IE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt enable for OUT endpoint 5 0: Interrupt disabled 1: Interrupt enabled"]
    #[inline(always)]
    #[must_use]
    pub fn outep5ie(&mut self) -> OUTEP5IE_W<OIE_SPEC> {
        OUTEP5IE_W::new(self, 5)
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
#[doc = "Interrupt enable mask for OUT endpoints 1-5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OIE_SPEC;
impl crate::RegisterSpec for OIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oie::R`](R) reader structure"]
impl crate::Readable for OIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oie::W`](W) writer structure"]
impl crate::Writable for OIE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OIE to value 0"]
impl crate::Resettable for OIE_SPEC {
    const RESET_VALUE: u32 = 0;
}
