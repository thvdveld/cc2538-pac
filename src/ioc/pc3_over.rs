#[doc = "Register `PC3_OVER` reader"]
pub struct R(crate::R<PC3_OVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC3_OVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC3_OVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC3_OVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC3_OVER` writer"]
pub struct W(crate::W<PC3_OVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC3_OVER_SPEC>;
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
impl From<crate::W<PC3_OVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC3_OVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PC3_over` reader - 0: output disable 1: oe - output enable"]
pub type PC3_OVER_R = crate::BitReader<bool>;
#[doc = "Field `PC3_over` writer - 0: output disable 1: oe - output enable"]
pub type PC3_OVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PC3_OVER_SPEC, bool, O>;
impl R {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc3_over(&self) -> PC3_OVER_R {
        PC3_OVER_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: output disable 1: oe - output enable"]
    #[inline(always)]
    pub fn pc3_over(&mut self) -> PC3_OVER_W<3> {
        PC3_OVER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This is the overide configuration register for each pad. PC3 has high drive capability.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc3_over](index.html) module"]
pub struct PC3_OVER_SPEC;
impl crate::RegisterSpec for PC3_OVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc3_over::R](R) reader structure"]
impl crate::Readable for PC3_OVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc3_over::W](W) writer structure"]
impl crate::Writable for PC3_OVER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC3_OVER to value 0x04"]
impl crate::Resettable for PC3_OVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
