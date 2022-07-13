#[doc = "Register `FWDATA` reader"]
pub struct R(crate::R<FWDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FWDATA` writer"]
pub struct W(crate::W<FWDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FWDATA_SPEC>;
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
impl From<crate::W<FWDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FWDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FWDATA` reader - 32-bit flash write data Writes to this register are accepted only during a flash write sequence; that is, writes to this register after having written 1 to the FCTL.WRITE bit. New 32-bit data is written only if FCTL.FULL = 0."]
pub type FWDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FWDATA` writer - 32-bit flash write data Writes to this register are accepted only during a flash write sequence; that is, writes to this register after having written 1 to the FCTL.WRITE bit. New 32-bit data is written only if FCTL.FULL = 0."]
pub type FWDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FWDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 32-bit flash write data Writes to this register are accepted only during a flash write sequence; that is, writes to this register after having written 1 to the FCTL.WRITE bit. New 32-bit data is written only if FCTL.FULL = 0."]
    #[inline(always)]
    pub fn fwdata(&self) -> FWDATA_R {
        FWDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit flash write data Writes to this register are accepted only during a flash write sequence; that is, writes to this register after having written 1 to the FCTL.WRITE bit. New 32-bit data is written only if FCTL.FULL = 0."]
    #[inline(always)]
    pub fn fwdata(&mut self) -> FWDATA_W<0> {
        FWDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash data This register contains the 32-bits of data to be written to the flash location selected in FADDR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwdata](index.html) module"]
pub struct FWDATA_SPEC;
impl crate::RegisterSpec for FWDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwdata::R](R) reader structure"]
impl crate::Readable for FWDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fwdata::W](W) writer structure"]
impl crate::Writable for FWDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FWDATA to value 0"]
impl crate::Resettable for FWDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
