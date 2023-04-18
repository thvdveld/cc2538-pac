#[doc = "Register `PTME2` reader"]
pub struct R(crate::R<PTME2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTME2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTME2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTME2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PTME2` writer"]
pub struct W(crate::W<PTME2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTME2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PTME2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTME2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2C0TME` reader - I2C 0 test mode enable"]
pub type I2C0TME_R = crate::BitReader<bool>;
#[doc = "Field `I2C0TME` writer - I2C 0 test mode enable"]
pub type I2C0TME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTME2_SPEC, bool, O>;
#[doc = "Field `T0TME` reader - Timer0 test mode enable"]
pub type T0TME_R = crate::BitReader<bool>;
#[doc = "Field `T0TME` writer - Timer0 test mode enable"]
pub type T0TME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTME2_SPEC, bool, O>;
#[doc = "Field `T1TME` reader - Timer1 test mode enable"]
pub type T1TME_R = crate::BitReader<bool>;
#[doc = "Field `T1TME` writer - Timer1 test mode enable"]
pub type T1TME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTME2_SPEC, bool, O>;
#[doc = "Field `MTTME` reader - MacTimer test mode enable"]
pub type MTTME_R = crate::BitReader<bool>;
#[doc = "Field `MTTME` writer - MacTimer test mode enable"]
pub type MTTME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTME2_SPEC, bool, O>;
#[doc = "Field `T3TME` reader - Timer3 test mode enable"]
pub type T3TME_R = crate::BitReader<bool>;
#[doc = "Field `T3TME` writer - Timer3 test mode enable"]
pub type T3TME_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTME2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I2C 0 test mode enable"]
    #[inline(always)]
    pub fn i2c0tme(&self) -> I2C0TME_R {
        I2C0TME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Timer0 test mode enable"]
    #[inline(always)]
    pub fn t0tme(&self) -> T0TME_R {
        T0TME_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer1 test mode enable"]
    #[inline(always)]
    pub fn t1tme(&self) -> T1TME_R {
        T1TME_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MacTimer test mode enable"]
    #[inline(always)]
    pub fn mttme(&self) -> MTTME_R {
        MTTME_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer3 test mode enable"]
    #[inline(always)]
    pub fn t3tme(&self) -> T3TME_R {
        T3TME_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C 0 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0tme(&mut self) -> I2C0TME_W<0> {
        I2C0TME_W::new(self)
    }
    #[doc = "Bit 16 - Timer0 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn t0tme(&mut self) -> T0TME_W<16> {
        T0TME_W::new(self)
    }
    #[doc = "Bit 17 - Timer1 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn t1tme(&mut self) -> T1TME_W<17> {
        T1TME_W::new(self)
    }
    #[doc = "Bit 18 - MacTimer test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mttme(&mut self) -> MTTME_W<18> {
        MTTME_W::new(self)
    }
    #[doc = "Bit 19 - Timer3 test mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn t3tme(&mut self) -> T3TME_W<19> {
        T3TME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral test mode enable 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptme2](index.html) module"]
pub struct PTME2_SPEC;
impl crate::RegisterSpec for PTME2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptme2::R](R) reader structure"]
impl crate::Readable for PTME2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ptme2::W](W) writer structure"]
impl crate::Writable for PTME2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTME2 to value 0"]
impl crate::Resettable for PTME2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
