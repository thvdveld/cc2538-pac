#[doc = "Register `LCTL` reader"]
pub struct R(crate::R<LCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCTL` writer"]
pub struct W(crate::W<LCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCTL_SPEC>;
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
impl From<crate::W<LCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASTER` reader - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
pub type MASTER_R = crate::BitReader<bool>;
#[doc = "Field `MASTER` writer - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
pub type MASTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCTL_SPEC, bool, O>;
#[doc = "Field `BLEN` reader - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
pub type BLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BLEN` writer - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
pub type BLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LCTL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
    #[inline(always)]
    pub fn blen(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LIN master enable 1: The UART operates as a LIN master. 0: The UART operates as a LIN slave."]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<0> {
        MASTER_W::new(self)
    }
    #[doc = "Bits 4:5 - Sync break length 0x3: Sync break length is 16T bits 0x2: Sync break length is 15T bits 0x1: Sync break length is 14T bits 0x0: Sync break length is 13T bits (default)"]
    #[inline(always)]
    #[must_use]
    pub fn blen(&mut self) -> BLEN_W<4> {
        BLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART LIN control The LCTL register is the configures the operation of the UART when in LIN mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lctl](index.html) module"]
pub struct LCTL_SPEC;
impl crate::RegisterSpec for LCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lctl::R](R) reader structure"]
impl crate::Readable for LCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lctl::W](W) writer structure"]
impl crate::Writable for LCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCTL to value 0"]
impl crate::Resettable for LCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
