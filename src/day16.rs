use crate::{day, EyreResult};

day! {
    parser,
    part1 => "Sum of versions is {}",
    part2 => "Value is {}",
}

#[derive(Debug)]
pub(crate) struct Packet {
    version: u8,
    ty: u8,
    payload: Payload,
}

impl Packet {
    fn version_sum(&self) -> u64 {
        match &self.payload {
            Payload::Litteral(_) => self.version as u64,
            Payload::Operator(v) => {
                self.version as u64 + v.iter().map(Self::version_sum).sum::<u64>()
            }
        }
    }

    fn calculate(&self) -> u64 {
        match &self.payload {
            Payload::Litteral(v) => *v,
            Payload::Operator(v) => {
                let mut values = v.iter().map(Self::calculate);
                match self.ty {
                    0 => values.sum(),
                    1 => values.product(),
                    2 => values.min().unwrap(),
                    3 => values.max().unwrap(),
                    5 | 6 | 7 => {
                        let a = values.next().unwrap();
                        let b = values.next().unwrap();
                        (match self.ty {
                            5 => a > b,
                            6 => a < b,
                            7 => a == b,
                            _ => unreachable!(),
                        }) as u64
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum Payload {
    Litteral(u64),
    Operator(Vec<Packet>),
}

type Parsed = Packet;

fn bits(v: u8) -> impl Iterator<Item = Bit> {
    (0..8)
        .rev()
        .map(move |i| Bit::Set(((1 << i) & v) != 0))
        .chain(std::iter::once(Bit::Boundry))
}

fn hex_digit(v: u8) -> u8 {
    if v < b'A' {
        v - b'0'
    } else {
        v - b'A' + 10
    }
}

fn hex_slice(s: &[u8]) -> u8 {
    (hex_digit(s[0]) << 4) | hex_digit(s[1])
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Bit {
    Set(bool),
    Boundry,
}

impl Bit {
    fn to_bit(self) -> u8 {
        match self {
            Bit::Set(b) => b as u8,
            Bit::Boundry => panic!(),
        }
    }
}

fn bit_stream(input: &str) -> impl Iterator<Item = Bit> + '_ {
    input
        .trim()
        .as_bytes()
        .chunks_exact(2)
        .map(|s| bits(hex_slice(s)))
        .flatten()
}

fn is_not_boundry(&b: &Bit) -> bool {
    b != Bit::Boundry
}

fn num<I>(bit_count: usize, bits: &mut I) -> u64
where
    I: Iterator<Item = Bit>,
{
    bits.filter(is_not_boundry)
        .take(bit_count)
        .map(Bit::to_bit)
        .fold(0, |current, digit| current << 1 | digit as u64)
}

fn take_bool<I>(bits: &mut I) -> bool
where
    I: Iterator<Item = Bit>,
{
    bits.filter(is_not_boundry).next().unwrap().to_bit() == 1
}

fn parse_group<I>(bits: &mut I) -> ((bool, u8), usize)
where
    I: Iterator<Item = Bit>,
{
    let last = !take_bool(bits);
    ((last, num(4, bits) as u8), 1 + 4)
}

fn parse_literal<I>(bits: &mut I) -> (u64, usize)
where
    I: Iterator<Item = Bit>,
{
    let mut val = 0;
    let mut read = 0;
    loop {
        let ((last, v), r) = parse_group(bits);
        read += r;
        val = val << 4 | v as u64;
        if last {
            break (val, read);
        }
    }
}

fn parse_operator<I>(bits: &mut I) -> (Vec<Packet>, usize)
where
    I: Iterator<Item = Bit>,
{
    let is_packet_count = take_bool(bits);

    let mut read;
    let count = if is_packet_count {
        read = 1 + 11;
        num(11, bits)
    } else {
        read = 1 + 15;
        num(15, bits)
    };

    if is_packet_count {
        let mut sub_packets = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let (packet, r) = parse_packet(bits);
            read += r;
            sub_packets.push(packet);
        }

        (sub_packets, read)
    } else {
        let mut remain = count;

        let mut sub_packets = Vec::new();
        while remain != 0 {
            let (packet, r) = parse_packet(bits);
            remain -= r as u64;
            read += r;
            sub_packets.push(packet);
        }
        (sub_packets, read)
    }
}

fn parse_packet<I>(bits: &mut I) -> (Packet, usize)
where
    I: Iterator<Item = Bit>,
{
    let version = num(3, bits) as u8;
    let id = num(3, bits);
    if id == 4 {
        let (payload, read) = parse_literal(bits);
        (
            Packet {
                version,
                ty: id as u8,
                payload: Payload::Litteral(payload),
            },
            read + 6,
        )
    } else {
        let (payload, read) = parse_operator(bits);
        (
            Packet {
                version,
                ty: id as u8,
                payload: Payload::Operator(payload),
            },
            read + 6,
        )
    }
}

pub(crate) fn parser(input: &str) -> EyreResult<Parsed> {
    let mut stream = bit_stream(input);
    Ok(parse_packet(&mut stream).0)
}

pub(crate) fn part1(packet: Parsed) -> EyreResult<u64> {
    Ok(packet.version_sum())
}

pub(crate) fn part2(packet: Parsed) -> EyreResult<u64> {
    Ok(packet.calculate())
}

#[cfg(test)]
mod test {

    #[test]
    fn bits() {
        use super::bits;
        use super::Bit::*;

        assert_eq!(
            Vec::from_iter(bits(0b00110101)),
            vec![
                Set(false),
                Set(false),
                Set(true),
                Set(true),
                Set(false),
                Set(true),
                Set(false),
                Set(true),
                Boundry
            ]
        )
    }

    #[test]
    fn hex_slice() {
        use super::hex_slice;

        assert_eq!(hex_slice(&[b'D', b'2']), 0b11010010)
    }

    #[test]
    fn hex() {
        use super::bits;
        use super::Bit::*;

        assert_eq!(
            Vec::from_iter(bits(0b00110101)),
            vec![
                Set(false),
                Set(false),
                Set(true),
                Set(true),
                Set(false),
                Set(true),
                Set(false),
                Set(true),
                Boundry
            ]
        )
    }

    #[test]
    fn input() {
        use super::{bit_stream, Bit::*};

        assert_eq!(
            Vec::from_iter(bit_stream("D2FE28").filter(|&p| p != Boundry)),
            Vec::from_iter(
                "110100101111111000101000"
                    .as_bytes()
                    .iter()
                    .map(|&b| Set(b == b'1'))
            )
        )
    }
}
