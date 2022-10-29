use crate::utils::now_timestamp;
use base64;

const DEFAULT_EPOCH: u64 = 1_666_595_382;

#[derive(Clone, Debug)]
pub struct Snowflake {
    epoch: u64,
    seq: u32,
    last_sequence_exhaustion: u64,
}

#[derive(Clone, Debug)]
pub struct DecodedSnowflake {
    pub id: u64,
    pub timestamp: u64,
    pub seq: u64,
    pub epoch: u64,
}

#[derive(Clone, Debug)]
pub struct LemonId {
    _snowflake: Snowflake,
    prefix: String
}


impl LemonId {
    pub fn new(p: &str) -> Self {
        let snowflake = Snowflake::new(Some(DEFAULT_EPOCH));
        Self {
            _snowflake: snowflake,
            prefix: p.to_string(),
        }
    }

    pub fn gen(&mut self) -> String {
        let sf = self._snowflake.gen();
        let p = &self.prefix;
        format!("{}_{}", p, base64::encode(sf).replace("=", ""))
    }
}

impl Snowflake {
    pub fn new(epoch: Option<u64>) -> Snowflake {
        let e = epoch.or(Some(DEFAULT_EPOCH)).unwrap();
        Snowflake {
            epoch: e,
            seq: 0,
            last_sequence_exhaustion: 0,
        }
    }

    #[inline]
    pub fn gen(&mut self) -> String {
        self.gen_with_ts(now_timestamp())
    }

    pub fn gen_with_ts(&mut self, timestamp: u64) -> String {
        if self.seq >= 4095 && timestamp == self.last_sequence_exhaustion {
            while now_timestamp() - timestamp < 1 {
                continue;
            }
        }

        let sf = ((timestamp - self.epoch) << 22)
            | u64::from(self.seq);

        self.seq = if self.seq >= 4095 { 0 } else { self.seq + 1 };

        if self.seq == 4095 {
            self.last_sequence_exhaustion = timestamp;
        }

        sf.to_string()
    }

    pub fn decode(&self, sf: &str) -> DecodedSnowflake {
        let sf = sf.parse::<u64>().unwrap();
        let timestamp = (sf >> 22) + self.epoch;
        let seq = sf & 0b1111_1111_1111;

        DecodedSnowflake {
            id: sf,
            timestamp,
            seq,
            epoch: self.epoch,
        }
    }
}
