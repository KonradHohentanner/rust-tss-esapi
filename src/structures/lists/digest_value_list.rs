// Copyright 2020 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use crate::structures::Digest;
//use crate::tss2_esys::TPML_DIGEST_VALUES;
//use crate::{Error, Result, WrapperErrorKind};
//use log::error;
//use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone, Default)]
pub struct DigestValueList {
    digests: Vec<Digest>,
}
/*
impl DigestValueList {
    // minimum is two for TPM2_PolicyOR().
    pub const MIN_SIZE: usize = 2;
    pub const MAX_SIZE: usize = 8;
    pub fn new() -> Self {
        DigestValueList {
            digests: Vec::new(),
        }
    }

    pub fn value(&self) -> &[Digest] {
        &self.digests
    }

    pub fn add(&mut self, dig: Digest) -> Result<()> {
        if self.digests.len() >= DigestValueList::MAX_SIZE {
            error!("Error: Exceeded maximum count(> {})", DigestValueList::MAX_SIZE);
            return Err(Error::local_error(WrapperErrorKind::WrongParamSize));
        }
        self.digests.push(dig);
        Ok(())
    }
}

impl TryFrom<TPML_DIGEST_VALUES> for DigestValueList {
    type Error = Error;
    fn try_from(tpml_digest: TPML_DIGEST_VALUES) -> Result<Self> {
        let digests_count = tpml_digest.count as usize;
        if digests_count < DigestValueList::MIN_SIZE {
            error!(
                "Error: Invalid TPML_DIGEST count(< {})",
                DigestValueList::MIN_SIZE
            );
            return Err(Error::local_error(WrapperErrorKind::InvalidParam));
        }
        if digests_count > DigestValueList::MAX_SIZE {
            error!(
                "Error: Invalid TPML_DIGEST count(> {})",
                DigestValueList::MAX_SIZE
            );
            return Err(Error::local_error(WrapperErrorKind::InvalidParam));
        }
        let digests = &tpml_digest.digests[..digests_count];
        let digests: Result<Vec<Digest>> = digests.iter().map(|x| Digest::try_from(*x)).collect();
        Ok(DigestValueList { digests: digests? })
    }
}

impl TryFrom<DigestValueList> for TPML_DIGEST_VALUES {
    type Error = Error;
    fn try_from(digest_list: DigestValueList) -> Result<Self> {
        if digest_list.digests.len() < DigestValueList::MIN_SIZE {
            error!(
                "Error: Invalid digest list size(< {})",
                DigestValueList::MIN_SIZE
            );
            return Err(Error::local_error(WrapperErrorKind::WrongParamSize));
        }
        if digest_list.digests.len() > DigestValueList::MAX_SIZE {
            error!(
                "Error: Invalid digest list size(> {})",
                DigestValueList::MAX_SIZE
            );
            return Err(Error::local_error(WrapperErrorKind::WrongParamSize));
        }

        let mut tss_digest_list: TPML_DIGEST_VALUES = Default::default();
        for digest in digest_list.digests.iter() {
            tss_digest_list.digests[tss_digest_list.count as usize] = digest.clone().try_into()?;
            tss_digest_list.count += 1;
        }
        Ok(tss_digest_list)
    }
}
*/