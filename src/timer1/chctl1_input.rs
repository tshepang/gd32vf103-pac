#[doc = r" Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::CHCTL1_INPUT {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits };
        let mut w = W { bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct CH3CAPFLTR {
    bits: u8,
}
impl CH3CAPFLTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH3CAPPSCR {
    bits: u8,
}
impl CH3CAPPSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH3MSR {
    bits: u8,
}
impl CH3MSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH2CAPFLTR {
    bits: u8,
}
impl CH2CAPFLTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH2CAPPSCR {
    bits: u8,
}
impl CH2CAPPSCR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CH2MSR {
    bits: u8,
}
impl CH2MSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CH3CAPFLTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3CAPFLTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH3CAPPSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3CAPPSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH3MSW<'a> {
    w: &'a mut W,
}
impl<'a> _CH3MSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH2CAPFLTW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2CAPFLTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x0f;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH2CAPPSCW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2CAPPSCW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CH2MSW<'a> {
    w: &'a mut W,
}
impl<'a> _CH2MSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 0x03;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u16) << OFFSET);
        self.w.bits |= ((value & MASK) as u16) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline]
    pub fn ch3capflt(&self) -> CH3CAPFLTR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH3CAPFLTR { bits }
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline]
    pub fn ch3cappsc(&self) -> CH3CAPPSCR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH3CAPPSCR { bits }
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline]
    pub fn ch3ms(&self) -> CH3MSR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH3MSR { bits }
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline]
    pub fn ch2capflt(&self) -> CH2CAPFLTR {
        let bits = {
            const MASK: u8 = 0x0f;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH2CAPFLTR { bits }
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline]
    pub fn ch2cappsc(&self) -> CH2CAPPSCR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH2CAPPSCR { bits }
    }
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline]
    pub fn ch2ms(&self) -> CH2MSR {
        let bits = {
            const MASK: u8 = 0x03;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u16) as u8
        };
        CH2MSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 12:15 - Channel 3 input capture filter control"]
    #[inline]
    pub fn ch3capflt(&mut self) -> _CH3CAPFLTW {
        _CH3CAPFLTW { w: self }
    }
    #[doc = "Bits 10:11 - Channel 3 input capture prescaler"]
    #[inline]
    pub fn ch3cappsc(&mut self) -> _CH3CAPPSCW {
        _CH3CAPPSCW { w: self }
    }
    #[doc = "Bits 8:9 - Channel 3 mode selection"]
    #[inline]
    pub fn ch3ms(&mut self) -> _CH3MSW {
        _CH3MSW { w: self }
    }
    #[doc = "Bits 4:7 - Channel 2 input capture filter control"]
    #[inline]
    pub fn ch2capflt(&mut self) -> _CH2CAPFLTW {
        _CH2CAPFLTW { w: self }
    }
    #[doc = "Bits 2:3 - Channel 2 input capture prescaler"]
    #[inline]
    pub fn ch2cappsc(&mut self) -> _CH2CAPPSCW {
        _CH2CAPPSCW { w: self }
    }
    #[doc = "Bits 0:1 - Channel 2 mode selection"]
    #[inline]
    pub fn ch2ms(&mut self) -> _CH2MSW {
        _CH2MSW { w: self }
    }
}
