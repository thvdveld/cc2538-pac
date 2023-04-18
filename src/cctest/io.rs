#[doc = "Register `IO` reader"]
pub struct R(crate::R<IO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO` writer"]
pub struct W(crate::W<IO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO_SPEC>;
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
impl From<crate::W<IO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SC` reader - I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
pub type SC_R = crate::BitReader<bool>;
#[doc = "Field `SC` writer - I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
pub type SC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IO_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
    #[inline(always)]
    pub fn sc(&self) -> SC_R {
        SC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
    #[inline(always)]
    #[must_use]
    pub fn sc(&mut self) -> SC_W<0> {
        SC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output strength control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io](index.html) module"]
pub struct IO_SPEC;
impl crate::RegisterSpec for IO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io::R](R) reader structure"]
impl crate::Readable for IO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io::W](W) writer structure"]
impl crate::Writable for IO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IO to value 0"]
impl crate::Resettable for IO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
