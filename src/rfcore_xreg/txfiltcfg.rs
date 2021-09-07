#[doc = "Register `TXFILTCFG` reader"]
pub struct R(crate::R<TXFILTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFILTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFILTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFILTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXFILTCFG` writer"]
pub struct W(crate::W<TXFILTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFILTCFG_SPEC>;
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
impl From<crate::W<TXFILTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFILTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FC` reader - Drives signal rfr_txfilt_fc"]
pub struct FC_R(crate::FieldReader<u8, u8>);
impl FC_R {
    pub(crate) fn new(bits: u8) -> Self {
        FC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC` writer - Drives signal rfr_txfilt_fc"]
pub struct FC_W<'a> {
    w: &'a mut W,
}
impl<'a> FC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Drives signal rfr_txfilt_fc"]
    #[inline(always)]
    pub fn fc(&self) -> FC_R {
        FC_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Drives signal rfr_txfilt_fc"]
    #[inline(always)]
    pub fn fc(&mut self) -> FC_W {
        FC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX filter configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfiltcfg](index.html) module"]
pub struct TXFILTCFG_SPEC;
impl crate::RegisterSpec for TXFILTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfiltcfg::R](R) reader structure"]
impl crate::Readable for TXFILTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txfiltcfg::W](W) writer structure"]
impl crate::Writable for TXFILTCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXFILTCFG to value 0"]
impl crate::Resettable for TXFILTCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
