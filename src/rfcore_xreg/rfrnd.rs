#[doc = "Register `RFRND` reader"]
pub type R = crate::R<RfrndSpec>;
#[doc = "Field `IRND` reader - Random bit from the I channel of the receiver"]
pub type IrndR = crate::BitReader;
#[doc = "Field `QRND` reader - Random bit from the Q channel of the receiver"]
pub type QrndR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Random bit from the I channel of the receiver"]
    #[inline(always)]
    pub fn irnd(&self) -> IrndR {
        IrndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Random bit from the Q channel of the receiver"]
    #[inline(always)]
    pub fn qrnd(&self) -> QrndR {
        QrndR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Random data\n\nYou can [`read`](crate::Reg::read) this register and get [`rfrnd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfrndSpec;
impl crate::RegisterSpec for RfrndSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfrnd::R`](R) reader structure"]
impl crate::Readable for RfrndSpec {}
#[doc = "`reset()` method sets RFRND to value 0"]
impl crate::Resettable for RfrndSpec {
    const RESET_VALUE: u32 = 0;
}
