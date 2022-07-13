#[doc = "Register `RXMASKCLR` reader"]
pub struct R(crate::R<RXMASKCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMASKCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMASKCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMASKCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXMASKCLR` writer"]
pub struct W(crate::W<RXMASKCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXMASKCLR_SPEC>;
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
impl From<crate::W<RXMASKCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXMASKCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXENMASKCLR` reader - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
pub type RXENMASKCLR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXENMASKCLR` writer - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
pub type RXENMASKCLR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXMASKCLR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
    #[inline(always)]
    pub fn rxenmaskclr(&self) -> RXENMASKCLR_R {
        RXENMASKCLR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - When written, the written data is inverted and ANDed with the RXENMASK and stored in RXENMASK. For example, if 1 is written to one or more bit positions in this register, the corresponding bits are cleared in RXENMASK."]
    #[inline(always)]
    pub fn rxenmaskclr(&mut self) -> RXENMASKCLR_W<0> {
        RXENMASKCLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX disabling\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmaskclr](index.html) module"]
pub struct RXMASKCLR_SPEC;
impl crate::RegisterSpec for RXMASKCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmaskclr::R](R) reader structure"]
impl crate::Readable for RXMASKCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxmaskclr::W](W) writer structure"]
impl crate::Writable for RXMASKCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXMASKCLR to value 0"]
impl crate::Resettable for RXMASKCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
