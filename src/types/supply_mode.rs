// Generated by Molecule 0.7.3

/////// manually added lines start ///////
#![allow(unused_imports)]
use ckb_types::molecule;
use ckb_types::packed::*;
use ckb_types::prelude::*;
/////// manually added lines end ///////
#[derive(Clone)]
pub struct InfoCell(molecule::bytes::Bytes);
impl ::core::fmt::LowerHex for InfoCell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl ::core::fmt::Debug for InfoCell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl ::core::fmt::Display for InfoCell {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "version", self.version())?;
        write!(f, ", {}: {}", "current_supply", self.current_supply())?;
        write!(f, ", {}: {}", "max_supply", self.max_supply())?;
        write!(f, ", {}: {}", "sudt_script_hash", self.sudt_script_hash())?;
        write!(f, ", {}: {}", "other_data", self.other_data())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl ::core::default::Default for InfoCell {
    fn default() -> Self {
        let v: Vec<u8> = vec![
            89, 0, 0, 0, 24, 0, 0, 0, 25, 0, 0, 0, 41, 0, 0, 0, 57, 0, 0, 0, 89, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0,
        ];
        InfoCell::new_unchecked(v.into())
    }
}
impl InfoCell {
    pub const FIELD_COUNT: usize = 5;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn version(&self) -> Byte {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        Byte::new_unchecked(self.0.slice(start..end))
    }
    pub fn current_supply(&self) -> Uint128 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Uint128::new_unchecked(self.0.slice(start..end))
    }
    pub fn max_supply(&self) -> Uint128 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        Uint128::new_unchecked(self.0.slice(start..end))
    }
    pub fn sudt_script_hash(&self) -> Uint256 {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        let end = molecule::unpack_number(&slice[20..]) as usize;
        Uint256::new_unchecked(self.0.slice(start..end))
    }
    pub fn other_data(&self) -> BytesOpt {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[20..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[24..]) as usize;
            BytesOpt::new_unchecked(self.0.slice(start..end))
        } else {
            BytesOpt::new_unchecked(self.0.slice(start..))
        }
    }
    pub fn as_reader<'r>(&'r self) -> InfoCellReader<'r> {
        InfoCellReader::new_unchecked(self.as_slice())
    }
}
impl molecule::prelude::Entity for InfoCell {
    type Builder = InfoCellBuilder;
    const NAME: &'static str = "InfoCell";
    fn new_unchecked(data: molecule::bytes::Bytes) -> Self {
        InfoCell(data)
    }
    fn as_bytes(&self) -> molecule::bytes::Bytes {
        self.0.clone()
    }
    fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }
    fn from_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        InfoCellReader::from_slice(slice).map(|reader| reader.to_entity())
    }
    fn from_compatible_slice(slice: &[u8]) -> molecule::error::VerificationResult<Self> {
        InfoCellReader::from_compatible_slice(slice).map(|reader| reader.to_entity())
    }
    fn new_builder() -> Self::Builder {
        ::core::default::Default::default()
    }
    fn as_builder(self) -> Self::Builder {
        Self::new_builder()
            .version(self.version())
            .current_supply(self.current_supply())
            .max_supply(self.max_supply())
            .sudt_script_hash(self.sudt_script_hash())
            .other_data(self.other_data())
    }
}
#[derive(Clone, Copy)]
pub struct InfoCellReader<'r>(&'r [u8]);
impl<'r> ::core::fmt::LowerHex for InfoCellReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        use molecule::hex_string;
        if f.alternate() {
            write!(f, "0x")?;
        }
        write!(f, "{}", hex_string(self.as_slice()))
    }
}
impl<'r> ::core::fmt::Debug for InfoCellReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{}({:#x})", Self::NAME, self)
    }
}
impl<'r> ::core::fmt::Display for InfoCellReader<'r> {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(f, "{} {{ ", Self::NAME)?;
        write!(f, "{}: {}", "version", self.version())?;
        write!(f, ", {}: {}", "current_supply", self.current_supply())?;
        write!(f, ", {}: {}", "max_supply", self.max_supply())?;
        write!(f, ", {}: {}", "sudt_script_hash", self.sudt_script_hash())?;
        write!(f, ", {}: {}", "other_data", self.other_data())?;
        let extra_count = self.count_extra_fields();
        if extra_count != 0 {
            write!(f, ", .. ({} fields)", extra_count)?;
        }
        write!(f, " }}")
    }
}
impl<'r> InfoCellReader<'r> {
    pub const FIELD_COUNT: usize = 5;
    pub fn total_size(&self) -> usize {
        molecule::unpack_number(self.as_slice()) as usize
    }
    pub fn field_count(&self) -> usize {
        if self.total_size() == molecule::NUMBER_SIZE {
            0
        } else {
            (molecule::unpack_number(&self.as_slice()[molecule::NUMBER_SIZE..]) as usize / 4) - 1
        }
    }
    pub fn count_extra_fields(&self) -> usize {
        self.field_count() - Self::FIELD_COUNT
    }
    pub fn has_extra_fields(&self) -> bool {
        Self::FIELD_COUNT != self.field_count()
    }
    pub fn version(&self) -> ByteReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[4..]) as usize;
        let end = molecule::unpack_number(&slice[8..]) as usize;
        ByteReader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn current_supply(&self) -> Uint128Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[8..]) as usize;
        let end = molecule::unpack_number(&slice[12..]) as usize;
        Uint128Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn max_supply(&self) -> Uint128Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[12..]) as usize;
        let end = molecule::unpack_number(&slice[16..]) as usize;
        Uint128Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn sudt_script_hash(&self) -> Uint256Reader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[16..]) as usize;
        let end = molecule::unpack_number(&slice[20..]) as usize;
        Uint256Reader::new_unchecked(&self.as_slice()[start..end])
    }
    pub fn other_data(&self) -> BytesOptReader<'r> {
        let slice = self.as_slice();
        let start = molecule::unpack_number(&slice[20..]) as usize;
        if self.has_extra_fields() {
            let end = molecule::unpack_number(&slice[24..]) as usize;
            BytesOptReader::new_unchecked(&self.as_slice()[start..end])
        } else {
            BytesOptReader::new_unchecked(&self.as_slice()[start..])
        }
    }
}
impl<'r> molecule::prelude::Reader<'r> for InfoCellReader<'r> {
    type Entity = InfoCell;
    const NAME: &'static str = "InfoCellReader";
    fn to_entity(&self) -> Self::Entity {
        Self::Entity::new_unchecked(self.as_slice().to_owned().into())
    }
    fn new_unchecked(slice: &'r [u8]) -> Self {
        InfoCellReader(slice)
    }
    fn as_slice(&self) -> &'r [u8] {
        self.0
    }
    fn verify(slice: &[u8], compatible: bool) -> molecule::error::VerificationResult<()> {
        use molecule::verification_error as ve;
        let slice_len = slice.len();
        if slice_len < molecule::NUMBER_SIZE {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE, slice_len);
        }
        let total_size = molecule::unpack_number(slice) as usize;
        if slice_len != total_size {
            return ve!(Self, TotalSizeNotMatch, total_size, slice_len);
        }
        if slice_len == molecule::NUMBER_SIZE && Self::FIELD_COUNT == 0 {
            return Ok(());
        }
        if slice_len < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, HeaderIsBroken, molecule::NUMBER_SIZE * 2, slice_len);
        }
        let offset_first = molecule::unpack_number(&slice[molecule::NUMBER_SIZE..]) as usize;
        if offset_first % molecule::NUMBER_SIZE != 0 || offset_first < molecule::NUMBER_SIZE * 2 {
            return ve!(Self, OffsetsNotMatch);
        }
        if slice_len < offset_first {
            return ve!(Self, HeaderIsBroken, offset_first, slice_len);
        }
        let field_count = offset_first / molecule::NUMBER_SIZE - 1;
        if field_count < Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        } else if !compatible && field_count > Self::FIELD_COUNT {
            return ve!(Self, FieldCountNotMatch, Self::FIELD_COUNT, field_count);
        };
        let mut offsets: Vec<usize> = slice[molecule::NUMBER_SIZE..offset_first]
            .chunks_exact(molecule::NUMBER_SIZE)
            .map(|x| molecule::unpack_number(x) as usize)
            .collect();
        offsets.push(total_size);
        if offsets.windows(2).any(|i| i[0] > i[1]) {
            return ve!(Self, OffsetsNotMatch);
        }
        ByteReader::verify(&slice[offsets[0]..offsets[1]], compatible)?;
        Uint128Reader::verify(&slice[offsets[1]..offsets[2]], compatible)?;
        Uint128Reader::verify(&slice[offsets[2]..offsets[3]], compatible)?;
        Uint256Reader::verify(&slice[offsets[3]..offsets[4]], compatible)?;
        BytesOptReader::verify(&slice[offsets[4]..offsets[5]], compatible)?;
        Ok(())
    }
}
#[derive(Debug, Default)]
pub struct InfoCellBuilder {
    pub(crate) version: Byte,
    pub(crate) current_supply: Uint128,
    pub(crate) max_supply: Uint128,
    pub(crate) sudt_script_hash: Uint256,
    pub(crate) other_data: BytesOpt,
}
impl InfoCellBuilder {
    pub const FIELD_COUNT: usize = 5;
    pub fn version(mut self, v: Byte) -> Self {
        self.version = v;
        self
    }
    pub fn current_supply(mut self, v: Uint128) -> Self {
        self.current_supply = v;
        self
    }
    pub fn max_supply(mut self, v: Uint128) -> Self {
        self.max_supply = v;
        self
    }
    pub fn sudt_script_hash(mut self, v: Uint256) -> Self {
        self.sudt_script_hash = v;
        self
    }
    pub fn other_data(mut self, v: BytesOpt) -> Self {
        self.other_data = v;
        self
    }
}
impl molecule::prelude::Builder for InfoCellBuilder {
    type Entity = InfoCell;
    const NAME: &'static str = "InfoCellBuilder";
    fn expected_length(&self) -> usize {
        molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1)
            + self.version.as_slice().len()
            + self.current_supply.as_slice().len()
            + self.max_supply.as_slice().len()
            + self.sudt_script_hash.as_slice().len()
            + self.other_data.as_slice().len()
    }
    fn write<W: molecule::io::Write>(&self, writer: &mut W) -> molecule::io::Result<()> {
        let mut total_size = molecule::NUMBER_SIZE * (Self::FIELD_COUNT + 1);
        let mut offsets = Vec::with_capacity(Self::FIELD_COUNT);
        offsets.push(total_size);
        total_size += self.version.as_slice().len();
        offsets.push(total_size);
        total_size += self.current_supply.as_slice().len();
        offsets.push(total_size);
        total_size += self.max_supply.as_slice().len();
        offsets.push(total_size);
        total_size += self.sudt_script_hash.as_slice().len();
        offsets.push(total_size);
        total_size += self.other_data.as_slice().len();
        writer.write_all(&molecule::pack_number(total_size as molecule::Number))?;
        for offset in offsets.into_iter() {
            writer.write_all(&molecule::pack_number(offset as molecule::Number))?;
        }
        writer.write_all(self.version.as_slice())?;
        writer.write_all(self.current_supply.as_slice())?;
        writer.write_all(self.max_supply.as_slice())?;
        writer.write_all(self.sudt_script_hash.as_slice())?;
        writer.write_all(self.other_data.as_slice())?;
        Ok(())
    }
    fn build(&self) -> Self::Entity {
        let mut inner = Vec::with_capacity(self.expected_length());
        self.write(&mut inner)
            .unwrap_or_else(|_| panic!("{} build should be ok", Self::NAME));
        InfoCell::new_unchecked(inner.into())
    }
}
