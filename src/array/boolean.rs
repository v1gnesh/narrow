//! Array with boolean values.

use super::Array;
use crate::{
    bitmap::{Bitmap, BitmapRef, BitmapRefMut, ValidityBitmap},
    buffer::{BufferRef, BufferRefMut, BufferType, VecBuffer},
    nullable::Nullable,
    validity::Validity,
    Length,
};

/// Array with boolean values.
///
/// Values are stored using single bits in a [Bitmap].
pub struct BooleanArray<const NULLABLE: bool = false, Buffer: BufferType = VecBuffer>(
    pub(crate) <Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer>,
)
where
    Bitmap<Buffer>: Validity<NULLABLE>;

impl<const NULLABLE: bool, Buffer: BufferType> Array for BooleanArray<NULLABLE, Buffer> where
    Bitmap<Buffer>: Validity<NULLABLE>
{
}

impl<const NULLABLE: bool, Buffer: BufferType> BufferRef<u8> for BooleanArray<NULLABLE, Buffer>
where
    Bitmap<Buffer>: Validity<NULLABLE>,
    <Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer>: BufferRef<u8>,
{
    type Buffer =
        <<Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer> as BufferRef<u8>>::Buffer;

    fn buffer_ref(&self) -> &Self::Buffer {
        self.0.buffer_ref()
    }
}

impl<const NULLABLE: bool, Buffer: BufferType> BufferRefMut<u8> for BooleanArray<NULLABLE, Buffer>
where
    Bitmap<Buffer>: Validity<NULLABLE>,
    <Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer>: BufferRefMut<u8>,
{
    type BufferMut =
        <<Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer> as BufferRefMut<u8>>::BufferMut;

    fn buffer_ref_mut(&mut self) -> &mut Self::BufferMut {
        self.0.buffer_ref_mut()
    }
}

impl<const NULLABLE: bool, Buffer: BufferType> Default for BooleanArray<NULLABLE, Buffer>
where
    Bitmap<Buffer>: Validity<NULLABLE>,
    <Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer>: Default,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<U, const NULLABLE: bool, Buffer: BufferType> Extend<U> for BooleanArray<NULLABLE, Buffer>
