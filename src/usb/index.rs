#[doc = "Register `INDEX` reader"]
pub type R = crate::R<INDEX_SPEC>;
#[doc = "Register `INDEX` writer"]
pub type W = crate::W<INDEX_SPEC>;
#[doc = "Field `USBINDEX` reader - Index of the currently selected endpoint The index is set to 0 to enable access to endpoint 0 control and status registers The index is set to 1, 2, 3, 4 or 5 to enable access to IN/OUT endpoint 1, 2, 3, 4 or 5 control and status registers, respectively"]
pub type USBINDEX_R = crate::FieldReader;
#[doc = "Field `USBINDEX` writer - Index of the currently selected endpoint The index is set to 0 to enable access to endpoint 0 control and status registers The index is set to 1, 2, 3, 4 or 5 to enable access to IN/OUT endpoint 1, 2, 3, 4 or 5 control and status registers, respectively"]
pub type USBINDEX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Index of the currently selected endpoint The index is set to 0 to enable access to endpoint 0 control and status registers The index is set to 1, 2, 3, 4 or 5 to enable access to IN/OUT endpoint 1, 2, 3, 4 or 5 control and status registers, respectively"]
    #[inline(always)]
    pub fn usbindex(&self) -> USBINDEX_R {
        USBINDEX_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Index of the currently selected endpoint The index is set to 0 to enable access to endpoint 0 control and status registers The index is set to 1, 2, 3, 4 or 5 to enable access to IN/OUT endpoint 1, 2, 3, 4 or 5 control and status registers, respectively"]
    #[inline(always)]
    #[must_use]
    pub fn usbindex(&mut self) -> USBINDEX_W<INDEX_SPEC> {
        USBINDEX_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Index register for selecting the endpoint status and control registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`index::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`index::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INDEX_SPEC;
impl crate::RegisterSpec for INDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`index::R`](R) reader structure"]
impl crate::Readable for INDEX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`index::W`](W) writer structure"]
impl crate::Writable for INDEX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INDEX to value 0"]
impl crate::Resettable for INDEX_SPEC {
    const RESET_VALUE: u32 = 0;
}
