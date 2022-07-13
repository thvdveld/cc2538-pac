#[doc = "Register `SHORT_ADDR0` reader"]
pub struct R(crate::R<SHORT_ADDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORT_ADDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORT_ADDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORT_ADDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORT_ADDR0` writer"]
pub struct W(crate::W<SHORT_ADDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORT_ADDR0_SPEC>;
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
impl From<crate::W<SHORT_ADDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORT_ADDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHORT_ADDR0` reader - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
pub type SHORT_ADDR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SHORT_ADDR0` writer - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
pub type SHORT_ADDR0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHORT_ADDR0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
    #[inline(always)]
    pub fn short_addr0(&self) -> SHORT_ADDR0_R {
        SHORT_ADDR0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SHORT_ADDR\\[7:0\\]
The short address used during destination address filtering"]
    #[inline(always)]
    pub fn short_addr0(&mut self) -> SHORT_ADDR0_W<0> {
        SHORT_ADDR0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Local address information This register is stored in RAM; the reset value is undefined.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [short_addr0](index.html) module"]
pub struct SHORT_ADDR0_SPEC;
impl crate::RegisterSpec for SHORT_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [short_addr0::R](R) reader structure"]
impl crate::Readable for SHORT_ADDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [short_addr0::W](W) writer structure"]
impl crate::Writable for SHORT_ADDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHORT_ADDR0 to value 0"]
impl crate::Resettable for SHORT_ADDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
