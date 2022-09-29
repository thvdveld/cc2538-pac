#[doc = "Register `PI_IEN` reader"]
pub struct R(crate::R<PI_IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PI_IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PI_IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PI_IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PI_IEN` writer"]
pub struct W(crate::W<PI_IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PI_IEN_SPEC>;
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
impl From<crate::W<PI_IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PI_IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAIEN0` reader - Port A bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN0_R = crate::BitReader<bool>;
#[doc = "Field `PAIEN0` writer - Port A bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PAIEN1` reader - Port A bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN1_R = crate::BitReader<bool>;
#[doc = "Field `PAIEN1` writer - Port A bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PAIEN2` reader - Port A bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN2_R = crate::BitReader<bool>;
#[doc = "Field `PAIEN2` writer - Port A bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PAIEN3` reader - Port A bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN3_R = crate::BitReader<bool>;
#[doc = "Field `PAIEN3` writer - Port A bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PAIEN4` reader - Port A bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN4_R = crate::BitReader<bool>;
#[doc = "Field `PAIEN4` writer - Port A bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PAIEN5` reader - Port A bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN5_R = crate::BitReader<bool>;
#[doc = "Field `PAIEN5` writer - Port A bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PAIEN6` reader - Port A bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN6_R = crate::BitReader<bool>;
#[doc = "Field `PAIEN6` writer - Port A bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PAIEN7` reader - Port A bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN7_R = crate::BitReader<bool>;
#[doc = "Field `PAIEN7` writer - Port A bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub type PAIEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PBIEN0` reader - Port B bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN0_R = crate::BitReader<bool>;
#[doc = "Field `PBIEN0` writer - Port B bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PBIEN1` reader - Port B bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN1_R = crate::BitReader<bool>;
#[doc = "Field `PBIEN1` writer - Port B bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PBIEN2` reader - Port B bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN2_R = crate::BitReader<bool>;
#[doc = "Field `PBIEN2` writer - Port B bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PBIEN3` reader - Port B bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN3_R = crate::BitReader<bool>;
#[doc = "Field `PBIEN3` writer - Port B bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PBIEN4` reader - Port B bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN4_R = crate::BitReader<bool>;
#[doc = "Field `PBIEN4` writer - Port B bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PBIEN5` reader - Port B bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN5_R = crate::BitReader<bool>;
#[doc = "Field `PBIEN5` writer - Port B bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PBIEN6` reader - Port B bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN6_R = crate::BitReader<bool>;
#[doc = "Field `PBIEN6` writer - Port B bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PBIEN7` reader - Port B bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN7_R = crate::BitReader<bool>;
#[doc = "Field `PBIEN7` writer - Port B bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub type PBIEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PCIEN0` reader - Port C bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN0_R = crate::BitReader<bool>;
#[doc = "Field `PCIEN0` writer - Port C bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PCIEN1` reader - Port C bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN1_R = crate::BitReader<bool>;
#[doc = "Field `PCIEN1` writer - Port C bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PCIEN2` reader - Port C bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN2_R = crate::BitReader<bool>;
#[doc = "Field `PCIEN2` writer - Port C bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PCIEN3` reader - Port C bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN3_R = crate::BitReader<bool>;
#[doc = "Field `PCIEN3` writer - Port C bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PCIEN4` reader - Port C bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN4_R = crate::BitReader<bool>;
#[doc = "Field `PCIEN4` writer - Port C bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PCIEN5` reader - Port C bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN5_R = crate::BitReader<bool>;
#[doc = "Field `PCIEN5` writer - Port C bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PCIEN6` reader - Port C bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN6_R = crate::BitReader<bool>;
#[doc = "Field `PCIEN6` writer - Port C bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PCIEN7` reader - Port C bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN7_R = crate::BitReader<bool>;
#[doc = "Field `PCIEN7` writer - Port C bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub type PCIEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PDIEN0` reader - Port D bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN0_R = crate::BitReader<bool>;
#[doc = "Field `PDIEN0` writer - Port D bit 0 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PDIEN1` reader - Port D bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN1_R = crate::BitReader<bool>;
#[doc = "Field `PDIEN1` writer - Port D bit 1 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PDIEN2` reader - Port D bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN2_R = crate::BitReader<bool>;
#[doc = "Field `PDIEN2` writer - Port D bit 2 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PDIEN3` reader - Port D bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN3_R = crate::BitReader<bool>;
#[doc = "Field `PDIEN3` writer - Port D bit 3 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PDIEN4` reader - Port D bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN4_R = crate::BitReader<bool>;
#[doc = "Field `PDIEN4` writer - Port D bit 4 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PDIEN5` reader - Port D bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN5_R = crate::BitReader<bool>;
#[doc = "Field `PDIEN5` writer - Port D bit 5 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PDIEN6` reader - Port D bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN6_R = crate::BitReader<bool>;
#[doc = "Field `PDIEN6` writer - Port D bit 6 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
#[doc = "Field `PDIEN7` reader - Port D bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN7_R = crate::BitReader<bool>;
#[doc = "Field `PDIEN7` writer - Port D bit 7 interrupt enable: 1: Enabled 2: Disabled"]
pub type PDIEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PI_IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Port A bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien0(&self) -> PAIEN0_R {
        PAIEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port A bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien1(&self) -> PAIEN1_R {
        PAIEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port A bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien2(&self) -> PAIEN2_R {
        PAIEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port A bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien3(&self) -> PAIEN3_R {
        PAIEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port A bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien4(&self) -> PAIEN4_R {
        PAIEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port A bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien5(&self) -> PAIEN5_R {
        PAIEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port A bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien6(&self) -> PAIEN6_R {
        PAIEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port A bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien7(&self) -> PAIEN7_R {
        PAIEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port B bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien0(&self) -> PBIEN0_R {
        PBIEN0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port B bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien1(&self) -> PBIEN1_R {
        PBIEN1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port B bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien2(&self) -> PBIEN2_R {
        PBIEN2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port B bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien3(&self) -> PBIEN3_R {
        PBIEN3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port B bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien4(&self) -> PBIEN4_R {
        PBIEN4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port B bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien5(&self) -> PBIEN5_R {
        PBIEN5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port B bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien6(&self) -> PBIEN6_R {
        PBIEN6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port B bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien7(&self) -> PBIEN7_R {
        PBIEN7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port C bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien0(&self) -> PCIEN0_R {
        PCIEN0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port C bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien1(&self) -> PCIEN1_R {
        PCIEN1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port C bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien2(&self) -> PCIEN2_R {
        PCIEN2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port C bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien3(&self) -> PCIEN3_R {
        PCIEN3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port C bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien4(&self) -> PCIEN4_R {
        PCIEN4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port C bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien5(&self) -> PCIEN5_R {
        PCIEN5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port C bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien6(&self) -> PCIEN6_R {
        PCIEN6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port C bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien7(&self) -> PCIEN7_R {
        PCIEN7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port D bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien0(&self) -> PDIEN0_R {
        PDIEN0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port D bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien1(&self) -> PDIEN1_R {
        PDIEN1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port D bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien2(&self) -> PDIEN2_R {
        PDIEN2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port D bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien3(&self) -> PDIEN3_R {
        PDIEN3_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port D bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien4(&self) -> PDIEN4_R {
        PDIEN4_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port D bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien5(&self) -> PDIEN5_R {
        PDIEN5_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port D bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien6(&self) -> PDIEN6_R {
        PDIEN6_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port D bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien7(&self) -> PDIEN7_R {
        PDIEN7_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien0(&mut self) -> PAIEN0_W<0> {
        PAIEN0_W::new(self)
    }
    #[doc = "Bit 1 - Port A bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien1(&mut self) -> PAIEN1_W<1> {
        PAIEN1_W::new(self)
    }
    #[doc = "Bit 2 - Port A bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien2(&mut self) -> PAIEN2_W<2> {
        PAIEN2_W::new(self)
    }
    #[doc = "Bit 3 - Port A bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien3(&mut self) -> PAIEN3_W<3> {
        PAIEN3_W::new(self)
    }
    #[doc = "Bit 4 - Port A bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien4(&mut self) -> PAIEN4_W<4> {
        PAIEN4_W::new(self)
    }
    #[doc = "Bit 5 - Port A bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien5(&mut self) -> PAIEN5_W<5> {
        PAIEN5_W::new(self)
    }
    #[doc = "Bit 6 - Port A bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien6(&mut self) -> PAIEN6_W<6> {
        PAIEN6_W::new(self)
    }
    #[doc = "Bit 7 - Port A bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn paien7(&mut self) -> PAIEN7_W<7> {
        PAIEN7_W::new(self)
    }
    #[doc = "Bit 8 - Port B bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien0(&mut self) -> PBIEN0_W<8> {
        PBIEN0_W::new(self)
    }
    #[doc = "Bit 9 - Port B bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien1(&mut self) -> PBIEN1_W<9> {
        PBIEN1_W::new(self)
    }
    #[doc = "Bit 10 - Port B bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien2(&mut self) -> PBIEN2_W<10> {
        PBIEN2_W::new(self)
    }
    #[doc = "Bit 11 - Port B bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien3(&mut self) -> PBIEN3_W<11> {
        PBIEN3_W::new(self)
    }
    #[doc = "Bit 12 - Port B bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien4(&mut self) -> PBIEN4_W<12> {
        PBIEN4_W::new(self)
    }
    #[doc = "Bit 13 - Port B bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien5(&mut self) -> PBIEN5_W<13> {
        PBIEN5_W::new(self)
    }
    #[doc = "Bit 14 - Port B bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien6(&mut self) -> PBIEN6_W<14> {
        PBIEN6_W::new(self)
    }
    #[doc = "Bit 15 - Port B bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pbien7(&mut self) -> PBIEN7_W<15> {
        PBIEN7_W::new(self)
    }
    #[doc = "Bit 16 - Port C bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien0(&mut self) -> PCIEN0_W<16> {
        PCIEN0_W::new(self)
    }
    #[doc = "Bit 17 - Port C bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien1(&mut self) -> PCIEN1_W<17> {
        PCIEN1_W::new(self)
    }
    #[doc = "Bit 18 - Port C bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien2(&mut self) -> PCIEN2_W<18> {
        PCIEN2_W::new(self)
    }
    #[doc = "Bit 19 - Port C bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien3(&mut self) -> PCIEN3_W<19> {
        PCIEN3_W::new(self)
    }
    #[doc = "Bit 20 - Port C bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien4(&mut self) -> PCIEN4_W<20> {
        PCIEN4_W::new(self)
    }
    #[doc = "Bit 21 - Port C bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien5(&mut self) -> PCIEN5_W<21> {
        PCIEN5_W::new(self)
    }
    #[doc = "Bit 22 - Port C bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien6(&mut self) -> PCIEN6_W<22> {
        PCIEN6_W::new(self)
    }
    #[doc = "Bit 23 - Port C bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pcien7(&mut self) -> PCIEN7_W<23> {
        PCIEN7_W::new(self)
    }
    #[doc = "Bit 24 - Port D bit 0 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien0(&mut self) -> PDIEN0_W<24> {
        PDIEN0_W::new(self)
    }
    #[doc = "Bit 25 - Port D bit 1 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien1(&mut self) -> PDIEN1_W<25> {
        PDIEN1_W::new(self)
    }
    #[doc = "Bit 26 - Port D bit 2 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien2(&mut self) -> PDIEN2_W<26> {
        PDIEN2_W::new(self)
    }
    #[doc = "Bit 27 - Port D bit 3 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien3(&mut self) -> PDIEN3_W<27> {
        PDIEN3_W::new(self)
    }
    #[doc = "Bit 28 - Port D bit 4 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien4(&mut self) -> PDIEN4_W<28> {
        PDIEN4_W::new(self)
    }
    #[doc = "Bit 29 - Port D bit 5 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien5(&mut self) -> PDIEN5_W<29> {
        PDIEN5_W::new(self)
    }
    #[doc = "Bit 30 - Port D bit 6 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien6(&mut self) -> PDIEN6_W<30> {
        PDIEN6_W::new(self)
    }
    #[doc = "Bit 31 - Port D bit 7 interrupt enable: 1: Enabled 2: Disabled"]
    #[inline(always)]
    pub fn pdien7(&mut self) -> PDIEN7_W<31> {
        PDIEN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The power-up interrupt enable register selects, for its corresponding port A-D pin, whether interrupts are enabled or disabled.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pi_ien](index.html) module"]
pub struct PI_IEN_SPEC;
impl crate::RegisterSpec for PI_IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pi_ien::R](R) reader structure"]
impl crate::Readable for PI_IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pi_ien::W](W) writer structure"]
impl crate::Writable for PI_IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PI_IEN to value 0"]
impl crate::Resettable for PI_IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
