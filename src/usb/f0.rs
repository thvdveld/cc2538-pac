#[doc = "Register `F0` reader"]
pub type R = crate::R<F0_SPEC>;
#[doc = "Register `F0` writer"]
pub type W = crate::W<F0_SPEC>;
#[doc = "Field `USBF0` reader - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
pub type USBF0_R = crate::FieldReader;
#[doc = "Field `USBF0` writer - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
pub type USBF0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
    #[inline(always)]
    pub fn usbf0(&self) -> USBF0_R {
        USBF0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint 0 FIFO Reading this register unloads one byte from the endpoint 0 FIFO. Writing to this register loads one byte into the endpoint 0 FIFO. The FIFO memory for EP0 is used for incoming and outgoing data packets."]
    #[inline(always)]
    #[must_use]
    pub fn usbf0(&mut self) -> USBF0_W<F0_SPEC> {
        USBF0_W::new(self, 0)
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
#[doc = "Endpoint 0 FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct F0_SPEC;
impl crate::RegisterSpec for F0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`f0::R`](R) reader structure"]
impl crate::Readable for F0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`f0::W`](W) writer structure"]
impl crate::Writable for F0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets F0 to value 0"]
impl crate::Resettable for F0_SPEC {
    const RESET_VALUE: u32 = 0;
}
