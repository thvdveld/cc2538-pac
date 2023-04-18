#[doc = "Register `GPT` reader"]
pub struct R(crate::R<GPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPT` writer"]
pub struct W(crate::W<GPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPT_SPEC>;
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
impl From<crate::W<GPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPTIDOV` reader - GPTimer increment/decrement override value"]
pub type GPTIDOV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPTIDOV` writer - GPTimer increment/decrement override value"]
pub type GPTIDOV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPT_SPEC, u8, u8, 5, O>;
#[doc = "Field `GPTIDOE` reader - GPTimer increment/decrement override enable"]
pub type GPTIDOE_R = crate::BitReader<bool>;
#[doc = "Field `GPTIDOE` writer - GPTimer increment/decrement override enable"]
pub type GPTIDOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - GPTimer increment/decrement override value"]
    #[inline(always)]
    pub fn gptidov(&self) -> GPTIDOV_R {
        GPTIDOV_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - GPTimer increment/decrement override enable"]
    #[inline(always)]
    pub fn gptidoe(&self) -> GPTIDOE_R {
        GPTIDOE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - GPTimer increment/decrement override value"]
    #[inline(always)]
    #[must_use]
    pub fn gptidov(&mut self) -> GPTIDOV_W<0> {
        GPTIDOV_W::new(self)
    }
    #[doc = "Bit 8 - GPTimer increment/decrement override enable"]
    #[inline(always)]
    #[must_use]
    pub fn gptidoe(&mut self) -> GPTIDOE_W<8> {
        GPTIDOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPTIMER override values\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpt](index.html) module"]
pub struct GPT_SPEC;
impl crate::RegisterSpec for GPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpt::R](R) reader structure"]
impl crate::Readable for GPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpt::W](W) writer structure"]
impl crate::Writable for GPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPT to value 0"]
impl crate::Resettable for GPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
