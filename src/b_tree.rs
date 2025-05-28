pub const BTREE_PAGE_SIZE: usize = 4096;
const BTREE_MAX_KEY_SIZE: u64 = 1000;
const BTREE_MAX_VAL_SIZE: u64 = 3000;
const BTREE_MAX_NODE_SIZE: u64 = 4 + 1 * 8 + 1 * 2 + 4 + BTREE_MAX_KEY_SIZE + BTREE_MAX_VAL_SIZE;

pub mod node {
    use std::{fmt::{self, Debug, Formatter}, ops::Range};

    use super::BTREE_PAGE_SIZE;

    pub struct BNode<'a> {
        data: &'a mut [u8; BTREE_PAGE_SIZE],
    }

    impl<'a> Debug for BNode<'a> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            f.debug_struct("BNode")
            .field("data", &&self.data[..]) // `&&` converts `&mut [u8; N]` to `&&[u8]`
            .finish()
        }
    }

    impl<'a> BNode<'a> {
        pub fn new(data: &'a mut [u8; BTREE_PAGE_SIZE]) -> Self {
            BNode { data }
        }

        fn btype(&self) -> u16 {
            u16::from_le_bytes([self.data[0], self.data[1]])
        }

        fn nkeys(&self) -> u16 {
            u16::from_le_bytes([self.data[2], self.data[3]])
        }

        pub fn set_header(self, btype: u16, nkeys: u16) {
            let btype = btype.to_le_bytes();
            let nkeys = nkeys.to_le_bytes();

            self.data[0..2].copy_from_slice(&btype);
            self.data[2..4].copy_from_slice(&nkeys);
        }

        pub fn get_ptr(self, idx: u16) -> u64 {
            let pos: usize = 4 + 8 * (idx as usize); // header is 4 bytes
            let bytes = [
                self.data[pos],
                self.data[pos + 1],
                self.data[pos + 2],
                self.data[pos + 3],
                self.data[pos + 4],
                self.data[pos + 5],
                self.data[pos + 6],
                self.data[pos + 7],
            ];
            u64::from_le_bytes(bytes)
        }

        pub fn set_ptr(self, idx: u16, val: u64) {
            assert!(&self.nkeys().ge(&idx));
            let pos = 4 + 8 * (idx as usize);
            let bytes = val.to_le_bytes();
            self.data[pos..pos + 8].copy_from_slice(&bytes);
        }

        pub fn get_offset(&self, idx: u16) -> u16 {
            if idx == 0 {
                return 0;
            };

            let pos = (4 + 8 * self.nkeys() + 2 * (idx - 1)) as usize;
            u16::from_le_bytes([self.data[pos], self.data[pos + 1]])
        }

        pub fn kv_pos(&self, idx: u16) -> u16 {
            assert!(idx < self.nkeys());

            4 + 8 * self.nkeys() + 2 * self.nkeys() + self.get_offset(idx)
        }

        pub fn get_key(self, idx: u16) -> &'a [u8] {
            assert!(idx < self.nkeys());

            let pos = self.kv_pos(idx) as usize;
            let key_len = u16::from_le_bytes([self.data[pos], self.data[pos + 1]]) as usize;
            let val_len = u16::from_le_bytes([self.data[pos + 2], self.data[pos + 3]]) as usize;

            let start = pos + 4 + key_len;
            let end = pos + val_len;

            &self.data[start..end]
        }
    }

    fn slice_as_num(slice: &[u8], index: Range<usize>) -> Option<u16> {
        slice
            .get(index)
            .and_then(|b| b.try_into().ok())
            .map(u16::from_le_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_node_size_fits_in_page() {
        assert!(BTREE_MAX_NODE_SIZE <= BTREE_PAGE_SIZE as u64);
    }
}