where
    Bitmap<Buffer>: Validity<NULLABLE>,
    <Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer>: Extend<U>,
{
    fn extend<I: IntoIterator<Item = U>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl<Buffer: BufferType> From<BooleanArray<false, Buffer>> for BooleanArray<true, Buffer>
where
    Bitmap<Buffer>: FromIterator<bool>,
{
    fn from(value: BooleanArray<false, Buffer>) -> Self {
        Self(Nullable::wrap(value.0))
    }
}

impl<const NULLABLE: bool, U, Buffer: BufferType> FromIterator<U> for BooleanArray<NULLABLE, Buffer>
where
    Bitmap<Buffer>: Validity<NULLABLE>,
    <Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer>: FromIterator<U>,
{
    fn from_iter<I: IntoIterator<Item = U>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a, const NULLABLE: bool, Buffer: BufferType> IntoIterator
    for &'a BooleanArray<NULLABLE, Buffer>
where
    Bitmap<Buffer>: Validity<NULLABLE>,
    &'a <Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer>: IntoIterator,
{
    type Item = <&'a <Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer> as IntoIterator>::Item;
    type IntoIter =
        <&'a <Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const NULLABLE: bool, Buffer: BufferType> IntoIterator for BooleanArray<NULLABLE, Buffer>
where
    Bitmap<Buffer>: Validity<NULLABLE>,
    <Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer>: IntoIterator,
{
    type Item = <<Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer> as IntoIterator>::Item;
    type IntoIter =
        <<Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<const NULLABLE: bool, Buffer: BufferType> Length for BooleanArray<NULLABLE, Buffer>
where
    Bitmap<Buffer>: Validity<NULLABLE>,
    <Bitmap<Buffer> as Validity<NULLABLE>>::Storage<Buffer>: Length,
{
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<Buffer: BufferType> BitmapRef for BooleanArray<true, Buffer> {
    type Buffer = Buffer;

    fn bitmap_ref(&self) -> &Bitmap<Self::Buffer> {
        self.0.bitmap_ref()
    }
}

impl<Buffer: BufferType> BitmapRefMut for BooleanArray<true, Buffer> {
    fn bitmap_ref_mut(&mut self) -> &mut Bitmap<Self::Buffer> {
        self.0.bitmap_ref_mut()
    }
}

impl<Buffer: BufferType> ValidityBitmap for BooleanArray<true, Buffer> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bitmap::{BitmapRef, ValidityBitmap},
        buffer::{BoxBuffer, BufferRefMut},
    };
    use std::mem;

    #[test]
    fn from_iter() {
        let mut array = [true, false, true, true]
            .into_iter()
            .collect::<BooleanArray<false, BoxBuffer>>();
        assert_eq!(array.len(), 4);
        assert_eq!(array.buffer_ref().as_ref(), [0b00001101]);
        array.buffer_ref_mut()[0] = 0xff;
        assert_eq!(array.buffer_ref().as_ref(), [0b11111111]);

        let array = [Some(true), None, Some(true), Some(false)]
            .into_iter()
            .collect::<BooleanArray<true>>();
        assert_eq!(array.len(), 4);
        assert_eq!(array.0.data.is_valid(0), Some(true));
        assert_eq!(array.0.data.is_null(1), Some(true));
        assert_eq!(array.0.data.is_valid(2), Some(true));
        assert_eq!(array.0.data.is_valid(3), Some(false));
        assert_eq!(array.is_valid(0), Some(true));
        assert_eq!(array.is_null(1), Some(true));
        assert_eq!(array.is_valid(2), Some(true));
        assert_eq!(array.is_valid(3), Some(true));
        assert!(array.bitmap_ref()[0]);
        assert!(!array.bitmap_ref()[1]);
        assert!(array.bitmap_ref()[2]);
        assert!(array.bitmap_ref()[3]);
        assert!(array.0.data.is_valid(4).is_none());
        assert_eq!(array.0.data.bitmap_ref().len(), array.len());
    }

    #[test]
    fn into_iter() {
        let input = [true, false, true, true];
        let array = input.iter().collect::<BooleanArray>();
        let output = (&array).into_iter().collect::<Vec<_>>();
        assert_eq!(input, output.as_slice());

        let output = array.into_iter().collect::<Vec<_>>();
        assert_eq!(input, output.as_slice());

        let input = [Some(true), None, Some(true), Some(false)];
        let array = input.into_iter().collect::<BooleanArray<true>>();
        let output = (&array).into_iter().collect::<Vec<_>>();
        assert_eq!(input, output.as_slice());

        let output = array.into_iter().collect::<Vec<_>>();
        assert_eq!(input, output.as_slice());
    }

    #[test]
    fn buffer_ref_mut() {
        let input = [false, false, false, false];
        let mut array = input.iter().collect::<BooleanArray>();
        array.0.buffer_ref_mut()[0] = 0b0000_1111;
        assert_eq!(
            array.into_iter().collect::<Vec<_>>(),
            [true, true, true, true]
        );
    }

    #[test]
    fn convert_nullable() {
        let input = [true, false];
        let array = input.into_iter().collect::<BooleanArray>();
        let nullable: BooleanArray<true> = array.into();
        assert!(nullable.all_valid());
        assert_eq!(
            nullable.into_iter().collect::<Vec<_>>(),
            [Some(true), Some(false)]
        );
    }

    #[test]
    fn size_of() {
        assert_eq!(mem::size_of::<BooleanArray>(), mem::size_of::<Bitmap>());
        assert_eq!(
            mem::size_of::<BooleanArray<true>>(),
            mem::size_of::<BooleanArray>() + mem::size_of::<Bitmap>()
        );
    }
}
