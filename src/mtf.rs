use std;
use super::aux::UncheckedSliceExt;

const MTF_VALUE_ARRAY: [u16; 257] = include!(
    concat!(env!("OUT_DIR"), "/", "MTF_VALUE_ARRAY.txt"));
const MTF_INDEX_ARRAY: [u16; 257] = include!(
    concat!(env!("OUT_DIR"), "/", "MTF_INDEX_ARRAY.txt"));
const MTF_NEXT_ARRAY:  [u16; 257] = include!(
    concat!(env!("OUT_DIR"), "/", "MTF_NEXT_ARRAY.txt"));

pub struct MTFCoder {
    value_array: [u16; 257],
    index_array: [u16; 257],
}

impl MTFCoder {
    pub fn new() -> MTFCoder {
        return MTFCoder {
            value_array: MTF_VALUE_ARRAY,
            index_array: MTF_INDEX_ARRAY,
        };
    }

    pub fn encode(&mut self, value: u16, value_unlikely: u16) -> u16 {
        unsafe {
            let index = self.index_array.nocheck()[value as usize];
            let index_unlikely = self.index_array.nocheck()[value_unlikely as usize];

            let next_index = MTF_NEXT_ARRAY.nocheck()[index as usize];
            let next_value = self.value_array.nocheck()[next_index as usize];
            std::ptr::swap(
                self.index_array.get_unchecked_mut(value as usize),
                self.index_array.get_unchecked_mut(next_value as usize));
            std::ptr::swap(
                self.value_array.get_unchecked_mut(index as usize),
                self.value_array.get_unchecked_mut(next_index as usize));

            return match index.cmp(&index_unlikely) {
                std::cmp::Ordering::Equal   => 256,
                std::cmp::Ordering::Less    => index,
                std::cmp::Ordering::Greater => index - 1,
            };
        }
    }

    pub fn decode(&mut self, index: u16, value_unlikely: u16) -> u16 {
        unsafe {
            let index_unlikely = self.index_array.nocheck()[value_unlikely as usize];
            let index = match index {
                256                              => index_unlikely,
                _ if index + 1 <= index_unlikely => index,
                _ if index + 1 >  index_unlikely => index + 1,
                _ => unreachable!(),
            };

            let value = self.value_array.nocheck()[index as usize];
            let next_index = MTF_NEXT_ARRAY.nocheck()[index as usize];
            let next_value = self.value_array.nocheck()[next_index as usize];
            std::ptr::swap(
                self.index_array.get_unchecked_mut(value as usize),
                self.index_array.get_unchecked_mut(next_value as usize));
            std::ptr::swap(
                self.value_array.get_unchecked_mut(index as usize),
                self.value_array.get_unchecked_mut(next_index as usize));
            return value;
        }
    }
}
