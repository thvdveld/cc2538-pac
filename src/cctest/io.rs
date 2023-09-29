#[doc = "Register `IO` reader"]
pub type R = crate::R<IO_SPEC>;
#[doc = "Register `IO` writer"]
pub type W = crate::W<IO_SPEC>;
#[doc = "Field `SC` reader - I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
pub type SC_R = crate::BitReader;
#[doc = "Field `SC` writer - I/O strength control bit Common to all digital output pads Should be set when unregulated voltage is below approximately 2.6 V."]
pub type SC_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn sc(&mut self) -> SC_W<IO_SPEC, 0> {
        SC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Output strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`io::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IO_SPEC;
impl crate::RegisterSpec for IO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io::R`](R) reader structure"]
impl crate::Readable for IO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`io::W`](W) writer structure"]
impl crate::Writable for IO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IO to value 0"]
impl crate::Resettable for IO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
