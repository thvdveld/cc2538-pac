#[doc = "Register `CHASGN` reader"]
pub type R = crate::R<CHASGN_SPEC>;
#[doc = "Register `CHASGN` writer"]
pub type W = crate::W<CHASGN_SPEC>;
#[doc = "Field `CHASGN` reader - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
pub type CHASGN_R = crate::FieldReader<u32>;
#[doc = "Field `CHASGN` writer - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
pub type CHASGN_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
    #[inline(always)]
    pub fn chasgn(&self) -> CHASGN_R {
        CHASGN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel \\[n\\]
assignment select 0: Use the primary channel assignment 1: Use the secondary channel assignment"]
    #[inline(always)]
    #[must_use]
    pub fn chasgn(&mut self) -> CHASGN_W<CHASGN_SPEC, 0> {
        CHASGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "DMA channel assignment Each bit of the CHASGN register represents the corresponding uDMA channel. Setting a bit selects the secondary channel assignment as specified in the section \"Channel Assignments\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chasgn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chasgn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHASGN_SPEC;
impl crate::RegisterSpec for CHASGN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chasgn::R`](R) reader structure"]
impl crate::Readable for CHASGN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chasgn::W`](W) writer structure"]
impl crate::Writable for CHASGN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHASGN to value 0"]
impl crate::Resettable for CHASGN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
