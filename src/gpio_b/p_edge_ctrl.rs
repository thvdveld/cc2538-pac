#[doc = "Register `P_EDGE_CTRL` reader"]
pub struct R(crate::R<P_EDGE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P_EDGE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P_EDGE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P_EDGE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P_EDGE_CTRL` writer"]
pub struct W(crate::W<P_EDGE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P_EDGE_CTRL_SPEC>;
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
impl From<crate::W<P_EDGE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P_EDGE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDIRC7` reader - Port D bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC7_R = crate::BitReader<bool>;
#[doc = "Field `PDIRC7` writer - Port D bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PDIRC6` reader - Port D bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC6_R = crate::BitReader<bool>;
#[doc = "Field `PDIRC6` writer - Port D bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PDIRC5` reader - Port D bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC5_R = crate::BitReader<bool>;
#[doc = "Field `PDIRC5` writer - Port D bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PDIRC4` reader - Port D bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC4_R = crate::BitReader<bool>;
#[doc = "Field `PDIRC4` writer - Port D bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PDIRC3` reader - Port D bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC3_R = crate::BitReader<bool>;
#[doc = "Field `PDIRC3` writer - Port D bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PDIRC2` reader - Port D bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC2_R = crate::BitReader<bool>;
#[doc = "Field `PDIRC2` writer - Port D bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PDIRC1` reader - Port D bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC1_R = crate::BitReader<bool>;
#[doc = "Field `PDIRC1` writer - Port D bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PDIRC0` reader - Port D bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC0_R = crate::BitReader<bool>;
#[doc = "Field `PDIRC0` writer - Port D bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PDIRC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PCIRC7` reader - Port C bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC7_R = crate::BitReader<bool>;
#[doc = "Field `PCIRC7` writer - Port C bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PCIRC6` reader - Port C bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC6_R = crate::BitReader<bool>;
#[doc = "Field `PCIRC6` writer - Port C bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PCIRC5` reader - Port C bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC5_R = crate::BitReader<bool>;
#[doc = "Field `PCIRC5` writer - Port C bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PCIRC4` reader - Port C bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC4_R = crate::BitReader<bool>;
#[doc = "Field `PCIRC4` writer - Port C bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PCIRC3` reader - Port C bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC3_R = crate::BitReader<bool>;
#[doc = "Field `PCIRC3` writer - Port C bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PCIRC2` reader - Port C bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC2_R = crate::BitReader<bool>;
#[doc = "Field `PCIRC2` writer - Port C bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PCIRC1` reader - Port C bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC1_R = crate::BitReader<bool>;
#[doc = "Field `PCIRC1` writer - Port C bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PCIRC0` reader - Port C bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC0_R = crate::BitReader<bool>;
#[doc = "Field `PCIRC0` writer - Port C bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PCIRC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PBIRC7` reader - Port B bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC7_R = crate::BitReader<bool>;
#[doc = "Field `PBIRC7` writer - Port B bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PBIRC6` reader - Port B bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC6_R = crate::BitReader<bool>;
#[doc = "Field `PBIRC6` writer - Port B bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PBIRC5` reader - Port B bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC5_R = crate::BitReader<bool>;
#[doc = "Field `PBIRC5` writer - Port B bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PBIRC4` reader - Port B bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC4_R = crate::BitReader<bool>;
#[doc = "Field `PBIRC4` writer - Port B bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PBIRC3` reader - Port B bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC3_R = crate::BitReader<bool>;
#[doc = "Field `PBIRC3` writer - Port B bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PBIRC2` reader - Port B bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC2_R = crate::BitReader<bool>;
#[doc = "Field `PBIRC2` writer - Port B bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PBIRC1` reader - Port B bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC1_R = crate::BitReader<bool>;
#[doc = "Field `PBIRC1` writer - Port B bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PBIRC0` reader - Port B bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC0_R = crate::BitReader<bool>;
#[doc = "Field `PBIRC0` writer - Port B bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PBIRC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PAIRC7` reader - Port A bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC7_R = crate::BitReader<bool>;
#[doc = "Field `PAIRC7` writer - Port A bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC7_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PAIRC6` reader - Port A bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC6_R = crate::BitReader<bool>;
#[doc = "Field `PAIRC6` writer - Port A bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC6_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PAIRC5` reader - Port A bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC5_R = crate::BitReader<bool>;
#[doc = "Field `PAIRC5` writer - Port A bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC5_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PAIRC4` reader - Port A bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC4_R = crate::BitReader<bool>;
#[doc = "Field `PAIRC4` writer - Port A bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC4_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PAIRC3` reader - Port A bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC3_R = crate::BitReader<bool>;
#[doc = "Field `PAIRC3` writer - Port A bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC3_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PAIRC2` reader - Port A bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC2_R = crate::BitReader<bool>;
#[doc = "Field `PAIRC2` writer - Port A bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PAIRC1` reader - Port A bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC1_R = crate::BitReader<bool>;
#[doc = "Field `PAIRC1` writer - Port A bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
#[doc = "Field `PAIRC0` reader - Port A bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC0_R = crate::BitReader<bool>;
#[doc = "Field `PAIRC0` writer - Port A bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
pub type PAIRC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, P_EDGE_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 31 - Port D bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc7(&self) -> PDIRC7_R {
        PDIRC7_R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 30 - Port D bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc6(&self) -> PDIRC6_R {
        PDIRC6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 29 - Port D bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc5(&self) -> PDIRC5_R {
        PDIRC5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 28 - Port D bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc4(&self) -> PDIRC4_R {
        PDIRC4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 27 - Port D bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc3(&self) -> PDIRC3_R {
        PDIRC3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 26 - Port D bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc2(&self) -> PDIRC2_R {
        PDIRC2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 25 - Port D bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc1(&self) -> PDIRC1_R {
        PDIRC1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 24 - Port D bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc0(&self) -> PDIRC0_R {
        PDIRC0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 23 - Port C bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc7(&self) -> PCIRC7_R {
        PCIRC7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 22 - Port C bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc6(&self) -> PCIRC6_R {
        PCIRC6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 21 - Port C bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc5(&self) -> PCIRC5_R {
        PCIRC5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 20 - Port C bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc4(&self) -> PCIRC4_R {
        PCIRC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 19 - Port C bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc3(&self) -> PCIRC3_R {
        PCIRC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 18 - Port C bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc2(&self) -> PCIRC2_R {
        PCIRC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - Port C bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc1(&self) -> PCIRC1_R {
        PCIRC1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - Port C bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc0(&self) -> PCIRC0_R {
        PCIRC0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 15 - Port B bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc7(&self) -> PBIRC7_R {
        PBIRC7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 14 - Port B bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc6(&self) -> PBIRC6_R {
        PBIRC6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 13 - Port B bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc5(&self) -> PBIRC5_R {
        PBIRC5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 12 - Port B bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc4(&self) -> PBIRC4_R {
        PBIRC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 11 - Port B bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc3(&self) -> PBIRC3_R {
        PBIRC3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 10 - Port B bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc2(&self) -> PBIRC2_R {
        PBIRC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 9 - Port B bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc1(&self) -> PBIRC1_R {
        PBIRC1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 8 - Port B bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc0(&self) -> PBIRC0_R {
        PBIRC0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 7 - Port A bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc7(&self) -> PAIRC7_R {
        PAIRC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 6 - Port A bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc6(&self) -> PAIRC6_R {
        PAIRC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 5 - Port A bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc5(&self) -> PAIRC5_R {
        PAIRC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 4 - Port A bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc4(&self) -> PAIRC4_R {
        PAIRC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 3 - Port A bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc3(&self) -> PAIRC3_R {
        PAIRC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 2 - Port A bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc2(&self) -> PAIRC2_R {
        PAIRC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 1 - Port A bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc1(&self) -> PAIRC1_R {
        PAIRC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 0 - Port A bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc0(&self) -> PAIRC0_R {
        PAIRC0_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Port D bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc7(&mut self) -> PDIRC7_W<31> {
        PDIRC7_W::new(self)
    }
    #[doc = "Bit 30 - Port D bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc6(&mut self) -> PDIRC6_W<30> {
        PDIRC6_W::new(self)
    }
    #[doc = "Bit 29 - Port D bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc5(&mut self) -> PDIRC5_W<29> {
        PDIRC5_W::new(self)
    }
    #[doc = "Bit 28 - Port D bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc4(&mut self) -> PDIRC4_W<28> {
        PDIRC4_W::new(self)
    }
    #[doc = "Bit 27 - Port D bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc3(&mut self) -> PDIRC3_W<27> {
        PDIRC3_W::new(self)
    }
    #[doc = "Bit 26 - Port D bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc2(&mut self) -> PDIRC2_W<26> {
        PDIRC2_W::new(self)
    }
    #[doc = "Bit 25 - Port D bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc1(&mut self) -> PDIRC1_W<25> {
        PDIRC1_W::new(self)
    }
    #[doc = "Bit 24 - Port D bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pdirc0(&mut self) -> PDIRC0_W<24> {
        PDIRC0_W::new(self)
    }
    #[doc = "Bit 23 - Port C bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc7(&mut self) -> PCIRC7_W<23> {
        PCIRC7_W::new(self)
    }
    #[doc = "Bit 22 - Port C bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc6(&mut self) -> PCIRC6_W<22> {
        PCIRC6_W::new(self)
    }
    #[doc = "Bit 21 - Port C bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc5(&mut self) -> PCIRC5_W<21> {
        PCIRC5_W::new(self)
    }
    #[doc = "Bit 20 - Port C bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc4(&mut self) -> PCIRC4_W<20> {
        PCIRC4_W::new(self)
    }
    #[doc = "Bit 19 - Port C bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc3(&mut self) -> PCIRC3_W<19> {
        PCIRC3_W::new(self)
    }
    #[doc = "Bit 18 - Port C bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc2(&mut self) -> PCIRC2_W<18> {
        PCIRC2_W::new(self)
    }
    #[doc = "Bit 17 - Port C bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc1(&mut self) -> PCIRC1_W<17> {
        PCIRC1_W::new(self)
    }
    #[doc = "Bit 16 - Port C bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pcirc0(&mut self) -> PCIRC0_W<16> {
        PCIRC0_W::new(self)
    }
    #[doc = "Bit 15 - Port B bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc7(&mut self) -> PBIRC7_W<15> {
        PBIRC7_W::new(self)
    }
    #[doc = "Bit 14 - Port B bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc6(&mut self) -> PBIRC6_W<14> {
        PBIRC6_W::new(self)
    }
    #[doc = "Bit 13 - Port B bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc5(&mut self) -> PBIRC5_W<13> {
        PBIRC5_W::new(self)
    }
    #[doc = "Bit 12 - Port B bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc4(&mut self) -> PBIRC4_W<12> {
        PBIRC4_W::new(self)
    }
    #[doc = "Bit 11 - Port B bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc3(&mut self) -> PBIRC3_W<11> {
        PBIRC3_W::new(self)
    }
    #[doc = "Bit 10 - Port B bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc2(&mut self) -> PBIRC2_W<10> {
        PBIRC2_W::new(self)
    }
    #[doc = "Bit 9 - Port B bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc1(&mut self) -> PBIRC1_W<9> {
        PBIRC1_W::new(self)
    }
    #[doc = "Bit 8 - Port B bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pbirc0(&mut self) -> PBIRC0_W<8> {
        PBIRC0_W::new(self)
    }
    #[doc = "Bit 7 - Port A bit 7 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc7(&mut self) -> PAIRC7_W<7> {
        PAIRC7_W::new(self)
    }
    #[doc = "Bit 6 - Port A bit 6 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc6(&mut self) -> PAIRC6_W<6> {
        PAIRC6_W::new(self)
    }
    #[doc = "Bit 5 - Port A bit 5 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc5(&mut self) -> PAIRC5_W<5> {
        PAIRC5_W::new(self)
    }
    #[doc = "Bit 4 - Port A bit 4 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc4(&mut self) -> PAIRC4_W<4> {
        PAIRC4_W::new(self)
    }
    #[doc = "Bit 3 - Port A bit 3 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc3(&mut self) -> PAIRC3_W<3> {
        PAIRC3_W::new(self)
    }
    #[doc = "Bit 2 - Port A bit 2 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc2(&mut self) -> PAIRC2_W<2> {
        PAIRC2_W::new(self)
    }
    #[doc = "Bit 1 - Port A bit 1 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc1(&mut self) -> PAIRC1_W<1> {
        PAIRC1_W::new(self)
    }
    #[doc = "Bit 0 - Port A bit 0 interrupt request condition: 0: Rising 1: Falling edge"]
    #[inline(always)]
    pub fn pairc0(&mut self) -> PAIRC0_W<0> {
        PAIRC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The port edge control register is used to control which edge of each port input causes that port to generate a power-up interrupt to the system.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p_edge_ctrl](index.html) module"]
pub struct P_EDGE_CTRL_SPEC;
impl crate::RegisterSpec for P_EDGE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [p_edge_ctrl::R](R) reader structure"]
impl crate::Readable for P_EDGE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p_edge_ctrl::W](W) writer structure"]
impl crate::Writable for P_EDGE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P_EDGE_CTRL to value 0"]
impl crate::Resettable for P_EDGE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
